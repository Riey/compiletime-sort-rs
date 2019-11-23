#![feature(proc_macro_hygiene)]

use compiletime_sort_derive::sort_panic;

fn main() {
    let arr = sort_panic!([5, 1, 3, 1, 6, 1, 73, 1, 2, 7]);
    println!("Hello, world!");
}
