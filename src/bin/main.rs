extern crate beanstalk;
extern crate getopts;
extern crate linenoise;

use beanstalk::Connection;
use std::env;
use std::io::BufRead;

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

fn callback(input: &str) -> Vec<String> {
    let ret: Vec<&str>;
    if input.starts_with("i") {
        ret = vec!["ignore"];
    } else if input.starts_with("l") {
        ret = vec!["list-tubes", "list-used-tube", "list-watched-tubes"];
    } else if input.starts_with("p") {
        ret = vec!["peek-buried", "peek-delayed", "peek-ready"];
    } else if input.starts_with("q") {
        ret = vec!["quit"];
    } else if input.starts_with("r") {
        ret = vec!["reserve"];
    } else if input.starts_with("s") {
        ret = vec!["stats"];
    } else if input.starts_with("t") {
        ret = vec!["tube-stats"];
    } else if input.starts_with("u") {
        ret = vec!["use"];
    } else if input.starts_with("w") {
        ret = vec!["watch"];
    } else {
        ret = vec!["help"];
    }

    return ret.iter().map(|s| s.to_string()).collect();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let opts = getopts::Options::new();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("Invalid options: {}", f)
    };

    let host = if matches.free.len() > 0 {
        matches.free[0].clone()
    } else {
        "127.0.0.1".to_string()
    };

    let port: u16 = if matches.free.len() > 1 {
        matches.free[1].parse::<u16>().unwrap()
    } else {
        11300
    };

    println!("# connecting to {}:{}", host, port);
    println!("# type 'help' for available commands");
    let mut conn = Connection::new(&host, port).unwrap();
    linenoise::set_callback(callback);

    loop {
        let val = linenoise::input("> ");

        match val {
            None => { break }
            Some(input) => {
                linenoise::history_add(&input[..]);
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
    }
}
