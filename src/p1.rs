use std::hash::BuildHasher;
use std::net::{TcpStream, SocketAddr, TcpListener};
use std::io::{Read, Write, BufWriter};
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use mysql::prelude::*;
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::Signed;
use serde::{Serialize, Deserialize};
// suse std::net::TcpListener;
use std::io::{self};
use std::thread;
use byteorder::{LittleEndian, ReadBytesExt};
use bincode::serialize;





pub struct LineItem {
    id:i32,
    column_value: i32,
}
#[derive(Serialize, Deserialize, Debug,Clone)]
struct Table {
    rows: Vec<Vec<u8>>,
}

fn generate_random_table(distance:i8) -> Table {
    let mut rng = rand::thread_rng();
    let rows: Vec<Vec<u8>> = (0..2)
    .map(|_| (0..8).map(|_| rng.gen_range(0..100)).collect())
    .collect();

Table { rows }

}


pub fn generate_truth_table( number:i32,distance:i8) -> (Table,Table){
    
    let binary_number:&str=&format!("{number:08b}");
    println!("binary:{:?}",binary_number);

   let mut p2_table= generate_random_table(distance);

   let mut p3_table = p2_table.clone();
    let my_variable: u8 = 1;

    for (index1, bit) in binary_number.chars().enumerate() {
        if bit == '0' {
            let row = p3_table.rows.get_mut(1);
           
            if let Some(element) = row.expect("REASON").get_mut(index1) {
                *element = *element - my_variable;
            }
        }
        if bit=='1'{
        let row = p3_table.rows.get_mut(0);
        if let Some(element) = row.expect("REASON").get_mut(index1) {
            *element = *element - my_variable;
        }
            }
        }
   return  (p2_table, Table { rows: p3_table.rows.to_vec() });
}
fn raw_value(pool:&Pool,user_number:i32,column_name:&str)->Result<Vec<LineItem>, mysql::Error>{
    let mut conn = pool.get_conn().unwrap();


    //connect to database and then subtract the value
//     let qu:String="SELECT  id ,".to_owned()+&column_name;
//   let query = qu+"from p1test_share";
  let query="SELECT id, order_key from p1test_share";
     let mut stmt = conn.query_map(query,|(id, column_value)| { LineItem { id, column_value} });
   
  return  stmt;


}
fn process_row(id:i32, order_key:i32, user_number:i32)->(i32,i32){

    return (id, (order_key- user_number.abs()));
    
}

fn send_shared__truthtable_to_parties(address:&str, table: Table,id:i32) {

        let mut stream = TcpStream::connect(address).unwrap();
        stream.write_all(b"p1").expect("Failed to send server identifier");

    let serialized_table = serialize(&table).expect("Failed to serialize table");

    stream.write_all(&serialized_table).expect("Failed to write to stream");
    // writer.flush().expect("Failed to flush stream");



  
}

fn start_p1(server_address: &str) {
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
    let url = "mysql://root:123456789@localhost:3306/testdb";
    let pool = Pool::new(url).unwrap();

let  distance=8;
let column_name="order_key";
let  p2_address="127.0.0.1:9092";
 let p3_address="127.0.0.1:8083";

 let mut server_identifier = [0; 2];
 stream.read_exact(&mut server_identifier).expect("Failed to read from client identifier");
  let server_identifier:&str= &String::from_utf8_lossy(&server_identifier).to_string();
  match &server_identifier {
    &"c1" => {
        let mut buffer_integer = [0; 4 ];// 4 bytes for 32 bits
        stream.read_exact(&mut buffer_integer).expect("Failed to read table data");
        let user_number= i32::from_be_bytes(buffer);
        let  smt=raw_value(&pool,user_number,column_name);

        for row in smt.iter().flatten() {
         let new_value=(row.column_value-user_number).abs();
         let (p2_share_table,p3_share_table) = generate_truth_table(new_value,distance);

         println!("p2 truth table {:?} p3share truth table:{:?} for row id:{:?}, and new value {:?}",p2_share_table,p3_share_table,row.id,new_value);
         send_shared__truthtable_to_parties(p2_address,p2_share_table,row.id);
         send_shared__truthtable_to_parties(p3_address,p3_share_table,row.id);

        }

    }
    _ => {
        println!("Unexpected server identifier expect p1: {}", server_identifier);
        // Handle the unexpected identifier
    }
}

}
fn main()  {

    // let url = "mysql://root:123456789@localhost:3306/testdb";
    // let pool = Pool::new(url).unwrap();


    let table_name="p1test_share";
    // let table_name="line_item_1m";

    // let column_name: &str="order_key";
    // let distance: i32=8;
    let p1_address = "127.0.0.1:8081";
  

    start_p1(p1_address);

   
}




