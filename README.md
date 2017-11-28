# WASM vs JS

> TLDR; Firefox Quantum wins in JavaScript execution whereas Chrome 61 wins at execution running with WebAssembly.

## Benchmark Results on different modern browsers

| | Edge 16 | Chrome 61 | Firefox Quantum (FF57) |
| --- | ---: | ---: | ---: |
| wasm | 2395.825 | **452.140** | 2051.350 |
| js | 843.735 | 282.760 | **94.935** |
