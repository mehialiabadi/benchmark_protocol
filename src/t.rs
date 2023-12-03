use benchmark_protocol::table::{PartyCollection, Partyr, Table};
use mysql::prelude::*;
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use serde::de::value::UsizeDeserializer;
use serde::{Deserialize, Serialize};
use serde_json::Deserializer;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::{option, thread};
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::{iter, Signed};
use std::net::TcpListener;

#[derive(Debug)]
enum Message {
    Data(String),
    NoData,
}

fn handle_p2(stream: TcpStream, sender: Sender<Message>) {
    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            sender.send(Message::Data(buffer)).unwrap();
        } else {
            sender.send(Message::NoData).unwrap();
        }
    } else {
        sender.send(Message::NoData).unwrap();
    }
}
fn handle_p3(stream: TcpStream, sender: Sender<Message>) {
    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            sender.send(Message::Data(buffer)).unwrap();
        } else {
            sender.send(Message::NoData).unwrap();
        }
    } else {
        sender.send(Message::NoData).unwrap();
    }
}

impl Message {
    fn to_string(&self) -> String {
        match self {
            Message::Data(text) => text.clone(),
            Message::NoData => todo!(),
            // Handle other variants if needed
        }
    }
}
impl Message {
    fn last_char(&self) -> Option<char> {
        match self {
            Message::Data(data) => data.chars().last(),
            Message::NoData => todo!(),
            // Handle other variants if needed
        }
    }
}
fn handle_client(stream: TcpStream, sender: Sender<Message>) {
    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            sender.send(Message::Data(buffer)).unwrap();
        } else {
            sender.send(Message::NoData).unwrap();
        }
    } else {
        sender.send(Message::NoData).unwrap();
    }
}

fn p4_prepare(data: &Message) -> Option<Partyr> {
    match data {
        Message::Data(data) => {
            let r: Result<Partyr, serde_json::Error> = serde_json::from_str(data);
            match r {
                Ok(party) => return Some(party),
                _ => None,
            }
        }
        Message::NoData => None,
    }
}

fn process_data(parrty1: Option<Partyr>) -> i8 {
    return 1;
}
fn start_server(server_address: &str) {
    let url = "mysql://root:123456789@localhost:3306/testdb";
    let pool = Pool::new(url).unwrap();
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    let (sender1, receiver1) = channel();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Clone the stream for each thread
            let sender1_clone = sender1.clone();

            let p2_stream = stream.try_clone().unwrap();

            //             // Spawn separate threads for P2 and P3
            let r2_handle = thread::spawn(move || handle_p2(p2_stream, sender1_clone));
            let mut data_rec = receiver1.recv().expect("Failed handle1 thread 1");

            let option_party: Partyr = p4_prepare(&data_rec).unwrap();

            let shared_data = Arc::new(Mutex::new(data_rec));
            let shared_data_clone = shared_data.clone();

            let t1 = thread::spawn(move || {
                let mut data_iter = shared_data.lock().unwrap();

                let party1: Option<Partyr> = p4_prepare(&data_iter);
            });
        }
    }
}

fn main() {
    let sever_address = "127.0.0.1:8084"; // Assuming p4 is listening on port 8084

    start_server(sever_address);
}
