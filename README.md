# CCWC

## From [John Cricket's coding challenge #1](https://codingchallenges.fyi/challenges/challenge-wc/): build a wc clone CLI tool

### written with <3 using Rust 🦀

This is a simple CLI tool that mimics the functionality of the `wc` command in Unix systems. It was written as a coding challenge for fun.

This is my first Rust project, so I'm sure there are many ways to improve the code. I'm open to suggestions!

### Usage

```
ccwc [OPTIONS] [PATH]

Arguments:
  [PATH]  The path to the file to read

Options:
  -c, --bytes  Number of bytes in file
  -l, --lines  Number of lines in file
  -w, --words  Number of words in file
  -m, --chars  Number of chars in file
  -h, --help   Print help
```

#### Alternate Usage:

```
<standard input> | ccwc [OPTIONS]

example: cat file.txt | ccwc -l
```

### Installation

#### From source

```
cargo build --release

sudo mv target/release/ccwc /usr/local/bin

# alternatively, create a symbolic link

ln -s $(pwd)/target/release/ccwc /usr/local/bin/ccwc
```
