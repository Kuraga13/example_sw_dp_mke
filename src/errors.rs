#![allow(unused)]


use std::fmt;
use std::io;

//#[allow(non_camel_case_types)]
//pub type USBDM_Result = Result<USBDM_RC_OK, USBDMerror>;


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