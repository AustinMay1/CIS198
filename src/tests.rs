#![cfg(test)]

use crate::problem1::{sum, dedup, filter};
use crate::problem2::sieve;

#[test]
fn test_sum_sm() {
    let array = [1,2,3];
    assert_eq!(sum(&array), 6);
}

#[test]
fn test_dedup_sm() {
    let vs = vec![10, 20, 10, 31, 10, 20];
    assert_eq!(dedup(&vs), vec![10,20])
}

fn is_even(x: i32) -> bool {
    (x % 2) == 0
} 

#[test]
fn test_filter_sm() {
    let vs = vec![10,20,10,31,10,20];
    assert_eq!(filter(&vs, &is_even), vec![10,20,10,10,20])
}

#[test]
fn test_sieve_sm() {
    assert_eq!(sieve(25), vec![2,3,5,7,11,13,17,19,23])
}