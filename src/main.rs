#![feature(const_fn)]
#![feature(const_if_match)]
#![feature(const_transmute)]

const fn sift_down(arr: &mut [usize], mut node: usize) {
    loop {
        let left = 2 * node + 1;
        let right = 2 * node + 2;

        let greater = if right < arr.len() && arr[left] < arr[right] {
            right
        } else {
            left
        };

        if greater >= arr.len() || arr[node] >= arr[greater] {
            break;
        }

        let tmp = arr[node];
        arr[node] = arr[greater];
        arr[greater] = tmp;
        node = greater;
    }
}

const fn heap_sort(arr: &mut [usize]) {
    use std::mem::transmute;
    let len = arr.len();
    let mut i = len / 2 - 1;
    loop {
        sift_down(arr, i);
        if i == 0 {
            break;
        }
        i -= 1;
    }

    i = len - 1;
    loop {
        let tmp = arr[0];
        arr[0] = arr[i];
        arr[i] = tmp;
        unsafe {
            sift_down(transmute((arr.as_ptr(), i)), 0);
        }
        if i == 1 {
            break;
        }
        i -= 1;
    }
}

const fn sort(mut arr: [usize; 10]) -> [usize; 10] {
    heap_sort(&mut arr);
    arr
}

const ARR: [usize; 10] = sort([5, 1, 3, 1, 6, 1, 73, 1, 2, 7]);

fn main() {
    println!("{:?}", ARR);
}

