# parallel-mandelbrot-rust

This project features the computation and rendering of the Mandelbrot set to show off the concurrency capabilities of Rust. This code is based on the work of [this repository](https://github.com/GiselaMD/parallel-mandelbrot-go).

<p align="center">
    <img width="500" src="render.gif"/>
</p>

## Executing
To run the project use `cargo run --release`

To run the tests use `cargo test`

To make use of the cli args:
1. release the project with `cargo build --release`
2. Change directories to the release folder with `cd ./target/release/`
3. And execute with arguments ```./parallel-mandelbrot-rust.exe --help```


## CLI args
```
USAGE:
    parallel-mandelbrot-rust.exe [OPTIONS]

OPTIONS:
    -h, --help                           Print help information
        --max-iter <MAX_ITER>            [default: 500]
        --num-blocks <NUM_BLOCKS>        [default: 100]
        --num-threads <NUM_THREADS>      [default: 10]
        --samples <SAMPLES>              [default: 200]
        --side-lengths <SIDE_LENGTHS>    [default: 1000]
    -V, --version                        Print version information
```

## Dependencies

To render the set this project uses the library minifb and has only been tested on Windows for this project.
If problems occur on any other OS, have a look at the [Github repository of minifb](https://github.com/emoon/rust_minifb#build-instructions)

