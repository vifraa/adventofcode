# Advent of Code
This repository contains my solutions to [Advent of Code 2021](https://adventofcode.com/).
The repository is structured as a CLI that can be used to run each
day and part through command line arguments. 

## Running
```
cargo run -- --day {day} --part {part} --input {input}
```

## Benchmarks
I am using
[hyperfine](https://github.com/sharkdp/hyperfine) to measure the execution
time. 
Everything from loading the input from disk to printing results is included in
the timing. Be aware that almost no speed optimizations will be made, I will
put more focus into learning how to write idiomatic Rust code.
The hyperfine command used for each day can be seen below.
```
hyperfine --warmup 1 -r 100 "cargo run --release -- --day {day} --part {part} --input {path to input}
```

| Day | Part A | Part B |
| --- | --- | --- |
| Day 1 | 52.4ms (+- 0.9ms) | 52.5ms (+- 0.7ms) |
|Day 2 | 52.2ms (+- 1.2ms) | 52.0ms (+- 1.0ms)|

