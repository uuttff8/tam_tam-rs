pub mod tam_tam;
pub mod error;


pub use tam_tam::TamTam;
//use self::error::Result;

pub type Result<T> = ::std::result::Result<T, reqwest::Error>;
