use std::io::{IoResult, BufferedStream, TcpStream};
use std::string::String;

macro_rules! execute {
    ($obj:expr, $cmd:expr) => ($obj.execute($cmd, vec!(), &[]));
    ($obj:expr, $cmd:expr, $($arg:tt)*) => ($obj.execute($cmd, vec!($($arg)*), &[]));
}

pub struct Connection {
    stream: BufferedStream<TcpStream>
}

impl Connection {
    pub fn new(host: &str, port: u16) -> Result<Connection, &str> {
        let sock = match TcpStream::connect((host, port)) {
            Ok(x) => x,
            Err(_) => { return Err("connection refused") },
        };

        let rv = Connection { stream: BufferedStream::new(sock) };

        Ok(rv)
    }

    fn send_command(&mut self, cmd: &str, args: Vec<String>, data: &[u8]) -> IoResult<()> {
        macro_rules! write { ($bytes:expr) => (let _ = self.stream.write($bytes)) }

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
            panic!(format!("beanstalkd command failed"));
        }

        let resp = response.unwrap();
        let line = resp.as_slice().trim_right();
        println!("{}", line);
        let fields: Vec<&str> = line.split(' ').collect();
        if fields.len() > 0 {
            match fields[0] {
                "OK" | "FOUND" | "RESERVED" => {
                    let bytes = from_str::<uint>(fields[fields.len()-1]).unwrap();
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
            vec!(priority.to_string(), delay.to_string(), ttr.to_string(), payload.len().to_string()),
            payload);
    }

    pub fn reserve_job(&mut self) {
        execute!(self, "reserve");
    }

    pub fn reserve_job_with_timeout(&mut self, seconds: u32) {
        execute!(self, "reserve-with-timeout", seconds.to_string());
    }

    pub fn bury_job(&mut self, id: u32, priority: u32) {
        execute!(self, "bury", id.to_string(), priority.to_string());
    }

    pub fn release_job(&mut self, id: u32, priority: u32, delay: u32) {
        execute!(self, "release", id.to_string(), priority.to_string(), delay.to_string());
    }

    pub fn touch_job(&mut self, id: u32) {
        execute!(self, "touch", id.to_string());
    }

    pub fn delete_job(&mut self, id: u32) {
        execute!(self, "delete", id.to_string());
    }

    pub fn kick_job(&mut self, id: u32) {
        execute!(self, "kick-job", id.to_string());
    }

    pub fn kick(&mut self, bound: u32) {
        execute!(self, "kick", bound.to_string());
    }

    pub fn peek(&mut self, id: u32) {
        execute!(self, "peek", id.to_string());
    }

    pub fn peek_ready(&mut self) {
        execute!(self, "peek-ready");
    }

    pub fn peek_delayed(&mut self) {
        execute!(self, "peek-delayed");
    }

    pub fn peek_buried(&mut self) {
        execute!(self, "peek-buried");
    }

    // tube commands

    pub fn use_tube(&mut self, tube: &str) {
        execute!(self, "use", tube.to_string());
    }

    pub fn watch_tube(&mut self, tube: &str) {
        execute!(self, "watch", tube.to_string());
    }

    pub fn ignore_tube(&mut self, tube: &str) {
        execute!(self, "ignore", tube.to_string());
    }

    pub fn pause_tube(&mut self, tube: &str, delay: u32) {
        execute!(self, "pause-tube", tube.to_string(), delay.to_string());
    }

    pub fn list_tubes(&mut self) {
        execute!(self, "list-tubes");
    }

    pub fn list_used_tube(&mut self) {
        execute!(self, "list-tube-used");
    }

    pub fn list_watched_tubes(&mut self) {
        execute!(self, "list-tubes-watched");
    }

    // statistics commands

    pub fn stats(&mut self) {
        execute!(self, "stats");
    }

    pub fn job_stats(&mut self, id: u32) {
        execute!(self, "stats-job", id.to_string());
    }

    pub fn tube_stats(&mut self, tube: &str) {
        execute!(self, "stats-tube", tube.to_string());
    }

    // miscellaneous commands

    pub fn quit(&mut self) {
        execute!(self, "quit");
        drop(self);
    }
}
