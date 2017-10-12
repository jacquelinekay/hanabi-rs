#![feature(plugin, use_extern_macros)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate tarpc;

pub mod config;
pub mod display;
pub mod server;
pub mod state;
pub mod types;
pub mod player;
pub mod player_type_conversion;
