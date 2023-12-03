use benchmark_protocol::table::Table;
use mysql::{prelude::*, Pool, PooledConn};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io::{self, Error, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::time::Instant;
use std::{result, thread};

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

fn raw_value(mut conn: PooledConn, column_name: &str) -> Result<Vec<LineItem>, mysql::Error> {
    // let query = "SELECT id, order_key from test1";
    let query = "SELECT id, order_key from line_item_1m_testp1";

    let mut stmt = conn.query_map(query, |(id, column_value)| LineItem { id, column_value });

    return stmt;
}

fn send_shared_truthtable_to_parties(addt: &str, table: &Table) -> io::Result<()> {
    // let mut stream1 = stream.lock().unwrap();
    let mut stream = TcpStream::connect(addt).unwrap();
    let bytes = serde_json::to_string(&table).unwrap();

    stream
        .write_all(&bytes.as_bytes())
        .expect("Failed to write table to stream");
    drop(stream);

    Ok(())
}
// fn send_shared_truthtable_to_parties(
//     mut stream: Arc<Mutex<TcpStream>>,
//     table: &Table,
// ) -> io::Result<()> {
//     let mut stream1 = stream.lock().unwrap();

//     let bytes = serde_json::to_string(&table).unwrap();

//     stream1
//         .write_all(&bytes.as_bytes())
//         .expect("Failed to write table to stream");
//     drop(stream1);

//     Ok(())
// }

fn start_p1(server_address: &str) {
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            thread::spawn(move || handle_client_connection(stream));
        }
    }
}

fn handle_client_connection(mut stream: TcpStream) {
    let distance = 8;
    let column_name = "order_key";

    let url = "mysql://root:123456789@localhost:3306/benchdb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let p2_address = "127.0.0.1:8082";
    // let connect_p2 = Arc::new(Mutex::new(
    //     TcpStream::connect_timeout(&p2_address, Duration::from_secs(5)).unwrap(),
    // ));

    let mut buffer = String::new();
    if let Ok(bytes_read) = stream.take(1024).read_to_string(&mut buffer) {
        if bytes_read > 0 {
            let client_share = &buffer[..bytes_read];
            let start_time = Instant::now();

            let client_i8: i32 = convert_str_int(client_share).into();

            let smnt = raw_value(conn, column_name); //

            for row in smnt.iter().flatten() {
                let new_value = row.column_value.wrapping_sub(client_i8.abs());

                let (tab_p2, tab_p3) = generate_truth_table(new_value.try_into().unwrap(), 8);
                let table_p2: Table = Table {
                    row_id: row.id as usize,
                    rows: tab_p2,
                };
                let table_p3: Table = Table {
                    row_id: row.id as usize,
                    rows: tab_p3,
                };
                // println!("Table2:{:?}---Table3:{:?}", table_p2, table_p3);
                send_shared_truthtable_to_parties(p2_address, &table_p2);
                // send_shared_truthtable_to_parties(connect_p3.clone(), &table_p3);
            }
            let elapsed_time = start_time.elapsed();
            println!("P1 generating truth tables: {:?}", elapsed_time);
        }
    }
}

fn main() {
    let p1_address = "127.0.0.1:8081";

    start_p1(p1_address);
}
