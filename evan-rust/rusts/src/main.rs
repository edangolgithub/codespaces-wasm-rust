#![allow(dead_code)]
#![allow(unused_variables)]
#![warn(unused_imports)]

mod dsa;
mod enum_mod;
mod estruct;
mod func;
mod function;
mod match_mod;
mod mod_mod;
mod owner_mod;
mod slice_mod;
mod vectors_mod;

mod usefuls;

mod generic_mod;

mod db_mod;
mod guessing_game;
mod string_mod;

//#[allow(unused_variables)]
fn run() {
    dotenvy::dotenv().ok();
    owner_mod::run();
}

fn main() {
   
    run();
}
