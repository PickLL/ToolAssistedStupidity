#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(unused)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

mod mario;
mod donkey;
mod link;
mod samus;
mod samusd;
mod yoshi;
mod fox;
mod kirby;
mod pikachu;
mod luigi;
mod ness;
mod falcon;
mod purin;

mod falco;

mod sheik;

mod dedede;

mod custom;

#[skyline::main(name = "TAS")]
pub fn main() {
    mario::install();
    donkey::install();
    link::install();
    samus::install();
    samusd::install();
    yoshi::install();
    fox::install();
    kirby::install();
    pikachu::install();
    luigi::install();
    ness::install();
    falcon::install();
    purin::install();

    falco::install();

    sheik::install();

    dedede::install();

    custom::install();
}