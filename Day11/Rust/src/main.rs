use std::*;

use std::collections::HashSet;

fn increment_string(s: &str) -> String {
    s.chars().rev().fold(("".to_owned(), true), |mut acc, c| {
        if acc.1 {
            if c == 'z' {
                acc.0.insert(0, 'a'); // wrap over, keep carrying

                if acc.0.len() == s.len() {
                    acc.0.insert(0, 'a');
                }
            } else {
                acc.0.insert(0, char::from_u32(c as u32 + 1).unwrap());
                acc.1 = false;
            }
        } else {
            acc.0.insert(0, c);
        }

        acc
    }).0
}

fn is_good(s: &str) -> bool {
    if ["i", "o", "l"].iter().any(|c| s.contains(c)) {
        return false;
    }

    let mut w = (None, None, None);
    let mut has_seq = false;
    let mut pairs = HashSet::new();

    let c0 = |c0: char| c0 as u32;
    let c1 = |c1: char| c1 as u32 - 1;
    let c2 = |c2: char| c2 as u32 - 2;

    for c in s.chars() {
        w.0 = w.1;
        w.1 = w.2;
        w.2 = Some(c);

        if w.1 == w.2 {
            pairs.insert(c);
        } else if w.0.map(&c0) == w.1.map(&c1) && w.1.map(&c1) == w.2.map(&c2) {
            has_seq = true;
        }
    }

    has_seq && pairs.len() >= 2
}

fn main() {
    let mut input = match env::args().nth(1) {
        None => {
            println!("usage: {} <password>", env::args().nth(0).unwrap());
            return;
        },

        Some(p) => p,
    };

    while !is_good(&input) {
        input = increment_string(&input);

        println!("Trying: {}...", input);
    }

    println!("Success: {}", input);
}
