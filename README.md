# WASM vs JS

> TLDR; Chrome 61 runs the fastest.

## Testing methodology

Run `fibonacci(32)` repeatedly for `1e3 - 1` times and measure the total execution time.

## Testing machine

| Component | Specs |
| --- | --- |
| CPU | Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz 4C8T |
| RAM | 16.0GB DDR-4 2133MHz |

## Benchmark results on different modern browsers

| Total Execution time (ms) | Edge 16 | Chrome 61 | Chrome 64 | Firefox Quantum (FF57) | Firefox Nightly (FF59) |
| --- | ---: | ---: | ---: | ---: | ---: |
| wasm | - | **7841.225** | 8575.750 | 8963.590 | 8742.015 |
| js | - | **19288.455** | 23067.515 | 20580.420 | 20696.595 |

_** Unable to run on Edge 16 without page crashing._
