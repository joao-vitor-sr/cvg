# svg

A SV Generator written in rust.

## Installation

By now there's no pre-compiled bnaries, so you have to compile by yourself
using the [cargo](https://github.com/rust-lang/cargo) tool

1. Copy the repository with `git clone https://github.com/joao-vitor-sr/svg`
2. Enter the directroy with `cd svg/`
3. Compile the binary with `cargo build --release`
4. copy the binary to your path (the binary is located at
   `target/release/svg/`)

## Usage

run `svg` with a file conatining the data as an argument

```
svg data.toml
```

there's a example of this file on the repository, the file is called
`data.toml`
