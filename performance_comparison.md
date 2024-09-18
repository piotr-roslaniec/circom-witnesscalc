# Circuit Performance Comparison

| Circuit Name                 | Native Circuit Time | Native Witness Time | wasm_nodejs.txt Circuit Time | wasm_nodejs.txt Witness Time |
|------------------------------|---------------------|---------------------|------------------------------|------------------------------|
| `circuit1.circom`            | 0m0.015s            | 15.475µs            | 0m0.761s                     | 0.045ms                      |
| `circuit2.circom`            | 0m0.020s            | 25.991µs            | 0m0.817s                     | 0.073ms                      |
| `circuit3.circom`            | 0m0.007s            | 18.856µs            | 0m0.841s                     | 0.042ms                      | 
| `circuit4.circom`            | 0m0.019s            | 18.273µs            | 0m0.788s                     | 0.047ms                      |
| `circuit5_poseidon.circom`   | 0m1.016s            | 89.357µs            | 0m1.258s                     | 0.489ms                      |
| `circuit6_num2bits.circom`   | 0m0.094s            | 126.67µs            | 0m0.795s                     | 1.369ms                      |
| `circuit7_poseidon4.circom`  | 0m0.929s            | 216.199µs           | 0m1.501s                     | 1.019ms                      |
| `circuit8_sha256_512.circom` | 0m4.887s            | 45.367202ms         | 0m12.159s                    | 96.835ms                     |
| `circuit9_authV2.circom`     | 0m6.994s            | 28.441522ms         | 0m20.957s                    | 73.385ms                     |