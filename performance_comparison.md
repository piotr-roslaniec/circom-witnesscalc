# Circuit Performance Comparison

| Circuit Name                 | Native Circuit Time | Native Witness Time | wasm_nodejs.txt Circuit Time | wasm_nodejs.txt Witness Time |
|------------------------------|---------------------|---------------------|------------------------------|------------------------------|
| `circuit1.circom`            | 0m0.014s            | 20.482µs            | 0m0.761s                     | 0.045ms                      |
| `circuit2.circom`            | 0m0.028s            | 48.836µs            | 0m0.817s                     | 0.073ms                      |
| `circuit3.circom`            | 0m0.007s            | 18.905µs            | 0m0.841s                     | 0.042ms                      |
| `circuit4.circom`            | 0m0.020s            | 22.768µs            | 0m0.788s                     | 0.047ms                      |
| `circuit5_poseidon.circom`   | 0m0.803s            | 103.313µs           | 0m1.258s                     | 0.489ms                      |
| `circuit6_num2bits.circom`   | 0m0.123s            | 167.22µs            | 0m0.795s                     | 1.369ms                      |
| `circuit7_poseidon4.circom`  | 0m1.102s            | 261.25µs            | 0m1.501s                     | 1.019ms                      |
| `circuit8_sha256_512.circom` | 0m6.160s            | 26.932778ms         | 0m12.159s                    | 96.835ms                     |
| `circuit9_authV2.circom`     | 0m9.803s            | 27.584395ms         | 0m20.957s                    | 73.385ms                     |
