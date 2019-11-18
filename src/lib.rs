#![feature(test)]
extern crate test; //for bench

#[path = "first.rs"]
pub mod first;

#[path = "second.rs"]
pub mod second;

#[cfg(test)]
mod tests {
    
    use super::test::Bencher;

    #[bench]
    #[ignore]
    fn benchit(b:&mut Bencher){
        b.iter(| | 5*6);
    }
}
