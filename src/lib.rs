#![feature(test)]
extern crate test; //for bench

#[path = "first.rs"]
pub mod first;

#[path = "second.rs"]
pub mod second;

#[path = "thrid.rs"]
pub mod thrid;

#[cfg(test)]
mod tests {
    
}
