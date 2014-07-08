#![crate_id = "beanstalk-example#0.0.1"]
#![crate_type = "bin"]

extern crate beanstalk;

use std::os;
use beanstalk::Connection;

fn main() {
    let args = os::args();

    let host = if args.len() > 1 {
        args.get(1).clone()
    } else {
        String::from_str("127.0.0.1").clone()
    };

    let port: u16 = if args.len() > 2 {
        from_str::<u16>(args.get(2).as_slice()).unwrap()
    } else {
        11300
    };

    println!("# connecting to {}:{}", host, port);
    let mut conn = Connection::new(host.as_slice(), port).unwrap();

    conn.use_tube("pending");
    conn.add_job(1, 0, 600, b"abcdefghijklmnopqrstuvwxyz");
    conn.stats();
    conn.list_tubes();
    conn.list_used_tube();
    conn.list_watched_tubes();
    conn.tube_stats("pending");
    conn.delete_job(1);
    conn.quit();
}
