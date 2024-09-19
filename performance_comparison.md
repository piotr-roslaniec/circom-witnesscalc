# Circuit Performance Comparison

| Circuit Name                 | Native Circuit Time | Native Witness Time | wasm_nodejs.txt Circuit Time | wasm_nodejs.txt Witness Time |
|------------------------------|---------------------|---------------------|------------------------------|------------------------------|
| `circuit1.circom`            | 0m0.022s            | 18.513µs            | 0m0.049s                     |                              |
| `circuit2.circom`            | 0m0.043s            | 56.477µs            | N/A                          | N/A                          |
| `circuit3.circom`            | 0m0.006s            | 14.99µs             | N/A                          | N/A                          |
| `circuit4.circom`            | 0m0.019s            | 16.497µs            | N/A                          | N/A                          |
| `circuit5_poseidon.circom`   | 0m0.784s            | 109.134µs           | N/A                          | N/A                          |
| `circuit6_num2bits.circom`   | 0m0.091s            | 150.224µs           | N/A                          | N/A                          |
| `circuit7_poseidon4.circom`  | 0m1.135s            | 228.06µs            | N/A                          | N/A                          |
| `circuit8_sha256_512.circom` | 0m7.109s            | 38.877057ms         | N/A                          | N/A                          |
| `circuit9_authV2.circom`     | 0m7.362s            | 33.915273ms         | N/A                          | N/A                          |
