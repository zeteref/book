fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if *item > *largest {
            largest = item;
        }
    }

    largest
}

fn hello(name: &str) -> String {
    format!("hello {}!", name)
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let a = &mut hello("world");
    a.push_str("!!");
    println!("{}", a);
}
