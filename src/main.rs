use compiletime_sort_derive::sort;

#[sort]
const ARR: [usize; 10] = [5, 1, 3, 1, 6, 1, 73, 1, 2, 7];

fn main() {
    println!("Hello, world!");
}
