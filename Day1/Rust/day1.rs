#![feature(io)]

use std::io::{Read};

fn main() {
    let mut level: i32 = 0;
    let mut basement: Option<u32> = None;
    let mut i: u32 = 0;

    for c in std::io::stdin().chars() {
        i += 1;

        match c.ok() {
            Some('(') => {
                level += 1
            },
            Some(')') => {
                level -= 1;

                if level < 0 && basement.is_none() {
                    basement = Some(i);
                }
            },

            Some('\n') | Some('\r') => {
                continue;
            }

            Some(x) => {
                panic!("bad command: {}", x);
            }

            None => {
                panic!("invalid input :(!");
            }
        }
    }

    if let Some(basement_position) = basement {
        println!("Finished on level {}, first entered basement at move {}.",
             level, basement_position);
    } else {
        println!("Finished on level {}, never entered basement.", level);
    }
}
