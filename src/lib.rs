#![allow(dead_code)]

pub type DynResult<T> = Result<T, Box<dyn std::error::Error>>;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
