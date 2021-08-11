#![allow(dead_code)] //https://stackoverflow.com/questions/27454761/what-is-a-crate-attribute-and-where-do-i-add-it
#![allow(unused_imports)]

pub mod download;

pub mod course_manager;

#[cfg(test)]
mod tests;
