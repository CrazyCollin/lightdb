use crate::{Response, Storage};

pub trait CommandService{
    fn execute(self,store:&impl Storage)->Response;
}