fn main() {
    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    another_function(5, 6);
}

fn another_function(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}
