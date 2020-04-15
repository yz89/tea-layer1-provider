use serde_json::json;
use std::collections::HashMap;
use jsonrpc;
use serde::{Serialize, Deserialize};
use std::vec::Vec;

pub const OP_TPM_INFO: &'static str = "tpm_info";

pub struct Layer1 {
    map: HashMap<i32, i32>
}

impl Layer1 {
    pub fn new() -> Self {
        let mut init_map = HashMap::new();
        Layer1 {
            map: init_map
        }
    }

    pub fn tpm_info(&self, _actor: &str, id: String) -> Result<Node, Box<dyn std::error::Error>> {
        let url ="http://localhost:9933";
        let method = "tea_getNode";
        let params = &[json!(id)];
        println!("{:?}", params);

        let client = jsonrpc::client::Client::new(url.to_owned(), None, None);
        let request = client.build_request(method, params);
        let node = client.send_request(&request).and_then(|res| res.into_result::<Node>())?;
        println!("{:#?}", node);

        Ok(node)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    key: Vec<u8>,
    amt: u64,
}