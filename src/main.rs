extern crate petgraph;
extern crate literal;

mod fileutil;
mod bits;
mod advent2019;
mod advent2021;

fn main() -> () {
    advent2021::day09::run();
}