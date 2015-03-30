extern crate beanstalk;

use std::env;
use std::io::{BufRead, BufReader, stdin};
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
    let args: Vec<String> = env::args().map(|s| s.to_string()).collect();

    let host = if args.len() > 1 {
        args[1].clone()
    } else {
        "127.0.0.1".to_string()
    };

    let port: u16 = if args.len() > 2 {
        args[2].parse::<u16>().unwrap()
    } else {
        11300
    };

    println!("# connecting to {}:{}", host, port);
    println!("# type 'help' for available commands");
    let mut conn = Connection::new(&host, port).unwrap();

    loop {
        print!("> ");

        let mut reader = BufReader::new(stdin());
        let mut line = String::new();
        let _ = reader.read_line(&mut line);
        let input = line.trim_right();
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
