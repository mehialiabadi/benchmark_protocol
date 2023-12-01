use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub row_id: usize,
    pub rows: Vec<Vec<i8>>,
}
