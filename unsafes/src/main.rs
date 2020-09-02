use std::slice;

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid],
    //  &mut slice[mid..])
    unsafe {
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    println!("r1 = {:p}, r2 = {:p}", r1, r2);
    unsafe {
        println!("*r1 = {}, *r2 = {}", *r1, *r2);
    }

    {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    let address = 0x0123456usize;
    let r = address as *mut i32;
    println!("r = {:p}", r);

    // let slice : &[i32] = unsafe {
    //     slice::from_raw_parts_mut(r, 10000)
    // };
    // println!("slice = {:?}", slice); // segmentation fault

    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}
