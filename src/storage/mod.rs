mod memory;

use crate::{KvPair, Result};
use crate::value::Value;

pub trait Storage{
    // set a kv pair and return old value
    fn set(&mut self,key:&str,value:&str)->Result<Option<Value>>;
    // get value from store
    fn get(&mut self,key:&str)->Result<Option<Value>>;
    // delete a kv pair and return it after remove
    fn delete(&mut self,key:&str)->Result<Option<Value>>;
    // check if a key is exist in store
    fn contains(&mut self,key:&str)->Result<bool>;
    // get all kv pairs from specify table
    fn get_all(&mut self, table:&str) ->Result<Vec<KvPair>>;
    // get iterator from specify table
    fn get_iter(&mut self,table:&str)->Result<Box<dyn Iterator<Item=KvPair>>>;

}