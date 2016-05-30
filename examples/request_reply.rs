/// ### Build this example
/// ```
/// cargo build --example request_reply
/// ```
/// ### Run this example
/// ```
/// cargo run --example request_reply -- node0 ipc:///tmp/request_reply.ipc &
/// cargo run --example request_reply -- node1 ipc:///tmp/request_reply.ipc
/// killall request_reply
/// ```
/// ### Expected output
/// ```
/// NODE1: SENDING DATE REQUEST DATE
/// NODE0: RECEIVED DATE REQUEST
/// NODE0: SENDING DATE Mon May 30 12:02:01 2016
/// NODE1: RECEIVED DATE Mon May 30 12:02:01 2016
/// ```
extern crate nanomsg;
extern crate time;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration};


fn date() -> String {
    let time = time::get_time();
    let local = time::at(time);
    local.asctime().to_string()
}

fn node0(url: String) {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    let mut endpoint = socket.bind(&url).unwrap();

    let mut request = String::new();

    loop {
        match socket.read_to_string(&mut request) {
            Ok(_) => {
                println!("NODE0: RECEIVED DATE REQUEST");
                let reply = format!("{}", date());
                match socket.write_all(reply.as_bytes()) {
                    Ok(..) => { println!("NODE0: SENDING DATE {}", reply); },
                    Err(err) => {
                        println!("NODE0: Failed to send reply '{}'", err);
                        break
                    }
                }
                request.clear();
            },
            Err(err) => {
                panic!("NODE0: Problem while reading: '{}'", err);
            }
        }
        thread::sleep(Duration::from_millis(400));
    }
    let _ = endpoint.shutdown();
    drop(socket);
}

fn node1(url: String) {
    let mut socket = Socket::new(Protocol::Req).unwrap();
    let mut endpoint = socket.connect(&url).unwrap();

    let mut request = String::new();

    println!("NODE1: SENDING DATE REQUEST {}", "DATE");
    match socket.write_all("DATE".as_bytes()) {
        Ok(_) => {
            match socket.read_to_string(&mut request) {
                Ok(_) => { println!("NODE1: RECEIVED DATE {}", request); },
                Err(err) => { println!("NODE1: Failed to read replay '{}'", err); }
            }
        },
        Err(err) => { println!("NODE1: Failed to write DATE request '{}'", err); }
    }
    let _ = endpoint.shutdown();
}


fn usage() {
    println!("Usage: request_reply node0|node1 <URL>");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return usage()
    }

    match args[1].as_ref() {
        "node0" => node0(args[2].clone()),
        "node1" => node1(args[2].clone()),
        _ => usage(),
    }
}
