/// ### Build this example
/// ```
/// cargo build --example pipeline
/// ```
/// ### Run this example
/// ```
/// cargo run --example pipeline -- node0 ipc:///tmp/pipeline.ipc & node0=$! && sleep 1
/// cargo run --example pipeline -- node1 ipc:///tmp/pipeline.ipc "Hello, World\!"
/// cargo run --example pipeline -- node1 ipc:///tmp/pipeline.ipc "Goodbye."
/// kill $node0
/// ```
/// ### Expected output
/// ```
/// NODE1: SENDING 'Hello, World!'
/// NODE0: RECEIVED 'Hello, World!'
/// NODE1: SENDING 'Goodbye.'
/// NODE0: RECEIVED 'Goodbye.'
/// ```
extern crate nanomsg;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};

fn node0(url: String) {
    let mut socket = Socket::new(Protocol::Pull).unwrap();
    let mut text = String::new();
    let _ = socket.bind(&url); // let _ = means we don't need any return value stored somewere

    loop {
        match socket.read_to_string(&mut text) {
            Ok(_) => println!("NODE0: RECEIVED '{}'", text),
            Err(err) => {
                println!("NODE0: failed '{}'", err);
                break
            }
        }
        text.clear();
    }
}

fn node1(url: String, msg: String) {
    let mut socket = Socket::new(Protocol::Push).unwrap();
    socket.connect(&url).unwrap();

    match socket.write_all(&msg.as_bytes()) {
        Ok(_) => println!("NODE1: SENDING '{}'", &msg),
        Err(err) => {
            println!("NODE1: failed '{}'", err);
        }
    }
}

fn usage() {
    println!("Usage: pipeline node0|node1 <URL> <ARG> ...");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return usage()
    }

    match args[1].as_ref() {
        "node0" => {
            if args.len() > 1 {
                node0(args[2].clone())
            }
        }
        "node1" => {
            if args.len() > 2 {
                node1(args[2].clone(), args[3].clone())
            }
        }
        _ => usage(),
    }
}
