use mysql::Pool;
mod generate_data;
use generate_data::gen_data::{generate_line_item, store_data};
fn main() {
    let url = "mysql://root:123456789@localhost:3306/benchdb";
    let pool = Pool::new(url).unwrap();

    //    Step1: Generate and store 1M rows
    let line_items_1m = generate_line_item(1_000_000);
    store_data(&pool, "line_item_1m_testp1", &line_items_1m);
    let line_items_1m = generate_line_item(1_000_000);
    store_data(&pool, "line_item_1m_testp2", &line_items_1m);
    let line_items_1m = generate_line_item(1_000_000);

    store_data(&pool, "line_item_1m_testp3", &line_items_1m);
}
