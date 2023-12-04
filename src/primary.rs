use benchmark_protocol::table::ClientShare;
use benchmark_protocol::table::Table;
use mysql::{prelude::*, Pool, PooledConn};
use num::traits::float;
use num::Float;
use num::Signed;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::BufRead;
use std::io::BufReader;
use std::io::{self, Error, Read, Write};
use std::net::TcpStream;
use std::thread::Thread;
// use tokio::io::BufReader;
use std::net::{IpAddr, Ipv4Addr, Shutdown, SocketAddr, TcpListener};
use std::ops::Add;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::time::Instant;
use std::{result, thread};
// use tokio::io::AsyncWriteExt;
// use std::io::{BufRead, BufReader};
// use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream};

// #[derive(Serialize, Deserialize, Debug, Clone)]
// struct Table {
//     rows: [[i8; 8]; 2],
// }
#[derive(Debug)]
enum Message {
    Data(String),
    NoData,
}
pub struct LineItem {
    id: i32,
    column_value: i32,
}

fn generate_random_table(distance: i8) -> Vec<Vec<i8>> {
    let mut rng = rand::thread_rng();
    let random_table = (0..2)
        .map(|_| (0..8).map(|_| rng.gen_range(1..20)).collect())
        .collect();
    return random_table;
}

pub fn convert_str_int(str_value: &str) -> i8 {
    match str_value.parse::<i8>() {
        Ok(parsed_value) => {
            // Successfully parsed the string to i8
            return parsed_value;
        }
        Err(e) => {
            // Failed to parse the string to i8
            eprintln!("Error: {}", e);
            return 0;
        }
    }
}

fn generate_truth_table(num: i8, distance: i8) -> (Vec<Vec<i8>>, Vec<Vec<i8>>) {
    let binary_number: &str = &format!("{num:08b}");

    let mut p2_table = generate_random_table(distance);

    let mut p3_table = p2_table.clone();
    let my_variable: i8 = 1;

    for (index1, bit) in binary_number.chars().enumerate() {
        if bit == '0' {
            let row = p3_table.get_mut(1);

            if let Some(element) = row.expect("REASON").get_mut(index1) {
                *element = *element - my_variable;
                // *element = *element.wrapping_sub(my_variable);
            }
        }
        if bit == '1' {
            let row = p3_table.get_mut(0);
            if let Some(element) = row.expect("REASON").get_mut(index1) {
                *element = *element - my_variable;
            }
        }
    }
    return (p2_table, p3_table);
}

fn raw_value(conn: &mut PooledConn, column_name: &str) -> Result<Vec<LineItem>, mysql::Error> {
    // let query = "SELECT id, order_key from test1";
    let query = "SELECT id, order_key from line_item_1m_testp1";

    let mut stmt = conn.query_map(query, |(id, column_value)| LineItem { id, column_value });

    return stmt;
}

fn send_shared_truthtable_to_parties(
    mut stream: &TcpStream,
    my_vectors: &Vec<Table>,
) -> io::Result<()> {
    // let mut stream1 = stream.lock().unwrap();
    for item in my_vectors.iter() {
        let bytes = serde_json::to_string(&item).unwrap();

        stream.write_all(&bytes.as_bytes()).unwrap();
        stream.write_all(b"\n").unwrap();

        // stream
        //     .shutdown(Shutdown::Both)
        //     .expect("shutdown call failed");
    }
    Ok(())
}

fn start_p1(server_address: &str) -> io::Result<()> {
    // println!("what");
    let listener = TcpListener::bind(server_address).unwrap();

    // while let Ok((stream, _)) = listener.accept() {
    //     // let stream_p2_clone = stream_p2.try_clone().unwrap();

    //     Thread::spawn(handle_client_connection(stream));
    // }

    if let Ok((stream, _)) = listener.accept() {
        handle_client_connection(stream);
    }

    Ok(())
}

//

//

fn handle_client_connection(mut stream: TcpStream) {
    let distance = 8;
    let column_name = "order_key";
    let mut my_tables_p2: Vec<Table> = Vec::new();
    let mut my_tables_p3: Vec<Table> = Vec::new();

    let url = "mysql://root:123456789@localhost:3306/benchdb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    // let p2_stream = Arc::new(Mutex::new(tcp_stream));
    let mut stream_p2 = TcpStream::connect("127.0.0.1:8082").unwrap();
    let mut stream_p3 = TcpStream::connect("127.0.0.1:8083").unwrap();

    // let stream_p2_clone = stream_p2.try_clone().unwrap();
    // let stream_p3_clone = stream_p3.try_clone().unwrap();
    let mut truth_time: f64 = 0.0;
    let mut client_i8 = 0;

    let reader = BufReader::new(&stream);

    if let Some(Ok(line)) = reader.lines().next() {
        println!("Received line: {}", line);

        let mut my_share: ClientShare =
            serde_json::from_str(&line).expect("Failed to deserialize JSON");

        let start_time_database: Instant = Instant::now();

        let client_i8: i32 = my_share.share1;
    }

    let smnt = raw_value(&mut conn, column_name); //

    // let elapsed_time_database = start_time_database.elapsed();
    // println!(
    //     "time to get 1m records from databse-{:?}",
    //     elapsed_time_database
    // );

    let start_time_truth_table: Instant = Instant::now();

    for row in smnt.iter().flatten() {
        let new_value = row.column_value.wrapping_sub(client_i8.abs());

        let start_time_per_truth_table: Instant = Instant::now();

        let (tab_p2, tab_p3) = generate_truth_table(new_value.try_into().unwrap(), 8);
        let elapsed_time_per_truth_table = start_time_per_truth_table.elapsed();
        truth_time += elapsed_time_per_truth_table.as_secs_f64();
        let table_p2: Table = Table {
            row_id: row.id as usize,
            rows: tab_p2,
        };
        let table_p3: Table = Table {
            row_id: row.id as usize,
            rows: tab_p3,
        };

        my_tables_p2.push(table_p2);
        my_tables_p3.push(table_p3);
    }
    let _ = send_shared_truthtable_to_parties(&stream_p2, &my_tables_p2);

    send_shared_truthtable_to_parties(&stream_p3, &my_tables_p3);

    println!("truth table time:{:?}", truth_time);

    let elapsed_time_truth_table = start_time_truth_table.elapsed();

    println!("P1 process+networking: {:?}", elapsed_time_truth_table);
}

fn main() {
    let p1_address = "127.0.0.1:8081";
    start_p1(p1_address);
}
