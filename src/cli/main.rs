#![crate_name = "beanstalk-cli"]
#![crate_type = "bin"]

extern crate beanstalk;

use std::io;
use std::io::BufferedReader;
use std::os;
use beanstalk::Connection;

fn help() {
    println!("Available commands:\n");
    println!("  reserve");
    println!("  peek-ready");
    println!("  peek-delayed");
    println!("  peek-buried");
    println!("  list-tubes");
    println!("  list-used-tube");
    println!("  list-watched-tubes");
    println!("  stats");
    println!("  use <tube>");
    println!("  watch <tube>");
    println!("  ignore <tube>");
    println!("  tube-stats <tube>");
    println!("  quit");
    println!("");
}

fn main() {
    let args = os::args();

    let host = if args.len() > 1 {
        args[1].clone()
    } else {
        String::from_str("127.0.0.1").clone()
    };

    let port: u16 = if args.len() > 2 {
        from_str::<u16>(args[2].as_slice()).unwrap()
    } else {
        11300
    };

    println!("# connecting to {}:{}", host, port);
    println!("# type 'help' for available commands");
    let mut conn = Connection::new(host.as_slice(), port).unwrap();

    loop {
        print!("> ");

        let mut reader = BufferedReader::new(io::stdin());
        let line = reader.read_line().unwrap();
        let input = line.as_slice().trim_right();
        let args: Vec<&str> = input.split(' ').collect();

        match args[0] {
            "help" => { help() },
            "reserve" => {},
            "peek-ready" => {},
            "peek-delayed" => {},
            "peek-buried" => {},
            "list-tubes" => { conn.list_tubes() },
            "list-used-tube" => { conn.list_used_tube() },
            "list-watched-tubes" => { conn.list_watched_tubes() },
            "stats" => { conn.stats() },
            "use" => { conn.use_tube(args[1]) },
            "watch" => { conn.watch_tube(args[1]) },
            "ignore" => { conn.ignore_tube(args[1]) },
            "tube-stats" => { conn.tube_stats(args[1]) },
            "quit" => {
                conn.quit();
                break;
            },
            _ => {}
        }
    }
}
