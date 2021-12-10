extern crate petgraph;
extern crate literal;
extern crate bigint;

mod fileutil;
mod bits;
mod advent2019;
mod advent2021;

fn main() -> () {
    advent2021::day10::run();
}