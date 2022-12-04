/// client request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(oneof = "request::Request", tags = "1, 2, 3, 4, 5, 6, 7, 8, 9")]
    pub request: ::core::option::Option<request::Request>,
}
/// Nested message and enum types in `Request`.
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "1")]
        Hget(super::Hget),
        #[prost(message, tag = "2")]
        Hset(super::Hset),
        #[prost(message, tag = "3")]
        Hdel(super::Hdel),
        #[prost(message, tag = "4")]
        Hgetall(super::Hgetall),
        #[prost(message, tag = "5")]
        Hmget(super::Hmget),
        #[prost(message, tag = "6")]
        Hmset(super::Hmset),
        #[prost(message, tag = "7")]
        Hmdel(super::Hmdel),
        #[prost(message, tag = "8")]
        Hexists(super::Hexists),
        #[prost(message, tag = "9")]
        Hmexists(super::Hmexists),
    }
}
/// server response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// status code
    #[prost(uint32, tag = "1")]
    pub status: u32,
    /// response message
    #[prost(string, tag = "2")]
    pub message: ::prost::alloc::string::String,
    /// response value data
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<Value>,
    /// response kv pairs
    #[prost(message, repeated, tag = "4")]
    pub pairs: ::prost::alloc::vec::Vec<KvPair>,
}
/// get value form specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hget {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// set a kv pair to specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hset {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pair: ::core::option::Option<KvPair>,
}
/// delete a kv pair from specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hdel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// get all kv pairs from specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hgetall {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
}
/// get multiple kv pairs from specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmget {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// set multiple kv pairs to specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmset {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub pairs: ::prost::alloc::vec::Vec<KvPair>,
}
/// delete multiple kv pairs from specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmdel {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// check if a kv pair exists in specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hexists {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
}
/// check if multiple kv pairs exists in specify hash table
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hmexists {
    #[prost(string, tag = "1")]
    pub table: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// kv pair value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(oneof = "value::Value", tags = "1, 2, 3, 4, 5")]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "1")]
        String(::prost::alloc::string::String),
        #[prost(bytes, tag = "2")]
        Binary(::prost::alloc::vec::Vec<u8>),
        #[prost(int64, tag = "3")]
        Integer(i64),
        #[prost(double, tag = "4")]
        Float(f64),
        #[prost(bool, tag = "5")]
        Bool(bool),
    }
}
/// kv pair
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KvPair {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
}
