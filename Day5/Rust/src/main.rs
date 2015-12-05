use std::*;

fn is_nice_v1(s: &str) -> bool {
    if ["ab", "cd", "pq", "xy"].iter().any(|bw| s.contains(bw)) {
        return false;
    }

    const VOWELS: &'static str = "aeiou";

    let mut vowels = 0_u8;
    let mut dupl = false;

    let mut last: Option<char> = None;

    for c in s.chars() {
        if let Some(true) = last.map(|l| l == c) {
            dupl = true;
        }

        if VOWELS.contains(c) {
            vowels += 1;
        }

        last = Some(c);
    }

    vowels >= 3 && dupl
}

fn is_nice_v2(s: &str) -> bool {
    let mut has_rep= false;
    let mut has_pair = false;

    let mut c = s.chars();

    loop {
        // Get an iterator to the current position, before next()ing c
        let i = c.clone();

        if let cr@Some(_) = c.next() {
            let pair: String = i.clone().take(2).collect();
            let rest: String = i.clone().skip(2).collect();

            if !has_pair && rest.clone().contains(&pair) { has_pair = true; }
            if !has_rep  && cr == i.clone().nth(2)       { has_rep = true; }

            if has_pair && has_rep { return true; }
        } else {
            break;
        }
    }

    false
}

fn main() {
    let mut line = String::new();
    let mut nice_strings_v1 = Vec::<String>::new();
    let mut nice_strings_v2 = Vec::<String>::new();

    loop {
        match io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed_line = line.trim();

                if is_nice_v1(trimmed_line) {
                    nice_strings_v1.push(trimmed_line.to_owned());
                }

                if is_nice_v2(trimmed_line) {
                    nice_strings_v2.push(trimmed_line.to_owned());
                }
            },

            Err(e) => panic!("error: {}", e),
        }

        line.clear();
    }

    println!("Nice strings (v1): {}", nice_strings_v1.len());
    println!("Nice strings (v2): {}", nice_strings_v2.len());
}
