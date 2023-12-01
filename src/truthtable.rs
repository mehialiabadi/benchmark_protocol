use std::net::{TcpStream, SocketAddr};
use std::io::{Read, Write};
use rand::Rng;
use rusqlite::{Connection, Result, NO_PARAMS};
use mysql::prelude::*;
// use mysql::{OptsBuilder, Pool};
use mysql::*;
use num::Signed;
use std::net::TcpListener;
use std::io::{self};

fn main() {

     let (p2,p3)=generate_truth_table(8,8);
 println!("p2:{:?}, p3:{:?}",p2,p3);  
 let mut rand_tabl= generate_random_table(8);
 let (s2,r2)=p2_computaion(&mut rand_tabl,"00000010");
 println!("s2:{:?}.r2:{:?}",s2,r2);
}
fn generate_random_table(distance:i32) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    (0..2).map(|_| (0..distance).map(|_| rng.gen_range(1..10)).collect()).collect()
}

pub fn generate_truth_table( number:i32,distance:i32) -> (Vec<Vec<u8>> ,Vec<Vec<u8>>){
    
    let binary_number:&str=&format!("{number:08b}");
    println!("binary:{:?}",binary_number);

   let mut p2_table= generate_random_table(distance);
//    println!("random:{:?}",p2_table);

   let mut p3_table = p2_table.clone();
// println!("random table:{:?}",p2_table);
    let my_variable: u8 = 1;

    for (index1, bit) in binary_number.chars().enumerate() {
        if bit == '0' {
            let row = p3_table.get_mut(1);
            // println!("row:{:?}",row);
            // println!("index:{:?}",index1);
            if let Some(element) = row.expect("REASON").get_mut(index1) {
                // println!("elemnt:{:?} at index :{:?}",element,index1 );
                *element = *element - my_variable;
            }
        }
        if bit=='1'{
        let row = p3_table.get_mut(0);
        if let Some(element) = row.expect("REASON").get_mut(index1) {
            *element = *element - my_variable;
        }
            }
        }
   return  (p2_table, p3_table.to_vec());
}


fn p2_computaion(truth_table: &mut Vec<Vec<u8>>, binary_p2number: &str) -> (i8,i8){
    #![feature(int_roundings)]
println!("basic table:{:?}",truth_table);
let mut capital_s2:i32=0;
    let mut small_s2: i8=0;
    let mut r2=0;

            for (index, character) in binary_p2number.chars().enumerate() {
                if character == '0' {
            println!("value:{:?}",truth_table[1][index] );
                capital_s2+=truth_table[1][index] as i32;
    
                
                }
    
                else{
                    println!("value:{:?}",truth_table[0][index] );

                capital_s2+=truth_table[0][index] as i32;
            }
        }
        println!("capital S:{:?}",capital_s2);
        small_s2=(capital_s2/8) as i8;
        r2=capital_s2 % 8;
            return  ((small_s2 as f32).floor() as i8,r2 as i8);
    }