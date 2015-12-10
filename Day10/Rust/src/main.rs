use std::env;

fn groupdigits(s: &str) -> String {
    let mut running: i32 = 0;
    let mut last: Option<char> = None;

    let mut r = String::new();

    for c in s.chars() {
        if last.is_none() {
            last = Some(c);
            running = 0;
        } else if let Some(c2) = last {
            if c2 != c {
                r.push_str(&format!("{}{}", running, c2));

                last = Some(c);
                running = 0;
            }
        }

        running += 1;
    }

    r.push_str(&format!("{}{}", running, last.unwrap()));

    r
}

fn main() {
    if let None = env::args().nth(1) {
        println!("usage: {} <number>", env::args().nth(0).unwrap());
        return;
    }

    let input = env::args().nth(1).unwrap();

    if !input.chars().all(|n| n.is_digit(10)) {
        println!("{}: '{}' is not all-numeric",
            env::args().nth(0).unwrap(),
            input);

        return;
    }

    const ROUNDS: i32 = 50;

    let mut r = input.to_owned();

    for _ in 0..ROUNDS {
        r = groupdigits(&r);
    }

    println!("{}", r.len());
}
