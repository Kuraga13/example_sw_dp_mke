#![allow(unused)]


use std::fmt;
use std::io;



#[allow(non_camel_case_types)]
#[derive(Debug,Clone)]
pub enum Error {

   MdmExample(String),
}




impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       write!(f, "{:?}", self)
    }
}