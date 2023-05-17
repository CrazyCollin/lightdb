use bytes::Bytes;
use lightkv::{engine, options::Options};

fn main() {
    let opts=Options::default();
    let db=engine::Engine::open(opts).expect("open lightkv engine err");
    
    // put a key-value pair
    let _=db.put(Bytes::from("key1"),Bytes::from("value1"));
    // get a key-value pair
    let value1=db.get(Bytes::from("key1"));
    println!("{:?}",value1.unwrap());
    // change a key-value pair
    
    // get a key-value pair
    
    // delete a key-value pair
    
    
}