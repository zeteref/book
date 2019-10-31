use std::collections::HashMap;

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn mode(list: &Vec<i32>) -> i32 {
    let mut cnt = HashMap::<i32, i32>::new();
    for i in list {
        let c = cnt.entry(*i).or_insert(0);
        *c += 1;
    }

    let max = 1;
    let mut ret = &list[0];

    for (k, v) in &cnt {
        if *v > max {
            ret = k;
        }
    }
    *ret
}

fn hello<T: std::fmt::Display>(list: &Vec<T>) {
    for i in list {
        println!("{}", i);
    }

    let z = &list[0];
    let mut x = &list[0];
    println!("{}", x);
    let mut y = &list[0];
    println!("{}", x);
}

fn piglatin(s: &str) -> String {
    let mut chars = s.chars().peekable();
    let mut out = String::new();

    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'o' | 'e' | 'u' | 'i' => {
                out.push(c);
                String::from("-hay")
            }
            'a'..='z' | 'A'..='Z' => {
                format!("-{}ay", c)
            }
            _ => {
                out.push(c);
                continue;
            }
        };

        while let Some(&c) = chars.peek() {
            match c {
                'a'..='z' => {
                    out.push(c);
                    chars.next();
                }
                _ => break
            }
        }
        out += &suffix;
    }
    out
}

fn main() {
    println!("{}", piglatin("first apple"));
}
