#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(unused_imports)]

mod dsa;
mod estruct;
mod func;
mod function;
mod match_mod;
mod owner_mod;
mod mod_mod;
mod vectors_mod;
mod enum_mod;
mod slice_mod;

mod usefuls;

mod generic_mod;

mod string_mod;
mod guessing_game;

//#[allow(unused_variables)]
fn run() {
guessing_game::load_game();
}




fn main() {
    run();
}