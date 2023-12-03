use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub row_id: usize,
    pub rows: Vec<Vec<i8>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Partyr {
    pub row_id: usize,
    pub comput: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PartyCollection {
    pub parties: [Partyr; 2],
}
// impl P4paylod {

// impl P4paylod {
//     pub fn iter(&self) -> std::slice::Iter<'_, Partyr> {
//         self.parties.iter()
//     }
// }
impl ToString for Partyr {
    fn to_string(&self) -> String {
        format!("{{ row_id: {}, comput: {} }}", self.row_id, self.comput)
    }
}
