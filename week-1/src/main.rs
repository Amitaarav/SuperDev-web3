use std::fmt::Error;
// Macro: this code never run
// cargo run 
// 1. binary gets created
// 2. getting run

// cargo build
// ./target/debug/week-1

// g++ a.cpp -o .bin
// macro expansion
   // high level rust code

struct Rect {
    width: u32,
    height: u32
}

struct Square {
    side: u32
}

trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

impl Shape for Rect{
    fn area(&self) -> u32{
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

impl Shape for Square{
    fn area(&self) -> u32{
        return self.side * self.side;
    }

    fn perimeter(&self) -> u32 {
        return 4 * (self.side);
    }
}

fn get_area_and_perimeter(s: impl Shape) -> ( u32, u32){
    return (s.area(), s.perimeter());
}

// ++===================
#[derive(Debug)]
struct User {
    name: String,
    age: u32
}

// too much code
// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
//         write!(f, "name is {}, age is {}", self.name, self.age)
//     }
// }

// impl std::fmt::Display for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
//         write!(f, "name is {}, age is {}", self.name, self.age)
//     }
// }

// Derived macros


// custom macro

trait Serialize{
    fn serialize(&self) -> Vec<u8>;
}

struct Swap{
    qty_1: u32,
    qty_2: u32
}

impl Serialize for Swap{
    fn serialize(&self) -> Vec<u8>{
        let mut v = Vec::new();
        v.extend_from_slice(&self.qty_1.to_be_bytes());
        v.extend_from_slice(&self.qty_2.to_be_bytes());
        return v;
    }
}

trait Deserialize: Sized{
    fn deserialize(bytes: &[u8]) -> Self;
}

impl Deserialize for Swap{
    fn deserialize(bytes: &[u8]) -> Self{
        let qty_1 = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        let qty_2 = u32::from_be_bytes(bytes[0..4].try_into().unwrap());
        return Swap{qty_1, qty_2};
    }
}
fn main() {
    // println!("Hello, world!"); // ! not a function, but a macro invocation
    let r = Rect {
        width: 20,
        height: 10
    };

    let s = Square {
        side: 10
    };

    let (area, perimeter) = get_area_and_perimeter(r);

    let u = User {
        name: String::from("Amit"),
        age: 18
    };

    // println!("{}", u); // Display
    println!("{:?} {:?}", u.name, u.age); // Debug

    let swap = Swap { qty_1: 10, qty_2: 20 };

    let bytes = swap.serialize();
    println!("{:?}", bytes);

    let decoded = Swap::deserialize(&bytes);
    println!("{} {}", decoded.qty_1, decoded.qty_2);
}