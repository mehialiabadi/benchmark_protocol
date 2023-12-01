use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use std::option;
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use mysql::prelude::*;
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::Signed;
use std::net::TcpListener;
use std::thread;
use byteorder::{LittleEndian, ReadBytesExt};

// pub struct LineItem {
//     id:i32,
//     column_value: i32,
  
// }
pub fn p3_process(pool:&Pool,truth_table: &mut Vec<Vec<u8>>,row_id:i32)->(i8,i8){
    let column_name="order_key";

    let mut conn = pool.get_conn().unwrap();
    let query="select id, order_key from p1test_share where id=id";
    // let qu1: String="SELECT  id ,".to_owned()+&column_name;
// let mut  qu2:&str=qu1.to_owned().as_str();
// let mut qu3= ("from p1test_share where id=".to_owned()+&row_id.to_string()).to_owned().as_str();
// let mut query=qu2.to_owned()+qu3;
    // let query = qu2+"from p1test_share where id=".to_owned()+&row_id.to_string();
    // let query = qu+"from p1test_share where";

    let result: Option<(i32, i32)> = conn.exec_first(query, params! { "id" => row_id })
    .map(|row| row.map(|(id, column_value)| (id, column_value)))
    .expect("Failed to execute query");


    if let Some((id, column_value)) = result
     {
        println!("ID: {}, Name: {}", id, column_value);
        // let bin=format!("l as binary is: {column_value:#032b}");
        let bin: String = format!("{:08b}", column_value);

        let (s2,r2)= p3_computaion(truth_table,&bin);
        return  (s2,r2);
    } else {
        println!("No matching row found");
        return (0,0);
    }
  }
     

     fn p3_computaion(truth_table: &mut Vec<Vec<u8>>, binary_p2number: &str) -> (i8,i8){
        #![feature(int_roundings)]
    
    let mut capital_s2:i32=0;
        let mut small_s2: i8;
        let mut r2=0;
    
    let modulue=32;
                for (index, character) in binary_p2number.chars().enumerate() {
                    if character == '0' {
                
                    capital_s2+=truth_table[1][index] as i32;
        
                    
                    }
        
                    else{
                  
                    capital_s2+=truth_table[0][index] as i32;
                }
            }
            small_s2=(capital_s2/8) as i8;
            r2=capital_s2 % 8;
                return  ((small_s2 as f32).floor() as i8,r2 as i8);
        }
fn send_result_to_parties(mut stream_p: &TcpStream, result: i8,row_id:i32) {
    let serialized_result = bincode::serialize(&result).unwrap();
    stream_p.write_all(&serialized_result).unwrap();
    stream_p.write_all(&row_id.to_be_bytes()).unwrap();

}
fn start_p3(server_address: &str) {
    let listener = TcpListener::bind(server_address).expect("Failed to bind");

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            // Spawn a new thread to handle each client
            thread::spawn(move || handle_client(stream));
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 4]; // Adjust buffer size as needed
    let mut buffer_table = [0; 4 * 32 * 2]; // 4 bytes for each element in a 2x8 table

    // match stream.read(&mut buffer) {
    //     Ok(bytes_read) => {
    //         let received_data = &buffer[..bytes_read];
    //         println!("Server 2 received: {:?}", received_data);
    stream.read_exact(&mut buffer).unwrap();

            let integer_value= i32::from_be_bytes(buffer);
            println!("p3 received from client: {:?}", integer_value);

            stream.read_exact(&mut buffer_table).unwrap();
            let  mut received_table: Vec<Vec<i32>> = bincode::deserialize(&buffer_table).unwrap();
            println!("table p3:{:?}",received_table);
            // // Simulate processing the data (you can replace this with your actual logic)
            // let processed_data = process_data(received_data);

            // // Send a response back to the client
            // if let Err(err) = stream.write_all(&processed_data) {
            //     eprintln!("Error sending response: {}", err);
            // }
        }
        // Err(err) => {
        //     eprintln!("Error reading from client: {}", err);
        // }
//     }
// }
fn main() {
    let url = "mysql://root:123456789@localhost:3306/testdb";
    let pool = Pool::new(url).unwrap();
    // let table_name="line_item_1m";
    let table_name="p3share_test";

    let column_name="order_key";
    let distance=8;

    let client_address = "127.0.0.1:8080"; // Assuming p2 is listening on port 8082

    let p3_address = "127.0.0.1:8083"; // Assuming p2 is listening on port 8082
    let p4_address = "127.0.0.1:8084"; // Assuming p4 is listening on port 8084

    // let listener_p3 = TcpListener::bind(p3_address).unwrap();
    // let  stream_p4 = TcpStream::connect(p4_address).unwrap();
    // let mut stream_client = TcpStream::connect(client_address).unwrap();

    // let mut stream_p4_clone = stream_p4.try_clone().unwrap();
    
    start_p3(p3_address);
    // for stream in listener_p3.incoming() {
    //     match stream {
    //         Ok(mut stream) => {
    //             let mut buffer: [u8; 4] = [0; 4];
    //             stream.read_exact(&mut buffer).unwrap();
    //             let user_number = i32::from_be_bytes(buffer);
    //             println!("p3 receives share from client for keyword to seach {:?}",user_number);


    //             // let mut buffer: [u8; 256] = [0; 4 * 32 * 2]; // 4 bytes for each element in a 2x8 table
    //             // stream.read_exact(&mut buffer).unwrap();
    //             // let  mut received_table: Vec<Vec<i32>> = bincode::deserialize(&buffer).unwrap();
    //             // let row_id:i32 = bincode::deserialize(&buffer).unwrap();
    //             // println!("p3 received truth table from p1 for keyword to seach for row id:{:?} {:?}",received_table,row_id);

    //             // // Process the received table
    //             // let (s3,r3) = p3_process(&pool,&mut received_table,row_id);
    //             // println!("p3 computes s3:{:?} , r3:{:?} ",s3,r3);

    //             // // Send the result to p4
    //             // send_result_to_parties(&stream_p4_clone, r3,row_id);
    //             // send_result_to_parties(&stream_client, s3,row_id);

    //         }
    //         Err(e) => {
    //             eprintln!("Error: {}", e);
    //         }
    //     }
    // }
}
     
