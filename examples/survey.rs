/// ### Build this example
/// ```
/// cargo build --example survey
/// ```
/// ### Run this example
/// ```
/// cargo run --example survey -- server ipc:///tmp/survey.ipc &
/// cargo run --example survey -- client ipc:///tmp/survey.ipc  client0 &
/// cargo run --example survey -- client ipc:///tmp/survey.ipc  client1 &
/// cargo run --example survey -- client ipc:///tmp/survey.ipc  client2 &
/// sleep 3
/// killall -q cargo survey &>/dev/null
/// ```
/// ### Expected output (maybe in different order)
/// ```
/// SERVER: SENDING DATE SURVEY REQUEST
/// CLIENT (client0): RECEIVED "DATE" SURVEY REQUEST
/// CLIENT (client1): RECEIVED "DATE" SURVEY REQUEST
/// CLIENT (client2): RECEIVED "DATE" SURVEY REQUEST
/// CLIENT (client1): SENDING DATE SURVEY RESPONSE
/// CLIENT (client0): SENDING DATE SURVEY RESPONSE
/// CLIENT (client2): SENDING DATE SURVEY RESPONSE
/// SERVER: RECEIVED "Tue May 31 17:32:35 2016" SURVEY RESPONSE
/// SERVER: RECEIVED "Tue May 31 17:32:35 2016" SURVEY RESPONSE
/// SERVER: RECEIVED "Tue May 31 17:32:35 2016" SURVEY RESPONSE
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
    let mut socket = Socket::new(Protocol::Surveyor).unwrap();
    let _ = socket.bind(&url).unwrap();
    // Wait for connections
    thread::sleep(Duration::from_millis(1000));
    match socket.write_all("DATE".as_bytes()) {
        Ok(_) => {
            println!("SERVER: SENDING DATE SURVEY REQUEST");
        },
        Err(err) => { panic!("{}", err); },
    }


    loop {
        let mut buffer = String::new();

        match socket.read_to_string(&mut buffer) {
            Ok(_) => {
                println!("SERVER: RECEIVED \"{}\" SURVEY RESPONSE", buffer);
                buffer.clear();
            },
            Err(_) => { break; }
        }
    }
}

fn client(url: String, name: String) {
    let mut socket = Socket::new(Protocol::Respondent).unwrap();
    let _ = socket.connect(&url).unwrap();

    loop {
        let mut buffer = String::new();

        match socket.read_to_string(&mut buffer) {
            Ok(_) => {
                println!("CLIENT ({}): RECEIVED \"{}\" SURVEY REQUEST", name, buffer);
                buffer.clear();
                let date = date();
                match socket.write_all(&date.as_bytes()) {
                    Ok(_) => { println!("CLIENT ({}): SENDING DATE SURVEY RESPONSE", name); },
                    Err(err) => {
                        println!("CLIENT ({}) could not send date survey response: {}", name, err);
                    }
                }
            },
            Err(_) => { break; }
        }
    }
}

fn usage() {
    println!("Usage: survey server|client <URL> <ARG> ...");
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
