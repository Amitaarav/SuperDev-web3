fn main() {
    let a = Box::new(5);
    println!("a is {:p}", a);
    println!("a is {:p}", &a);
    println!("Hello, world!");
}
