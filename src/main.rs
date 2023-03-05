use crate::problem1::{sum, dedup, filter};

pub mod problem1;

fn is_even(v: i32) -> bool {
    (v % 2) == 0
}

fn main() {
    let arr = [1, 2, 3];
    let v1: Vec<i32> = vec![10, 20, 10, 31, 10, 20];
    let res = sum(&arr);
    let dups = dedup(&v1);
    let filtered = filter(&v1, &is_even);
    println!("{:?}", dups);
    println!("{}", res);
    println!("{:?}", filtered);
}