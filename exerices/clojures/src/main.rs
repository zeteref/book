use std::collections::HashMap;
use std::cmp::Eq;
use std::hash::Hash;

struct Cache<T>
where
    T: Fn(i32) -> i32,
{
    value: Option<i32>,
    calculation: T,
}

impl<T> Cache<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> Cache<T> {
        Cache {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(x) => x,
            None => {
                self.value = Some((self.calculation)(arg));
                arg
            }
        }
    }
}

struct HashCache<T>
where
    T: Fn(i32) -> i32, 
{
    values: HashMap<i32, i32>,
    calculation: T
}

impl<T> HashCache<T>
where
    T: Fn(i32) -> i32,
{
    fn new(calculation: T) -> HashCache<T> {
        HashCache{
            calculation,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        *self.values.entry(arg).or_insert(
            (self.calculation)(arg)
        )
    }
}

struct MultiHashCache<T, V, Z>
where
    T: Fn(V) -> Z, 
    V: Eq + Hash
{
    values: HashMap<V, Z>,
    calculation: T
}

impl<T, V, Z> MultiHashCache<T, V, Z>
where
    T: Fn(V) -> Z,
    V: Eq + Hash + Copy,
    Z: Copy
{
    fn new(calculation: T) -> MultiHashCache<T, V, Z> {
        MultiHashCache{
            calculation,
            values: HashMap::new()
        }
    }

    fn value(&mut self, arg: V) -> Z {
        *self.values.entry(arg).or_insert(
            (self.calculation)(arg)
        )
    }
}

fn main() {
    let mut c = MultiHashCache::<_, _, i32>::new(|x: &str| x.parse().unwrap() );
    println!("{}", c.value("1"));
    println!("{}", c.value("2"));
}
