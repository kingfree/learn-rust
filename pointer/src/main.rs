use std::rc::Rc;

enum List<T> {
    Cons(T, Rc<List<T>>),
    Nil,
}

struct Pointer<T>(T);

impl<T> Pointer<T> {
    fn new(x: T) -> Pointer<T> {
        Pointer(x)
    }
}

use std::ops::Deref;

impl<T> Deref for Pointer<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

use crate::List::{Cons, Nil};

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::mem::drop;

fn main() {
    let _list = Cons(1, 
        Rc::new(Cons(2, 
            Rc::new(Cons(3, 
                Rc::new(Nil))))));

    let x = 5;
    let y = Pointer::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let _c = CustomSmartPointer { data: String::from("my stuff") };
    drop(_c);
    let _d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");

    let a = Rc::new(Cons(5,
        Rc::new(Cons(10,
            Rc::new(Nil)))));
    println!("创建 a 后的引用计数 = {}", Rc::strong_count(&a));
    let b = Cons(3, a.clone()); // 只增加引用计数，而非深度拷贝
    println!("创建 b 后的引用计数 = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // 建议用这个
        println!("创建 c 后的引用计数 = {}", Rc::strong_count(&a));
    }
    println!("离开 c 作用域后的引用计数 = {}", Rc::strong_count(&a));
}
