# Journeys

See: https://github.com/mikehadlow/Journeys

Imagine a robot facing East, at coordinate (1,1).
It takes a series of steps: forward, turn right or left.
It ends up somewhere else.

This program reads a file describing journeys and checks they all makes sense.

# Build

New to Rust? :wave: Hello! You'll need to [install `rustup`](https://www.rust-lang.org/tools/install).

# Run

```
cargo run -- input.txt
```

or

```
cargo build --release
target/release/journeys input.txt
```

The output will be something like:

```
cargo run -- input.txt
   Compiling journeys v1.0.0 (/Users/richard/Developer/journeys)
    Finished dev [unoptimized + debuginfo] target(s) in 2.64s
     Running `target/debug/journeys input.txt`
Journey 0 👍
Journey 1 👍
Journey 2 👍
```

You can also run the program with `--help`.

# My implementation

Most of the logic is in _robot.rs_. There are some tests: `cargo test`.

I used this as an excuse to try out the parser combinator library (nom) for the first time.
That's in `parser.rs`.


