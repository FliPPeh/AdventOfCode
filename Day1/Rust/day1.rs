#![feature(io)]
use std::io::Read; // chars()

fn main() {
    let mut level: i32 = 0;
    let mut basement: Option<u32> = None;
    let mut i: u32 = 0;

    for c in std::io::stdin().chars() {
        match c.ok() {
            None => panic!("invalid input :(!"),

            Some('(') => level += 1,
            Some(')') => level -= 1,
            Some('\r') | Some('\n') => continue,

            Some(x) => panic!("bad command: {}", x),
        }

        if basement.is_none() {
            i += 1;

            if level < 0 {
                basement = Some(i);
            }
        }
    }

    if let Some(basement_position) = basement {
        println!("Finished on level {}, first entered basement at move {}.",
                 level,
                 basement_position);
    } else {
        println!("Finished on level {}, never entered basement.", level);
    }
}
