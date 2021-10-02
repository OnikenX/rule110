# Rule 110: C vs Rust

This repo was a test of speed between c and rust after seeing the video [Why is C Faster Than My Language?](https://www.youtube.com/watch?v=vFB0Ot-ZdIM), has one of the motos of rust is abstraction without costs, i tried to translated the c code to rust and benchmarked it with the simples commands:
```bash
time sh rust_bench.sh
# and
time sh c_bench.sh
```


| Try # | Rust  | C     | Ratio (Rust/C) |
| ----- | ----- | ----- | -------------- |
| #1    | 1,58s | 0,41s | 3.8537         |
| #2    | 1,59s | 0,43s | 3.6977         |
| #3    | 1,64s | 0,45s | 3.6444         |
| Mean  | 1,60s | 0,43s | 3.7320         |

Which means that in mean rust is 3.7 times slower than c in this specific use case.