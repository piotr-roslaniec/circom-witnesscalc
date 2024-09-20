# Circuit Performance Comparison

| Circuit Name         | Native Circuit Time | Native Witness Time | wasm_nodejs.txt Circuit Time | wasm_nodejs.txt Witness Time |
|----------------------|-----------------------|-----------------------|-----------------------|-----------------------|
| `circuit1.circom` | 0m0.022s | 20.771µs  | 0m0.687s | 0.053ms  |
| `circuit2.circom` | 0m0.036s | 23.611µs  | 0m0.650s | 0.082ms  |
| `circuit3.circom` | 0m0.006s | 21.97µs  | 0m0.747s | 0.059ms  |
| `circuit4.circom` | 0m0.018s | 16.459µs  | 0m0.714s | 0.253ms  |
| `circuit5_poseidon.circom` | 0m0.766s | 95.892µs  | 0m1.011s | 0.648ms  |
| `circuit6_num2bits.circom` | 0m0.122s | 234.461µs  | 0m0.754s | 0.826ms  |
| `circuit7_poseidon4.circom` | 0m1.747s | 273.599µs  | 0m1.380s | 1.027ms  |
| `circuit8_sha256_512.circom` | 0m5.180s | 25.28094ms  | 0m12.080s | 177.191ms  |
| `circuit9_authV2.circom` | 0m7.024s | 27.89077ms  | 0m20.472s | 54.368ms  |
