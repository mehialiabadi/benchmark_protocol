use bincode::{deserialize, serialize};
use byteorder::{LittleEndian, ReadBytesExt};
use mysql::prelude::*;
use mysql::*;
use num::complex::ComplexFloat;
use num::Signed;
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::sync::mpsc::{channel, Sender};
use std::thread;

use benchmark_protocol::table::{Partyr, Table};

enum Payload {
    Int(usize),
    Table(Table),
}

#[derive(Debug)]
enum Message {
    Data(String),
    NoData,
}
pub struct LineItem {
    id: i32,
    column_value: i32,
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

pub fn process_table(
    conn: &mut PooledConn,
    truth_table: &mut Vec<Vec<i8>>,
    row_id: &usize,
    shared: usize,
) -> (i32, i32) {
    let column_name = "order_key";

    // let query = "select id, order_key from p1test_share where id=:id";
    // let qu1: String="SELECT  id ,".to_owned()+&column_name;
    // let mut  qu2:&str=qu1.to_owned().as_str();
    // let mut qu3= ("from p1test_share where id=".to_owned()+&row_id.to_string()).to_owned().as_str();
    // let mut query=qu2.to_owned()+qu3;
    // let query = qu2+"from p1test_share where id=".to_owned()+&row_id.to_string();
    // let query = qu+"from p1test_share where";

    let query = format!("SELECT id, order_key FROM line_item_1m_testp2  WHERE id = :id",);

    let result: Option<(i32, i32)> = conn
        .exec_first(&query, params! { "id" => row_id })
        .map(|row| row.map(|(id, column_value)| (id, column_value)))
        .expect("Failed to execute query");

    if let Some((id, column_value)) = result {
        // println!("id: {}, value: {}", id, column_value);
        let bin: String = format!(
            "{:08b}",
            (column_value.wrapping_sub(shared.try_into().unwrap())).abs()
        );

        let (s2, r2) = p2_computaion(truth_table, &bin);
        return (s2, r2);
    } else {
        println!("No matching row found");
        return (0, 0);
    }
    fn p2_computaion(truth_table: &mut Vec<Vec<i8>>, binary_p2number: &str) -> (i32, i32) {
        #![feature(int_roundings)]

        let mut capital_s2: i32 = 0;
        let mut small_s2: i32;
        let mut r2 = 0;
        // println!("binary:{:?}", binary_p2number);
        let modulue = 32;
        for (index, character) in binary_p2number.chars().enumerate() {
            if character == '0' {
                capital_s2 += truth_table[1][index] as i32;
                // println!("element:{:?}", truth_table[1][index]);
            } else {
                capital_s2 += truth_table[0][index] as i32;
                // println!("element:{:?}", truth_table[0][index]);
            }
        }
        small_s2 = (capital_s2 / 8) as i32;
        r2 = capital_s2 % 8;
        // println!("capital S2:{:?}", capital_s2);
        return ((small_s2 as f32).floor() as i32, r2 as i32);
    }
}

fn p2_prepare(data: &Message) -> Option<Payload> {
    match data {
        Message::Data(data) => {
            if let Ok(i) = data.parse::<usize>() {
                return Some(Payload::Int(i));
            }

            let r: Result<Table, serde_json::Error> = serde_json::from_str(data);
            match r {
                Ok(table) => Some(Payload::Table(table)),
                _ => None,
            }
        }
        Message::NoData => None,
    }
}

fn send_result_to_parties(server_addr: &str, data: &Partyr) {
    let mut stream = TcpStream::connect(server_addr).unwrap();
    let bytes = serde_json::to_string(&data).unwrap();

    stream
        .write_all(&bytes.as_bytes())
        .expect("Failed to write table to stream");
}

fn start_p2(server_address: &str) {
    let url = "mysql://root:123456789@localhost:3306/benchdb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    let (sender1, receiver1) = channel();

    let p4_address = "127.0.0.1:8084"; // Assuming p4 is listening on port 8084
    let mut cleint_share = 0;
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            let sender1_clone = sender1.clone();

            // Spawn threads to handle each server connection
            let handle1 = thread::spawn(|| handle_client(stream, sender1_clone));
            let mut data_rec = receiver1.recv().expect("Failed handle1 thread 1");

            if let Some(res) = p2_prepare(&data_rec) {
                match res {
                    Payload::Int(i) => {
                        cleint_share = i;
                    }
                    Payload::Table(mut t) => {
                        let (s2, r2) =
                            process_table(&mut conn, &mut t.rows, &t.row_id, cleint_share);

                        let res_to_p4: Partyr = Partyr {
                            row_id: (t.row_id),
                            comput: (r2),
                        };
                        let res_to_client: Partyr = Partyr {
                            row_id: (t.row_id),
                            comput: (s2),
                        };

                        // send_result_to_parties(client_address, &res_to_client.to_string());
                        send_result_to_parties(p4_address, &res_to_p4);
                        //
                    }
                }
            }
        }
    }
}

fn main() {
    // let url = "mysql://root@localhost:3306/testdb";
    // let pool = Pool::new(url).unwrap();
    // let table_name="line_item_1m";
    // let table_name = "p2share_test";
    // let column_name = "order_key";
    // let distance = 32;

    let p2_address = "127.0.0.1:8082"; // Assuming p2 is listening on port 8082

    // let mut stream_p4_clone = stream_p4.try_clone().unwrap();
    start_p2(p2_address);
}
