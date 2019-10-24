use std::collections::HashMap;

fn mean(v: &Vec<u32>) -> u32 {
    let mut res = 0;
    for i in v {
        res += *i;
    }
    (res / 2)
}

fn median(v: &Vec<u32>) -> u32 {
    let mut v = v.to_vec();
    v.sort();

    if v.len() % 2 == 0 {
        (v[v.len()/2-1] + v[v.len()/2])/2
    } else {
        v[v.len()/2]
    }
}

fn mode(v: &Vec<u32>) -> u32 {
    let mut cnt = HashMap::new();
    for i in v {
        let c = cnt.entry(*i).or_insert(0);
        *c += 1;
    }
    let mut items: Vec<_> = cnt.into_iter().collect();
    items.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    items[0].0
}

fn piglatin(s: &str) -> &str {

}

fn main() {
    let v = vec![4, 2, 3, 4];
    println!("mean is {}", mean(&v));
    println!("median is {}", median(&v));
    println!("mode is {}", mode(&v));
}



