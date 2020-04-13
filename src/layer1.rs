use serde_json;
use std::collections::HashMap;

pub const OP_HELLO: &'static str = "hello";
pub struct Layer1{
  map:HashMap<i32,i32>
}

  
impl Layer1{
  pub fn new ()->Self{
    let mut init_map = HashMap::new();
    init_map.insert(1, 10);
    init_map.insert(2,20);
    init_map.insert(12,120);
    Layer1{
      map: init_map
    }
  }
  pub fn hello (&self, _actor:&str, key: i32)->i32{
    *self.map.get(&key).unwrap_or(&0)
  }

}