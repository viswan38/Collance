use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Monetary{
    pub amount: i64,
    pub currency: String,
}

impl Monetary{
    pub fn new(){

    }
}