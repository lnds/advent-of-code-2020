use std::collections::HashMap;
use std::io::{self, BufRead};

fn challenge1(target: i64, entries: &[i64]) -> Option<i64> {
    let mut pair: HashMap<i64, ()> = HashMap::new();
    for n in entries.iter() {
        let dif = target - n;
        if pair.contains_key(&dif) {
            return Some(n*dif);
        }
        pair.insert(*n, ());
    }
    None
}

fn get_two_factors(target: i64, entries: &[i64], pairs: &mut HashMap<i64, ()>) -> Option<(i64, i64)> {
    for n in entries.iter() {
        let dif = target - n;
        if pairs.contains_key(&dif) {
            return Some((*n, dif));
        }
        pairs.insert(*n, ());
    }
    None
}

fn challenge2(target: i64, entries: &[i64]) -> Option<i64> {
    let mut pair: HashMap<i64, ()> = HashMap::new();
    for n in entries.iter() {
        let dif = target - n;
        if let Some((n1, n2)) = get_two_factors(dif, entries, &mut pair) {
            return Some(n*n1*n2);
        };
    }
    None
}

fn main() -> io::Result<()> {
    let test = [1721, 979, 366, 299, 675, 1456];
    println!("challenge 1");
    match challenge1(2020, &test) {
        Some(n) => println!("{}", n),
        None => println!("nada")
    }
    let numbers: Vec<i64> = io::stdin().lock().lines().flatten().flat_map(|s|s.parse::<i64>()).collect();
    match challenge1(2020, &numbers) {
        Some(n) => println!("{}", n),
        None => println!("nada")
    }
    println!("challenge 2");
    match challenge2(2020, &test) {
        Some(n) => println!("{}", n),
        None => println!("nada")
    }
    match challenge2(2020, &numbers) {
        Some(n) => println!("{}", n),
        None => println!("nada")
    }

    Ok(())
}
