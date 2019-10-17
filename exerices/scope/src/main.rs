fn main() {

    let mut guess = String::new();

    let test = &guess;
    println!("guess is {}", test);
    append_to_str(&mut guess);


    println!("guess is {}", test);
}

fn append_to_str(s: &mut String) {
    s.push_str("testing");
    println!("s is {}", s);
}
