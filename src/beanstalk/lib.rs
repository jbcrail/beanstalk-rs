#![crate_name = "beanstalk"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(macro_rules)]

pub use connection::Connection;

mod connection;
