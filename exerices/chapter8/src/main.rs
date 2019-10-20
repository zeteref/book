
fn mean(v: &Vec<u32>) -> u32 {
    let mut res = 0;
    for i in v {
        res += *i;
    }
    res / 2
}

fn main() {
    let mut v = vec![1, 2, 3];
    v.sort_by(FnMut<u32, u32>(a, b) {
        a < b
    });

    println!("mean is {}", mean(&v));
    println!("v is {:?}", v);
}
