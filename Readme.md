# SLP Benchmarks

Compares
[peppi](https://github.com/hohav/peppi)
with
[slpz](https://github.com/AlexanderHarrison/slpz)
and
[slp_parser](https://github.com/AlexanderHarrison/slp_parser)
on a variety of parameters.

This is not a very fair comparison.
For example, slp_parser doesn't parse everything in an slp_file, only around 75% of it.
It's intended for use in rwing, not as a general purpose library.
But I found it interesting to compare them.

- peppi: 2.0.1
- slp_parser: commit #13934b0
- slpz: 1.2.0

### Benchmarks
Clone and run this repo to generate your own benchmarks.
Results are not averaged, assume ~20% error margin.

```
READ FULL GAME
22608us peppi slp parse full
7904us  peppi slpp parse full
10482us peppi zstd slpp parse full
5491us  peppi lz4 slpp parse full
2892us  slp_parser slp parse full
17680us slp_parser slpz parse full

READ GAME INFO
69us    peppi slp parse info
71us    peppi slpp parse info
49us    peppi zstd_slpp parse info
49us    peppi lz4_slpp parse info
1us     slp_parser slp parse info
0us     slp_parser slpz parse info

COMPRESS FULL GAME
35ms    peppi compress slp zstd  667kb   (11%)
25ms    peppi compress slp lzma  1785kb  (30%)
23ms    slpz compress slp fast   762kb   (12%)
186ms   slpz compress slp slow   569kb   (9%)
```

### Dependencies:
Calculated by counting the number of deps in a Cargo.lock file.

| lib | direct deps | total deps |
| ----- | ----- | ----- |
| peppi       | 10 | 104 |
| slp_parser  | 2  | 25  |
| slpz        | 1  | 4   |

### Lines of Code
Calculated by [clocc](https://github.com/AlexanderHarrison/clocc).

This is pretty useless as a metric.
Both slp_parser and peppi have more functionality than parsing and compression.

| lib | code | total |
| ----- | ----- | ----- |
| peppi       | 5700 | 6800 |
| slp_parser  | 6500 | 7300 |
| slpz        | 553  | 700  |

### Supported slp Versions
Found by reading the source code.

| lib | min | max |
| ----- | ----- | ----- |
| peppi       | 0.1.0 (?) unsure | 3.16.0 (3.18.0 implemented but not pushed to crates.io) |
| slp_parser  | 3.0.0 | none |
| slpz        | 0.1.0 | none |

### Takeaways
- peppi is around 10x slower than slp_parser at parsing simple slp files (again, not a totally fair comparison).
- I need to improve the performance of parsing slpz files!

