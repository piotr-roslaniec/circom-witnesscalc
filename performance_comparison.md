# Circuit Performance Comparison

| Circuit Name                 | Native Circuit Time | Native Witness Time | wasm_nodejs.txt Circuit Time | wasm_nodejs.txt Witness Time |
|------------------------------|---------------------|---------------------|------------------------------|------------------------------|
| `circuit1.circom`            | 0m0.019s            | 47.69µs             | 0m1.034s                     | 0.058ms                      |
| `circuit2.circom`            | 0m0.033s            | 77.077µs            | 0m0.881s                     | 0.08ms                       |
| `circuit3.circom`            | 0m0.010s            | 18.674µs            | 0m0.828s                     | 0.046ms                      |
| `circuit4.circom`            | 0m0.026s            | 25.993µs            | 0m0.905s                     | 0.061ms                      |
| `circuit5_poseidon.circom`   | 0m0.929s            | 125.651µs           | 0m1.281s                     | 0.483ms                      |
| `circuit6_num2bits.circom`   | 0m0.110s            | 154.51µs            | 0m3.219s                     | 4.121ms                      |
| `circuit7_poseidon4.circom`  | 0m1.804s            | 403.842µs           | 0m1.837s                     | 1.868ms                      |
| `circuit8_sha256_512.circom` | 0m6.421s            | 30.385175ms         | 0m13.497s                    | 112.413ms                    |
| `circuit9_authV2.circom`     | 0m7.681s            | 29.34194ms          | 0m21.967s                    | 68.451ms                     |
