window.BENCHMARK_DATA = {
  "lastUpdate": 1752159663487,
  "repoUrl": "https://github.com/xkikeg/pretty-decimal",
  "entries": {
    "Criterion.rs Benchmark": [
      {
        "commit": {
          "author": {
            "email": "kikeg@kikeg.com",
            "name": "kikeg",
            "username": "xkikeg"
          },
          "committer": {
            "email": "kikeg@kikeg.com",
            "name": "kikeg",
            "username": "xkikeg"
          },
          "distinct": true,
          "id": "19826aa7aae76ab078913cfb4941eb10c742f26d",
          "message": "Initial implementation.\n\nIt was copied from\nhttps://github.com/xkikeg/okane/blob/658fce900deb6d1aa515edf7737551d651b3db7c/core/src/syntax/pretty_decimal.rs",
          "timestamp": "2025-07-10T16:59:31+02:00",
          "tree_id": "918a8e0f3737e86240e69736dc33df4336953524",
          "url": "https://github.com/xkikeg/pretty-decimal/commit/19826aa7aae76ab078913cfb4941eb10c742f26d"
        },
        "date": 1752159662550,
        "tool": "cargo",
        "benches": [
          {
            "name": "parse/from_str/plain",
            "value": 25,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse/from_str/comma",
            "value": 26,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "to_stirng/to_string/plain",
            "value": 86,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "to_stirng/to_string/comma",
            "value": 87,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}