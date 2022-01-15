#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

kernel::test_runtime!(test_main);


#[test_case]
fn allocate_single_box() {
    let b = Box::new(42);
    assert_eq!(*b, 42);
}

#[test_case]
fn allocate_vec() {
    let n = 1000;
    let vec = (0..n)
        .into_iter()
        .collect::<Vec<_>>();
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn allocate_repeatedly() {
    for _ in 0..1_000 {
        vec![42u128; 1_000];
    }
}
