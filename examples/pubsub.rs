/// ### Build this example
/// ```
/// cargo build --example pubsub
/// ```
/// ### Run this example
/// ```
/// cargo run --example pubsub -- server ipc:///tmp/pubsub.ipc & sleep 2 &
/// cargo run --example pubsub -- client ipc:///tmp/pubsub.ipc  client0 &
/// cargo run --example pubsub -- client ipc:///tmp/pubsub.ipc  client1 &
/// cargo run --example pubsub -- client ipc:///tmp/pubsub.ipc  client2 &
/// sleep 5
/// killall pubsub
/// ```
/// ### Expected output
/// ```
/// SERVER: PUBLISHING DATE Tue May 31 13:00:43 2016
/// CLIENT (client0): RECEIVED Tue May 31 13:00:43 2016
/// CLIENT (client1): RECEIVED Tue May 31 13:00:43 2016
/// CLIENT (client2): RECEIVED Tue May 31 13:00:43 2016
/// SERVER: PUBLISHING DATE Tue May 31 13:00:44 2016
/// CLIENT (client1): RECEIVED Tue May 31 13:00:44 2016
/// CLIENT (client2): RECEIVED Tue May 31 13:00:44 2016
/// CLIENT (client0): RECEIVED Tue May 31 13:00:44 2016
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

fn server(url: String) {
    let mut socket = Socket::new(Protocol::Pub).unwrap();
    let _ = socket.bind(&url).unwrap();

    loop {
        let date = date();

        match socket.write_all(&date.as_bytes()) {
            Ok(_) => {
                println!("SERVER: PUBLISHING DATE {}", date);
                thread::sleep(Duration::from_millis(100));
            },
            Err(err) => panic!("{}", err),
        }
    }
}

fn client(url: String, name: String) {
    let mut socket = Socket::new(Protocol::Sub).unwrap();
    let topic = "";
    match socket.subscribe(topic) {
            Ok(_) => {},
            Err(err) => panic!("{}", err)
    }
    let _ = socket.connect(&url).unwrap();

    let mut buffer = String::new();

    loop {
        match socket.read_to_string(&mut buffer) {
            Ok(_) => {
                println!("CLIENT ({}): RECEIVED {}", name, buffer);
                buffer.clear();
            },
            Err(err) => { panic!("{}", err); }
        }
    }
}

fn usage() {
    println!("Usage: pubsub server|client <URL> <ARG> ...");
}

fn main() {
    let args: Vec<_> = std::env::args().collect();

    if args.len() < 2 {
        return usage()
    }

    match args[1].as_ref() {
        "server" => server(args[2].clone()),
        "client" => client(args[2].clone(), args[3].clone()),
        _ => usage(),
    }
}
