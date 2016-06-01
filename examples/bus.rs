/// ### Build this example
/// ```
/// cargo build --example bus
/// ```
/// ### Run this example
/// ```
/// cargo run --example bus -- node0 ipc:///tmp/node0.ipc ipc:///tmp/node1.ipc ipc:///tmp/node2.ipc &
/// cargo run --example bus -- node1 ipc:///tmp/node1.ipc ipc:///tmp/node2.ipc ipc:///tmp/node3.ipc &
/// cargo run --example bus -- node2 ipc:///tmp/node2.ipc ipc:///tmp/node3.ipc &
/// cargo run --example bus -- node3 ipc:///tmp/node3.ipc ipc:///tmp/node0.ipc &
/// sleep 5
/// killall -q cargo bus &>/dev/null
/// ```
/// ### Expected output (maybe in different order)
/// ```
/// node3: SENDING 'node3' ONTO BUS
/// node2: SENDING 'node2' ONTO BUS
/// node2: RECEIVED 'node3' FROM BUS
/// node3: RECEIVED 'node2' FROM BUS
/// node1: SENDING 'node1' ONTO BUS
/// node2: RECEIVED 'node1' FROM BUS
/// node3: RECEIVED 'node1' FROM BUS
/// node1: RECEIVED 'node3' FROM BUS
/// node1: RECEIVED 'node2' FROM BUS
/// node0: SENDING 'node0' ONTO BUS
/// node1: RECEIVED 'node0' FROM BUS
/// node0: RECEIVED 'node3' FROM BUS
/// node0: RECEIVED 'node2' FROM BUS
/// node2: RECEIVED 'node0' FROM BUS
/// node3: RECEIVED 'node0' FROM BUS
/// node0: RECEIVED 'node1' FROM BUS
/// ```
extern crate nanomsg;
extern crate time;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};
use std::thread;
use std::time::{Duration};


fn node(args: Vec<String>) {
    let mut socket = Socket::new(Protocol::Bus).unwrap();
    let _ = socket.bind(&args[2]).unwrap();

    let mut iter = args.iter().skip(3);

    loop {
        match iter.next() {
            Some(url) => {
                match socket.connect(&url) {
                    Ok(_) => {}
                    Err(err) => { panic!("{}", err); }
                }
            }
            None => break,
        }
    }
    thread::sleep(Duration::from_millis(1000));
    let _ = socket.set_receive_timeout(100);

    match socket.write_all(&args[1].as_bytes()) {
        Ok(_) => {
            println!("{}: SENDING '{}' ONTO BUS", &args[1], &args[1]);
        }
        Err(err) => { panic!("{}", err); }
    }

    loop {
        let mut buffer = String::new();

        match socket.read_to_string(&mut buffer) {
            Ok(_) => {
                println!("{}: RECEIVED '{}' FROM BUS", args[1], buffer);
                buffer.clear();
            }
            Err(_) => { }
        }
    }
}


fn usage() {
    println!("Usage: bus <NODE_NAME> <URL> <URL> ...");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() >= 3 {
        node(args);
    } else {
        return usage()
    }
}
