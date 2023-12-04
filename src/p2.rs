use benchmark_protocol::table::{ClientShare, Partyr, Table};
use mysql::prelude::*;
use mysql::*;
use mysql::{prelude::*, Pool, PooledConn};
use serde_json;
use std::io;
use std::io::Read;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
enum Payload {
    Int(usize),
    Table(Table),
}

fn start_p2() {
    let listener = TcpListener::bind("127.0.0.1:8082").unwrap();

    while let Ok((stream, _)) = listener.accept() {
        let mut stream_p4 = TcpStream::connect("127.0.0.1:8084").unwrap();
        let mut stream_client = TcpStream::connect("127.0.0.1:8080").unwrap();

        handle_client(stream, stream_p4, stream_client);
    }
}
fn handle_client(mut stream: TcpStream, mut stream_p4: TcpStream, mut stream_client: TcpStream) {
    let url = "mysql://root:123456789@localhost:3306/benchdb";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
    // let mut tcp_stream = TcpStream::connect("127.0.0.1:8084").unwrap();
    let buffer_size = 1;
    // let mut buffer = vec![0u8; buffer_size];
    // let mut stream = &stream;
    let cleint_share = 9;
    // stream.read_exact(&mut buffer);
    // let cleint_share = serde_json::from_slice(&buffer);
    // println!("client share:{:?}", cleint_share);
    // let received_data_str = String::from_utf8_lossy(&buffer).trim();
    // print!("{:?}", &buffer);
    let reader = BufReader::new(stream);

    // Deserialize the received JSON into MyData struct
    // let received_data: ClientShare = serde_json::from_str(received_data_str);

    for line in reader.lines() {
        match line {
            Ok(json_str) => {
                let mut my_table: Table =
                    serde_json::from_str(&json_str).expect("Failed to deserialize JSON");

                let (s2, r2) = process_table(
                    &mut conn,
                    &mut my_table.rows,
                    &my_table.row_id,
                    cleint_share,
                );

                let res_to_p4: Partyr = Partyr {
                    row_id: (my_table.row_id),
                    comput: (r2),
                };
                let res_to_client: Partyr = Partyr {
                    row_id: (my_table.row_id),
                    comput: (s2),
                };
                // println!("{:?}", res_to_client);
                send_result_to_parties(&stream_client, &res_to_client);

                // process(my_struct);
                send_result_to_parties(&stream_p4, &res_to_p4);
            }

            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
}

pub fn process_table(
    conn: &mut PooledConn,
    truth_table: &mut Vec<Vec<i8>>,
    row_id: &usize,
    shared: i32,
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
fn send_result_to_parties(mut stream: &TcpStream, data: &Partyr) {
    let bytes = serde_json::to_string(&data).unwrap();

    stream
        .write_all(&bytes.as_bytes())
        .expect("Failed to write table to stream");
    stream.write_all(b"\n").expect("Failed to write to stream");

    // stream
    //     .shutdown(Shutdown::Both)
    //     .expect("shutdown call failed");
}
fn main() {
    start_p2();
}
