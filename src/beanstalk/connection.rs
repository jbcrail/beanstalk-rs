use std::io::{IoResult, BufferedStream, TcpStream};
use std::string::String;

macro_rules! write(
    ($bytes:expr) => (let _ = self.stream.write($bytes));
)

macro_rules! execute(
    ($cmd:expr) => (self.execute($cmd, vec!(), []));
    ($cmd:expr, $($arg:tt)*) => (self.execute($cmd, vec!($($arg)*), []));
)

pub struct Connection {
    stream: BufferedStream<TcpStream>
}

impl Connection {
    pub fn new(host: &str, port: u16) -> Result<Connection, &str> {
        let sock = match TcpStream::connect(host, port) {
            Ok(x) => x,
            Err(_) => { return Err("connection refused") },
        };

        let rv = Connection { stream: BufferedStream::new(sock) };

        Ok(rv)
    }

    fn send_command(&mut self, cmd: &str, args: Vec<String>, data: &[u8]) -> IoResult<()> {
        println!("> {}", cmd);
        write!(cmd.as_bytes());
        for arg in args.iter() {
            write!(b" ");
            write!(arg.as_bytes());
        }
        write!(b"\r\n");
        if data.len() > 0 {
            write!(data);
            write!(b"\r\n");
        }
        self.stream.flush()
    }

    fn read_response(&mut self) -> IoResult<String> {
        self.stream.read_line()
    }

    fn execute(&mut self, cmd: &str, args: Vec<String>, data: &[u8]) {
        let _ = self.send_command(cmd, args, data);
        let response = self.read_response();
        if response.is_err() {
            fail!(format!("beanstalkd command failed"));
        }

        let resp = response.unwrap();
        let line = resp.as_slice().trim_right();
        println!("{}", line);
        let fields: Vec<&str> = line.split(' ').collect();
        if fields.len() > 0 {
            match *fields.get(0) {
                "OK" | "FOUND" | "RESERVED" => {
                    let bytes = from_str::<uint>(*fields.get(fields.len()-1)).unwrap();
                    let payload = self.stream.read_exact(bytes+2).unwrap();
                    println!("{}", String::from_utf8(payload).unwrap().as_slice().trim_right());
                },
                _ => {}
            }
        }
        println!("");
    }

    // job commands

    pub fn add_job(&mut self, priority: u32, delay: u32, ttr: u32, payload: &[u8]) {
        self.execute(
            "put",
            vec!(priority.to_str(), delay.to_str(), ttr.to_str(), payload.len().to_str()),
            payload);
    }

    pub fn reserve_job(&mut self) {
        execute!("reserve");
    }

    pub fn reserve_job_with_timeout(&mut self, seconds: u32) {
        execute!("reserve-with-timeout", seconds.to_str());
    }

    pub fn bury_job(&mut self, id: u32, priority: u32) {
        execute!("bury", id.to_str(), priority.to_str());
    }

    pub fn release_job(&mut self, id: u32, priority: u32, delay: u32) {
        execute!("release", id.to_str(), priority.to_str(), delay.to_str());
    }

    pub fn touch_job(&mut self, id: u32) {
        execute!("touch", id.to_str());
    }

    pub fn delete_job(&mut self, id: u32) {
        execute!("delete", id.to_str());
    }

    pub fn kick_job(&mut self, id: u32) {
        execute!("kick-job", id.to_str());
    }

    pub fn kick(&mut self, bound: u32) {
        execute!("kick", bound.to_str());
    }

    pub fn peek(&mut self, id: u32) {
        execute!("peek", id.to_str());
    }

    pub fn peek_ready(&mut self) {
        execute!("peek-ready");
    }

    pub fn peek_delayed(&mut self) {
        execute!("peek-delayed");
    }

    pub fn peek_buried(&mut self) {
        execute!("peek-buried");
    }

    // tube commands

    pub fn use_tube(&mut self, tube: &str) {
        execute!("use", tube.to_str());
    }

    pub fn watch_tube(&mut self, tube: &str) {
        execute!("watch", tube.to_str());
    }

    pub fn ignore_tube(&mut self, tube: &str) {
        execute!("ignore", tube.to_str());
    }

    pub fn pause_tube(&mut self, tube: &str, delay: u32) {
        execute!("pause-tube", tube.to_str(), delay.to_str());
    }

    pub fn list_tubes(&mut self) {
        execute!("list-tubes");
    }

    pub fn list_used_tube(&mut self) {
        execute!("list-tube-used");
    }

    pub fn list_watched_tubes(&mut self) {
        execute!("list-tubes-watched");
    }

    // statistics commands

    pub fn stats(&mut self) {
        execute!("stats");
    }

    pub fn job_stats(&mut self, id: u32) {
        execute!("stats-job", id.to_str());
    }

    pub fn tube_stats(&mut self, tube: &str) {
        execute!("stats-tube", tube.to_str());
    }

    // miscellaneous commands

    pub fn quit(&mut self) {
        execute!("quit");
        drop(self);
    }
}
