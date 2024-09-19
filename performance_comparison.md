# Circuit Performance Comparison

| Circuit Name                 | Native Circuit Time | Native Witness Time | wasm_nodejs.txt Circuit Time | wasm_nodejs.txt Witness Time |
|------------------------------|---------------------|---------------------|------------------------------|------------------------------|
| `circuit1.circom`            | 0m0.015s            | 17.67µs             | 0m0.672s                     | 0.087ms                      |
| `circuit2.circom`            | 0m0.020s            | 23.267µs            | 0m1.270s                     | 0.143ms                      |
| `circuit3.circom`            | 0m0.008s            | 19.415µs            | 0m1.049s                     | 0.065ms                      |
| `circuit4.circom`            | 0m0.018s            | 19.53µs             | 0m0.957s                     | 0.05ms                       |
| `circuit5_poseidon.circom`   | 0m0.738s            | 99.461µs            | 0m1.021s                     | 0.794ms                      |
| `circuit6_num2bits.circom`   | 0m0.090s            | 128.364µs           | 0m0.680s                     | 1.085ms                      |
| `circuit7_poseidon4.circom`  | 0m0.868s            | 251.351µs           | 0m1.583s                     | 2.101ms                      |
| `circuit8_sha256_512.circom` | 0m6.922s            | 42.400409ms         | 0m15.599s                    | 124.657ms                    |
| `circuit9_authV2.circom`     | 0m8.347s            | 27.049364ms         | 0m22.792s                    | 60.988ms                     |
