#![allow(non_snake_case)]
use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::thread;
use std::str;

fn main() {
    //define ip address
    let addr = "127.0.0.1:8866".to_string();
    //define tcp listener
    let listener = TcpListener::bind(&addr).unwrap();
    //get client msg, return Result
    for stream in listener.incoming() {

        println!("DEBUG::new connection");
        //check type of input
        match stream {
            //match input with OK
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream);
                });
            }
            //use panic output error
            Err(e) => {
                panic!("Ops, error {:?}", e)
            }
        }
    }
    //close TCP listening
    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    println!("DEBUG::deal with client msg");

    let mut buf = [0;512];

    loop {
        //read msg in bytes
        let bytes_read = stream.read(&mut buf).expect("reading fail, break");
        println!("DEBUG::byte size {}", bytes_read);
        //if no bytes readed, break loop
        if bytes_read == 0 {
            break;
        }

        //error detecting in rust, encoding check
        let s = match str::from_utf8(&buf[..bytes_read]) {
            Ok(v) => v,
            Err(_e) => {
                stream.write(b"Need utf-8 sequence").unwrap();
                continue;
            },
        };
        //echo
        //detect if "bye", close connection
        if s.len() >= 3 && s[0..3] == "bye".to_string() {
            stream.write(b"Server: Bye Bye.\n").unwrap();
            break;
        }
        //detect if "hello, return special greeding
        if s.len() >= 5 && s[0..5] == "hello".to_string() {
            stream.write(b"Server: hello, nice to meet you.\n").unwrap();
        }

        //return input as same, unwrap: ignore error, break
        println!("DEBUG::byte size {}", bytes_read);
        stream.write(&buf[..bytes_read]).unwrap();

    }
}