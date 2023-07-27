use std::fmt::Debug;

#[derive(Copy, Clone)]
pub enum LogLevel {
    Info,
    Warn,
    Error
}

impl Default for LogLevel {
    fn default() -> Self {LogLevel::Info}
}

#[derive(Debug)]
enum TypeDefinition{
    Internal,
    UserCustomize,
}

impl TypeDefinition {
    fn to_byte(&self)->u8{
        match self {
            TypeDefinition::Internal=>1,
            TypeDefinition::UserCustomize=>2,
        }
    }

    fn from_byte(value:u8)->Self{
        match value {
            1=>TypeDefinition::Internal,
            2=>TypeDefinition::UserCustomize,
            _=>unreachable!(),
        }
    }
}


enum TypeName{

}

pub trait LightKVValue:Debug{
    type SelfType:Debug;

    fn from_bytes(data:&[u8])->Self::SelfType;

    fn as_bytes(&self)->&[u8];

    fn type_name()->String;
}

pub trait LightKVKey:LightKVValue{

}
