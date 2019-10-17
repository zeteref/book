#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

struct Foo(String);


impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rec = Rectangle { width: 30, height: 50 };

    println!("rectangle are is {}", rec.area());

    let a = Foo(String::from("a"));
    let b = Foo(String::from("b"));

    let Foo(ref A) = a;
    let B = b.0;


}


