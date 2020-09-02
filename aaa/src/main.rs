fn print_vec(v: &mut Vec<i32>) {
    for &i in v.iter() {
        println!("{}", i);
    }
    v.push(4);
}

fn main() {
    let v = vec![1, 2, 3];
    print_vec(&mut v);
    print_vec(&mut v);
    println!("Hello, world!");
}
