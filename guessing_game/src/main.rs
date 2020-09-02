use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("猜数字游戏");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("输入你的猜测: ");

        let mut guess = String::new(); // mut = mutable

        io::stdin().read_line(&mut guess)
            .expect("读取输入行失败");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了"),
            Ordering::Greater => println!("大了"),
            Ordering::Equal => {
                println!("对了！");
                break;
            }
        }
    }
}
