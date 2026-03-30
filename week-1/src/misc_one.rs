fn main(){
    let mut s = String::new();

    println!("{}", s.capacity());

    s.push_str("helllllllllllllll");

    println!("{}", s.capacity());
    // for _ in 0..5 {
    //     s.push_str("hello");
    //     println!("{}", s.capacity());
    // }
}