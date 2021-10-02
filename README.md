# Rule 110: C vs Rust

This repo was a test of speed between c and rust after seeing the video [Why is C Faster Than My Language?](https://www.youtube.com/watch?v=vFB0Ot-ZdIM), has one of the motos of rust is abstraction without costs, i tried to translated the c code to rust and benchmarked it with the simples commands:
```bash
time sh rust_bench_c.sh
# and
time sh c_bench.sh
```


| Try # | Rust  | C     | Ratio (Rust/C) |
| ----- | ----- | ----- | -------------- |
| #1    | 1,58s | 0,41s | 3.8537         |
| #2    | 1,59s | 0,43s | 3.6977         |
| #3    | 1,64s | 0,45s | 3.6444         |
| Mean  | 1,60s | 0,43s | 3.7320         |

Which means that in mean rust is 3.7 times slower than c in this specific use case, at least without a buffered.

With a buffer for the output it gets a little better, the user *fnky* has a buffered version in https://gist.github.com/fnky/286f0cfbaaf203be706d7d3e1ea1ff88 , I changing the flush of the buffer for the end of the program so it only calls the system call once, in the end the values of the buffered version are these:

| Try # | Rust (buffered) | C     | Ratio (Rust/C) |
| ----- | --------------- | ----- | -------------- |
| #1    | 1,16s           | 0,41s | 3.8537         |
| #2    | 1,07s           | 0,43s | 3.6977         |
| #3    | 1,09s           | 0,45s | 3.6444         |
| Mean  | 1,11s           | 0,43s | 2.5816         |

As we can see rust is still takes more time than but now is only 2.5 compared to C.