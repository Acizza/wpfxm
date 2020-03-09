use std::collections::HashMap;

#[derive(Debug)]
pub struct Application {
    pub id: i32,
    pub name: String,
    pub prefix: String,
    pub env: HashMap<String, String>,
}
