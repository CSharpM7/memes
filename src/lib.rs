#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused
)]
#![deny(
    deprecated
)]

#[macro_use]
extern crate lazy_static;

mod pickel;
mod chargeable_throws;
mod installer;

mod imports;
//mod custom_vars;
pub mod data;
use data::gamemode::*;

pub use skyline::libc::c_char;

extern "C"{
    /// gets whether we are in training mode
    #[link_name = "\u{1}_ZN3app9smashball16is_training_modeEv"]
    pub fn is_training_mode() -> bool;
}


#[skyline::main(name = "smashline_memes")]
pub fn main() {
    data::gamemode::set_gamemode();
    //data::install();

    println!("[smashline_memes::main] Loading...");
    #[cfg(not(feature = "dev"))]{
        //Add hooks here
        #[cfg(feature = "devhook")]{
        println!("[smashline_memes::main] Dev Hook Installed");
        return;
        }
        installer::install();
    }

    #[cfg(feature = "dev")]
    installer::installer();
    println!("[smashline_memes::main] Loaded!");
}