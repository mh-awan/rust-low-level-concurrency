use std::thread;

fn main() {
    thread::spawn(f);
    thread::spawn(f);
    println!("Hello, world!");
}

fn f() {
    println!("hello from another thread");

    let id = thread::current().id();
    println!("this is my thread id: {id:?}")
}
