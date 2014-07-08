#![crate_id = "beanstalk#0.0.1"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]

#![feature(macro_rules)]

pub use connection::Connection;

mod connection;
