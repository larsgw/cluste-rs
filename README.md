## Dependencies

  - [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo)
  - [`rand`](https://rust-random.github.io/book/) and its transitive dependencies
    (will be installed by Cargo when building/running the code)

## Test

Run `cargo test` to run the unit tests.

## Run

```
cargo run --release < input.csv > ouptut.csv
```

  - `input.csv` should contain only numeric columns to run k-means clustering on.
  - `output.csv` will contain the center coordinates.
  - Timing information will be printed to standard error.
