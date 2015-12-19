pub use std::collections::{HashMap, HashSet};
use std::io;

pub fn replace_all(s: &str, p: &str, r: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut offset = 0;

    while let Some(j) = s[offset..].find(p) {
        let mut c = String::new();
        c.push_str(&s[0..offset + j]);
        c.push_str(r);
        c.push_str(&s[offset + j + p.len()..]);

        res.push(c);

        offset += j + p.len();
    }

    res
}

pub fn replace_first(s: &str, p: &str, r: &str) -> String {
    if let Some(j) = s.find(p) {
        let mut c = String::new();
        c.push_str(&s[0..j]);
        c.push_str(r);
        c.push_str(&s[j + p.len()..]);

        c
    } else {
        s.to_owned()
    }
}

mod part1 {
    use super::*;

    pub fn main(input: &str, replacements: &HashMap<String, Vec<String>>) -> usize {
        let mut combs = HashSet::new();

        for (key, val) in replacements {
            for choice in val {
                println!("{} => {}", key, choice);

                for c in replace_all(input, &key, &choice) {
                    combs.insert(c);
                }
            }
        }

        combs.len()
    }
}

mod part2 {
    extern crate rand;

    use super::*;
    use self::rand::{thread_rng, Rng};

    fn flatten(repl: &HashMap<String, Vec<String>>) -> Vec<(&str, &str)> {
        let mut res = Vec::<(&str, &str)>::new();

        for (key, val) in repl {
            for choice in val {
                res.push((&*key, &*choice));
            }
        }

        res.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        res
    }

    pub fn main(input: &str, replacements: &HashMap<String, Vec<String>>) -> usize {
        let mut rev = flatten(replacements);
        let mut formula = input.to_owned();
        let mut count = 0;

        while formula != "e" {
            let oformula = formula.clone();

            for &(k, v) in &rev {
                if formula.find(v).is_none() {
                    continue;
                }

                formula = replace_first(&formula, v, k);
                count += 1;
            }

            if oformula == formula {
                formula = input.to_owned();
                count = 0;
                thread_rng().shuffle(&mut rev);
            }
        }

        count
    }
}

fn main() {
    let mut lines = Vec::new();
    let mut repl = HashMap::<String, Vec<String>>::new();

    loop {
        let mut line = String::new();
        let res = match io::stdin().read_line(&mut line).expect("I/O error") {
            0 => break,
            _ => line.trim().to_owned(),
        };

        lines.push(res);
    }

    if lines.len() < 3 {
        println!("bad input!");
        return;
    }

    let formula = lines.pop().unwrap();

    for r in &lines {
        if r.len() == 0 {
            continue;
        }

        let mut p = r.split(" => ");
        let key = p.next().unwrap();
        let val = p.next().unwrap();

        if !repl.contains_key(key) {
            repl.insert(key.to_owned(), Vec::new());
        }

        repl.get_mut(key).unwrap().push(val.to_owned());
    }

    println!("Formula: {}", formula);
    println!("{:?}", repl);

    println!("{}", part1::main(&formula, &repl));
    println!("{}", part2::main(&formula, &repl));
}
