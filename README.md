# pi-generator-from-blocks

![Demo](demo.gif)

This is a way to generate digits of pi by using inelastic collisions
between an object 100^N larger than the other with a wall and counting
the number of collisions. 

Inspired by [this](https://www.youtube.com/watch?v=HEfHFsfGXjs) video
by 3Blue1Brown, [technical
paper](https://www.maths.tcd.ie/~lebed/Galperin).

## Usage

This program is written in Rust, primarily for speed. First, clone the
repository using `git clone
https://github.com/jakevossen5/pi-generator-from-blocks.git` 
or by downloading and extracting the zip. 

Then, install Rust from the [offical rust
guide](https://www.rust-lang.org/tools/install).

Compile the program using `rustc main.rs -o main`. Then you can run
the program using `main`. By default, it will start with 1 digit, then
2, then 3, etc, until you cancel the program. You can modify the
starting value by change `n` in the source and recompiling.

## Notes

I found that 9 is about the limit a modern computer can calculate in a
reasonable time (about 10 seconds on my 2018 Macbook Pro, and similar
speeds on other computers). It gets _very_ hard to calculate numbers
above 10. 
