extern crate petgraph;
extern crate literal;
extern crate bigint;
extern crate itertools;

mod fileutil;
mod bits;
mod advent2019;
mod advent2021;

fn main() -> () {
    advent2021::day11::run();
}