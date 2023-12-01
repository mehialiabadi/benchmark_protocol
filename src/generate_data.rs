

pub mod gen_data{
    use mysql::{prelude::Queryable, Pool};
use rand::Rng;
    // Define the size of the table (rows and columns)

  

    

    // let binary_representation:String=String::from("00001010");

    // Generate a random table
    // let rows = 2;
    // let columns = 8;
    // let decimal_number:usize=34;

    // let bin_a = String::from(format!("{decimal_number:b}"));
    // let mut table = generate_random_table(rows, columns);
    // print!("Base table:{:?}",table);

    // let share_p3: Vec<Vec<i64>>  = generate_truth_table(&mut table,&bin_a);

    // println!("{:?}",share_p3);


    pub struct LineItem {
        order_key: i64,
        part_key: i64,
        line_number: i64,
        supp_key: i64,
    }
    
    
    pub fn generate_line_item(num_rows: usize) -> Vec<LineItem> {
        let mut rng = rand::thread_rng();
        (0..num_rows)
            .map(|_| LineItem {
                order_key: rng.gen_range(1..=1000000),
                part_key: rng.gen_range(1..=500000),
                line_number: rng.gen_range(1..=100),
                supp_key: rng.gen_range(1..=200000),
            })
            .collect()
    }
    
    pub fn store_data(pool: &Pool, table_name: &str, data: &[LineItem]) {
        let mut conn = pool.get_conn().expect("Failed to get MySQL connection");
        conn.query_drop(format!("TRUNCATE TABLE {}", table_name)).expect("Failed to truncate table");
    
        for item in data {
            conn.exec_drop(
                format!(
                    "INSERT INTO {} (order_key, part_key, line_number, supp_key) VALUES (?, ?, ?, ?)",
                    table_name
                ),
                (item.order_key, item.part_key, item.line_number, item.supp_key),
            )
            .expect("Failed to insert data");
        }
    }
}








// fn print_table(table: &Vec<Vec<i64>>) {
//     for row in table {
//         for &cell in row {
//             print!("{:4} ", cell); // Adjust the width as needed
//         }
//         println!();
//     }
// }
