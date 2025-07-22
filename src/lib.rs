//! A [`Decimal`] with pretty printing.
//!
//! Library provides comma separated decimal.
//!
//! ```
//! # use rust_decimal_macros::dec;
//! # use pretty_decimal::PrettyDecimal;
//! let x = PrettyDecimal::comma3dot(dec!(-1234567.890));
//! assert_eq!(x.to_string(), "-1,234,567.890");
//! ```
//!
//! ```
//! # use rust_decimal_macros::dec;
//! # use pretty_decimal::PrettyDecimal;
//! let x: PrettyDecimal = "-1,234,567.890".parse().unwrap();
//! assert_eq!(x.value, dec!(-1234567.890));
//! ```

use std::{fmt::Display, ops::Neg, str::FromStr};

use rust_decimal::Decimal;

#[cfg(feature = "bounded-static")]
use bounded_static::{IntoBoundedStatic, ToBoundedStatic, ToStatic};

/// Decimal formatting type for pretty-printing.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "bounded-static", derive(ToStatic))]
#[non_exhaustive]
pub enum Format {
    /// Decimal without no formatting, such as
    /// `1234` or `1234.5`.
    Plain,
    /// Use `,` on every thousands, `.` for the decimal point.
    Comma3Dot,
}

/// Decimal with the pretty printing format information.
#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive] // Don't want to construct directly.
pub struct PrettyDecimal {
    /// Format of the decimal, None means there's no associated format.
    /// That makes difference on from_str, which concludes `Some(Plain)` if the given input is >= 1,000,
    /// while it'll leave `None` format on 999 or below.
    pub format: Option<Format>,
    /// Actual value of the Decimal.
    pub value: Decimal,
}

impl Neg for PrettyDecimal {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.set_sign_positive(!self.value.is_sign_positive());
        self
    }
}

#[cfg(feature = "bounded-static")]
impl ToBoundedStatic for PrettyDecimal {
    type Static = Self;

    fn to_static(&self) -> <Self as ToBoundedStatic>::Static {
        self.clone()
    }
}

#[cfg(feature = "bounded-static")]
impl IntoBoundedStatic for PrettyDecimal {
    type Static = Self;

    fn into_static(self) -> <Self as IntoBoundedStatic>::Static {
        self
    }
}

/// Error occured during parsing.
#[derive(thiserror::Error, PartialEq, Debug)]
pub enum ParseError {
    #[error("unexpected char {0} at {1}")]
    UnexpectedChar(String, usize),
    #[error("comma required at {0}")]
    CommaRequired(usize),
    #[error("unexpressible decimal {0}")]
    InvalidDecimal(#[from] rust_decimal::Error),
}

impl PrettyDecimal {
    /// Constructs unformatted `PrettyDecimal`.
    #[inline]
    pub const fn unformatted(value: Decimal) -> Self {
        Self::with_format(value, None)
    }

    /// Constructs plain `PrettyDecimal`.
    #[inline]
    pub const fn plain(value: Decimal) -> Self {
        Self::with_format(value, Some(Format::Plain))
    }

    /// Constructs comma3 PrettyDecimal.
    #[inline]
    pub const fn comma3dot(value: Decimal) -> Self {
        Self::with_format(value, Some(Format::Comma3Dot))
    }

    /// Constructs an instance with the given format.
    #[inline]
    pub const fn with_format(value: Decimal, format: Option<Format>) -> Self {
        Self { format, value }
    }

    /// Returns the reference to the underlying decimal value.
    // Note: Decimal is Copy, so no penalty to use reference.
    #[inline]
    pub const fn as_decimal(&self) -> &Decimal {
        &self.value
    }

    /// Returns the current scale.
    pub const fn scale(&self) -> u32 {
        self.value.scale()
    }

    /// Rescale the underlying value.
    pub fn rescale(&mut self, scale: u32) {
        self.value.rescale(scale)
    }

    /// Sets the sign positive.
    pub fn set_sign_positive(&mut self, positive: bool) {
        self.value.set_sign_positive(positive)
    }

    /// Returns `true` if the value is positive.
    pub const fn is_sign_positive(&self) -> bool {
        self.value.is_sign_positive()
    }
}

/// Implementation of the Display trait.
#[inline]
fn display_comma_3_dot_impl<T: std::fmt::Write>(
    mut w: T,
    mut value: Decimal,
    precision: Option<usize>,
) -> std::fmt::Result {
    let mut buf = itoa::Buffer::new();
    if let Some(precision) = precision {
        cold();
        value = value.round_dp(precision as u32);
    }
    // Here we assume mantissa is all ASCII (given it's [0-9]+),
    // so use unsafe method to avoid UTF-8 validation.
    let mantissa = buf.format(value.mantissa()).as_bytes();
    let scale: usize = value.scale() as usize;
    let mut remainder = mantissa;
    if value.is_sign_negative() {
        w.write_str("-")?;
        remainder = &mantissa[1..];
    }
    let mut initial_integer = true;
    // caluclate the first comma position out of the integral portion digits.
    let mut comma_pos = (remainder.len() - scale) % 3;
    if comma_pos == 0 {
        comma_pos = 3;
    }
    while remainder.len() > scale {
        if !initial_integer {
            w.write_str(",")?;
        }
        let section;
        (section, remainder) = unsafe { remainder.split_at_unchecked(comma_pos) };
        w.write_str(unsafe { std::str::from_utf8_unchecked(section) })?;
        comma_pos = 3;
        initial_integer = false;
    }
    if !remainder.is_empty() {
        w.write_str(".")?;
        w.write_str(unsafe { std::str::from_utf8_unchecked(remainder) })?;
    }
    if let Some(precision) = precision {
        cold();
        if precision > remainder.len() {
            for _i in 0..(precision - remainder.len()) {
                w.write_str("0")?;
            }
        }
    }
    Ok(())
}

impl From<PrettyDecimal> for Decimal {
    #[inline]
    fn from(value: PrettyDecimal) -> Self {
        *value.as_decimal()
    }
}

impl AsRef<Decimal> for PrettyDecimal {
    fn as_ref(&self) -> &Decimal {
        self.as_decimal()
    }
}

impl AsMut<Decimal> for PrettyDecimal {
    fn as_mut(&mut self) -> &mut Decimal {
        &mut self.value
    }
}

impl FromStr for PrettyDecimal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only ASCII chars supported, use bytes.
        let mut comma_pos = None;
        let mut format = None;
        let mut mantissa: i128 = 0;
        let mut scale: Option<u32> = None;
        let mut prefix_len = 0;
        let mut sign = 1;
        let aligned_comma = |offset, cp, pos| match (cp, pos) {
            // first comma may appear in the beginning 4 [0..=3] characters,
            // otherwise there should be a comma before already.
            (None, _) if pos > offset && pos <= 3 + offset => true,
            _ if cp == Some(pos) => true,
            _ => false,
        };
        for (i, c) in s.bytes().enumerate() {
            match (comma_pos, i, c) {
                (_, 0, b'-') => {
                    prefix_len = 1;
                    sign = -1;
                }
                (_, 0, b'+') => {
                    prefix_len = 1;
                }
                (_, _, b',') if aligned_comma(prefix_len, comma_pos, i) => {
                    format = Some(Format::Comma3Dot);
                    comma_pos = Some(i + 4);
                }
                (_, _, b'.') if comma_pos.is_none() || comma_pos == Some(i) => {
                    scale = Some(0);
                    comma_pos = None;
                }
                (Some(cp), _, _) if cp == i => {
                    return Err(ParseError::CommaRequired(i));
                }
                _ if c.is_ascii_digit() => {
                    if scale.is_none() && format.is_none() && i >= 3 + prefix_len {
                        format = Some(Format::Plain);
                    }
                    mantissa = mantissa * 10 + (c as u32 - '0' as u32) as i128;
                    scale = scale.map(|x| x + 1);
                }
                _ => {
                    return Err(ParseError::UnexpectedChar(try_find_char(s, i, c), i));
                }
            }
        }
        let value = Decimal::try_from_i128_with_scale(sign * mantissa, scale.unwrap_or(0))?;
        Ok(Self { format, value })
    }
}

// Find the char at i. Note it returns String instead of char for complicated situations.
fn try_find_char(s: &str, i: usize, chr: u8) -> String {
    let begin = (0..=i).rev().find(|j| s.is_char_boundary(*j)).unwrap_or(0);
    let end = (i + 1..s.len())
        .find(|j| s.is_char_boundary(*j))
        .unwrap_or(s.len());
    s.get(begin..end)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| format!("{:?}", chr))
}

impl Display for PrettyDecimal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.format {
            Some(Format::Plain) | None => self.value.fmt(f),
            Some(Format::Comma3Dot) => {
                if self.value.abs() < Decimal::new(1000, 0) {
                    // no comma needed.
                    return self.value.fmt(f);
                }
                let precision = f.precision();
                if likely(f.width().is_none()) {
                    if unlikely(f.sign_plus()) {
                        f.write_str("+")?;
                    }
                    display_comma_3_dot_impl(f, self.value, precision)
                } else {
                    let mut buf = arrayvec::ArrayString::<64>::new();
                    display_comma_3_dot_impl(&mut buf, self.value.abs(), precision)?;
                    f.pad_integral(!self.value.is_sign_negative(), "", buf.as_str())
                }
            }
        }
    }
}

#[inline]
#[cold]
fn cold() {}

#[inline]
fn likely(b: bool) -> bool {
    if !b {
        cold()
    }
    b
}

#[inline]
fn unlikely(b: bool) -> bool {
    if b {
        cold()
    }
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rust_decimal_macros::dec;

    #[test]
    fn conversions() {
        let mut pd = PrettyDecimal::with_format(dec!(1), Some(Format::Comma3Dot));
        assert!(std::ptr::eq(pd.as_decimal(), &pd.value));
        assert!(std::ptr::eq(pd.as_ref(), &pd.value));
        *pd.as_mut() = dec!(2);
        let pvalue: Decimal = pd.into();
        assert_eq!(pvalue, dec!(2));
    }

    #[test]
    fn from_str_unformatted() {
        // If the number is below 1000, we can't tell if the number is plain or comma3dot.
        // Thus we declare them as unformatted instead of plain.
        assert_eq!(Ok(PrettyDecimal::unformatted(dec!(1))), "1".parse());
        assert_eq!(Ok(PrettyDecimal::unformatted(dec!(-1))), "-1".parse());

        assert_eq!(Ok(PrettyDecimal::unformatted(dec!(12))), "12".parse());
        assert_eq!(Ok(PrettyDecimal::unformatted(dec!(-12))), "-12".parse());

        assert_eq!(Ok(PrettyDecimal::unformatted(dec!(123))), "123".parse());
        assert_eq!(Ok(PrettyDecimal::unformatted(dec!(-123))), "-123".parse());

        assert_eq!(
            Ok(PrettyDecimal::unformatted(dec!(0.123450))),
            "0.123450".parse()
        );
    }

    #[test]
    fn from_str_plain() {
        assert_eq!(Ok(PrettyDecimal::plain(dec!(1234))), "1234".parse());
        assert_eq!(Ok(PrettyDecimal::plain(dec!(-1234))), "-1234".parse());

        assert_eq!(Ok(PrettyDecimal::plain(dec!(1234567))), "1234567".parse());
        assert_eq!(Ok(PrettyDecimal::plain(dec!(-1234567))), "-1234567".parse());

        assert_eq!(Ok(PrettyDecimal::plain(dec!(1234.567))), "1234.567".parse());
        assert_eq!(
            Ok(PrettyDecimal::plain(dec!(-1234.567))),
            "-1234.567".parse()
        );
    }

    #[test]
    fn from_str_comma() {
        assert_eq!(Ok(PrettyDecimal::comma3dot(dec!(1234))), "1,234".parse());
        assert_eq!(Ok(PrettyDecimal::comma3dot(dec!(-1234))), "-1,234".parse());

        assert_eq!(Ok(PrettyDecimal::comma3dot(dec!(12345))), "12,345".parse());
        assert_eq!(
            Ok(PrettyDecimal::comma3dot(dec!(-12345))),
            "-12,345".parse()
        );

        assert_eq!(
            Ok(PrettyDecimal::comma3dot(dec!(123456))),
            "123,456".parse()
        );
        assert_eq!(
            Ok(PrettyDecimal::comma3dot(dec!(-123456))),
            "-123,456".parse()
        );

        assert_eq!(
            Ok(PrettyDecimal::comma3dot(dec!(1234567))),
            "1,234,567".parse()
        );
        assert_eq!(
            Ok(PrettyDecimal::comma3dot(dec!(-1234567))),
            "-1,234,567".parse()
        );

        assert_eq!(
            Ok(PrettyDecimal::comma3dot(dec!(1234.567))),
            "+1,234.567".parse()
        );
        assert_eq!(
            Ok(PrettyDecimal::comma3dot(dec!(-1234.567))),
            "-1,234.567".parse()
        );
    }

    #[test]
    fn from_str_err() {
        assert_eq!(
            ParseError::UnexpectedChar(format!(","), 4),
            PrettyDecimal::from_str("1234,567").unwrap_err()
        );

        assert_eq!(
            ParseError::CommaRequired(5),
            PrettyDecimal::from_str("1,2345,67").unwrap_err()
        );
    }

    #[test]
    fn display_plain() {
        assert_eq!("1.234000", PrettyDecimal::plain(dec!(1.234000)).to_string());

        assert_eq!(
            "   1234.56",
            format!("{:>10}", PrettyDecimal::plain(dec!(1234.56))),
        );
    }

    #[test]
    fn to_string_comma3_dot() {
        assert_eq!("0", PrettyDecimal::comma3dot(dec!(0)).to_string());
        assert_eq!(
            "0.0000000123",
            PrettyDecimal::comma3dot(dec!(0.0000000123)).to_string()
        );

        assert_eq!("123", PrettyDecimal::comma3dot(dec!(123)).to_string());
        assert_eq!("-123.4", PrettyDecimal::comma3dot(dec!(-123.4)).to_string());

        assert_eq!(
            "999.9999",
            PrettyDecimal::comma3dot(dec!(999.9999)).to_string()
        );
        assert_eq!(
            "-999.9999",
            PrettyDecimal::comma3dot(dec!(-999.9999)).to_string()
        );

        assert_eq!("1,234", PrettyDecimal::comma3dot(dec!(1234)).to_string());
        assert_eq!("-1,234", PrettyDecimal::comma3dot(dec!(-1234)).to_string());
        assert_eq!(
            "123,456",
            PrettyDecimal::comma3dot(dec!(123456)).to_string()
        );
        assert_eq!(
            "-123,456",
            PrettyDecimal::comma3dot(dec!(-123456)).to_string()
        );

        assert_eq!(
            "1,234.1200",
            PrettyDecimal::comma3dot(dec!(1234) + dec!(0.1200)).to_string()
        );
        assert_eq!(
            "1,234.0012",
            PrettyDecimal::comma3dot(dec!(1234) + dec!(0.0012)).to_string()
        );

        assert_eq!(
            //     12345678901234567890
            "1,234.000000000000000000001",
            PrettyDecimal::comma3dot(dec!(1234) + Decimal::new(1, 21)).to_string()
        );

        assert_eq!(
            "1,234,567.890120",
            PrettyDecimal::comma3dot(dec!(1234567.890120)).to_string()
        );

        let mut min = Decimal::MIN;
        assert_eq!(
            "-79,228,162,514,264,337,593,543,950,335",
            PrettyDecimal::comma3dot(min).to_string()
        );

        min.set_scale(1).unwrap();
        min.rescale(28);

        assert_eq!(
            "-7,922,816,251,426,433,759,354,395,033.5",
            PrettyDecimal::comma3dot(min).to_string()
        );
    }

    mod display_plain_precision {
        use super::*;

        use pretty_assertions::assert_eq;

        #[test]
        fn precision_0() {
            assert_eq!(
                "-79228162514264337593543950335",
                format!("{:.0}", Decimal::MIN)
            );
        }

        #[test]
        #[ignore = "rust_decimal::Decimal format implementation is wrong"]
        fn precision_5() {
            assert_eq!(
                "-79228162514264337593543950335.00000",
                format!("{:.5}", Decimal::MIN)
            );
        }
    }

    #[test]
    fn display_comma3_dot_precision() {
        assert_eq!(
            "1,235",
            format!("{:.0}", PrettyDecimal::comma3dot(dec!(1234.56)))
        );
        assert_eq!(
            "-1,234.6",
            format!("{:.1}", PrettyDecimal::comma3dot(dec!(-1234.56)))
        );
        assert_eq!(
            "1,234.56000",
            format!("{:.5}", PrettyDecimal::comma3dot(dec!(1234.56)))
        );
    }

    // just a test to compare against standard floating number.
    #[test]
    fn display_float_fill() {
        assert_eq!("  -1.234", format!("{:>8}", -1.234));
        assert_eq!("  -1.234", format!("{:8}", -1.234));
        assert_eq!("_-1.234_", format!("{:_^8}", -1.234));
        assert_eq!("-1.234  ", format!("{:<8}", -1.234));

        // with 0 flag, alignment / fill will be ignored.
        assert_eq!("+0001.24", format!("{:<+08.2}", 1.235));
        assert_eq!("+0001.24", format!("{:+08.2}", 1.235));
        assert_eq!("+01.2340", format!("{:+08.4}", 1.234));

        // too small width ignored
        assert_eq!("123456", format!("{:3}", 123456.0));
        assert_eq!("1.00000000", format!("{:3.8}", 1.0));
    }

    #[test]
    fn display_comma3_dot_fill() {
        let val = PrettyDecimal::comma3dot(dec!(1234.56));

        assert_eq!("  1,234.56", format!("{:>10}", val));
        assert_eq!("  1,234.56", format!("{:10}", val));
        assert_eq!("_1,234.56_", format!("{:_^10}", val));
        assert_eq!("+1,234.56 ", format!("{:<+10}", val));
        assert_eq!("-1,234.56 ", format!("{:<10}", -val));

        assert_eq!("001,234.56", format!("{:>010}", val));
        assert_eq!("-01,234.56", format!("{:<010}", -val));

        // too small width ignored
        assert_eq!("1,234.56", format!("{:3}", val));
        assert_eq!("1.00000000", format!("{:3.8}", Decimal::ONE));
    }

    #[test]
    fn scale_returns_correct_number() {
        assert_eq!(0, PrettyDecimal::comma3dot(dec!(1230)).scale());
        assert_eq!(1, PrettyDecimal::comma3dot(dec!(1230.4)).scale());
    }
}
