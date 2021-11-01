# brainfuck (complete)

An interpreter written in rust, for code written in brainfuck<br />
Please use the latest binary(built for linux) for use

# Building instructions
You must first install the [rust](https://www.rust-lang.org) compiler and the Cargo package management system.

in the package, run
```
cargo run --release <filename1> <filename2> ...
```
or just 
```
cargo run --release
```
to process code in the terminal itself

Visit this [site](https://gist.github.com/roachhd/dce54bec8ba55fb17d3a) to learn the language.<br />

# Specification
<ol>
    <li>There are errors for overflow/underflow.</li>
    <li>Cells dont wraparound :)</li>
    <li>Rest is as usual</li>
</ol>
