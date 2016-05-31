/// ### Build this example
/// ```
/// cargo build --example pair
/// ```
/// ### Run this example
/// ```
/// cargo run --example pair -- node0 ipc:///tmp/pair.ipc &
/// cargo run --example pair -- node1 ipc:///tmp/pair.ipc &
/// sleep 3
/// killall pair
/// ```
/// ### Expected output
/// ```
/// node0: SENDING "node0"
/// node1: SENDING "node1"
/// node1: RECEIVED "node0"
/// node0: RECEIVED "node1"
/// node1: SENDING "node1"
/// node1: RECEIVED "node0"
/// node0: SENDING "node0"
/// node0: RECEIVED "node1"
/// ```
extern crate nanomsg;
extern crate time;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration};


fn send_name(socket: &mut Socket, name: &str) {
    match socket.write_all(name.as_bytes()) {
        Ok(_) => {println!("{}: SENDING \"{}\"", name, name);},
        Err(err) => println!("Could not send: {}", err)
    }
}

fn receive_name(socket: &mut Socket, name: &str) -> String {
    let mut result = String::new();
    match socket.read_to_string(&mut result) {
        Ok(_) => println!("{}: RECEIVED \"{}\"", name, &result),
        Err(_) => {},
    }

    result
}

fn send_receive(socket: &mut Socket, name: &str) {
    let _ = socket.set_receive_timeout(100); // without this receive_name would block and never time out
    loop {
        receive_name(socket, name);
        thread::sleep(Duration::from_millis(1000));
        send_name(socket, name);
    }
}

fn node0(url: String) {
    let mut socket = Socket::new(Protocol::Pair).unwrap();
    let mut endpoint = socket.bind(&url).unwrap();

    send_receive(&mut socket, "node0");

    let _ = endpoint.shutdown();
}

fn node1(url: String) {
    let mut socket = Socket::new(Protocol::Pair).unwrap();
    let mut endpoint = socket.connect(&url).unwrap();

    send_receive(&mut socket, "node1");

    let _ = endpoint.shutdown();
}

fn usage() {
    println!("Usage: pair node0|node1 <URL>");
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
