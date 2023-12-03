pub mod additive_share {

    use rand::Rng;
    use rusqlite::{Connection, Result, NO_PARAMS};

    use mysql::prelude::*;
    // use mysql::{OptsBuilder, Pool};
    use mysql::*;

    // fn process_row (row: &rusqlite::Row) -> (i64, i64, i64, i64, i64, i64,i64,i64,i64) {
    fn process_row(
        id: i64,
        order_key: i64,
        part_key: i64,
        line_number: i64,
        supp_key: i64,
    ) -> (i64, i64, i64, i64, i64, i64, i64, i64, i64) {
        //   let id: i64 = row.get(0).unwrap_or_default();
        // let order_key: i64 = row.get(1).unwrap_or_default();
        // let part_key: i64 = row.get(2).unwrap_or_default();
        // let line_number: i64 = row.get(3).unwrap_or_default();
        // let supp_key: i64 = row.get(4).unwrap_or_default();

        let (order_key1, order_key2) = share(order_key);
        let (part_key1, part_key2) = share(part_key);
        let (line_number1, line_number2) = share(line_number);
        let (supp_key1, supp_key2) = share(supp_key);

        return (
            id,
            order_key1,
            order_key2,
            part_key1,
            part_key2,
            line_number1,
            line_number2,
            supp_key1,
            supp_key2,
        );
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct LineItem {
        id: i32,
        order_key: i32,
        part_key: i32,
        line_number: i32,
        supp_key: i32,
    }

    pub fn share(value: i32) -> (i32, i32) {
        let mut rng = rand::thread_rng();
        let share1 = rng.gen_range(1..=value);
        let share2 = value - share1;
        return (share1, share2);
    }

    pub fn creat_shared_tables(pool: &Pool) -> Result<(), mysql::Error> {
        let mut conn = pool.get_conn().unwrap();

        let query = "SELECT  id, order_key, part_key, line_number, supp_key from line_item_1m";
        let mut stmt = conn.query_map(query, |(id, order_key, part_key, line_number, supp_key)| {
            LineItem {
                id,
                order_key,
                part_key,
                line_number,
                supp_key,
            }
        });

        for row in stmt.iter().flatten() {
            let (
                id,
                order_key1,
                order_key2,
                part_key1,
                part_key2,
                line_number1,
                line_number2,
                supp_key1,
                supp_key2,
            ) = process_row(
                row.id,
                row.order_key,
                row.part_key,
                row.line_number,
                row.supp_key,
            );

            conn.exec_drop(
            "INSERT INTO line_item_1m_share_p1 ( id, order_key_p1, part_key_p1, line_number_p1, supp_key_p1) VALUES ( :id, :order_key_p1, :part_key_p1, :line_number_p1, :supp_key_p1)",
            params! {
                "id" => id,
                "order_key_p1" => order_key1,
                "part_key_p1" => part_key1,
                "line_number_p1" => line_number1,
                "supp_key_p1" => supp_key1,

            },
        );
            conn.exec_drop(
            "INSERT INTO line_item_1m_share_p23 ( id, order_key_p23, part_key_p23, line_number_p23, supp_key_p23) VALUES ( :id, :order_key_p23, :part_key_p23, :line_number_p23, :supp_key_p23)",
            params! {
                "id" => id,
                "order_key_p23" => order_key2,
                "part_key_p23" => part_key2,
                "line_number_p23" => line_number2,
                "supp_key_p23" => supp_key2,

            },
        )?;
        }

        Ok(())
    }
}
