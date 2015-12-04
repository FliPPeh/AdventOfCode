use std::io::{Read};

fn main() {
    let mut buf: [u8; 1] = [0; 1];
    let mut level: i32 = 0;
    let mut basement: Option<u32> = None;
    let mut i: u32 = 0;

    while let Ok(n) = std::io::stdin().read(&mut buf) {
        i += 1;

        if n == 0 {
            break;
        } else {
            match std::char::from_u32(buf[0] as u32) {
                Some('(') => {
                    level += 1
                },
                Some(')') => {
                    level -= 1;

                    if level < 0 && basement.is_none() {
                        basement = Some(i);
                    }
                },

                // Newlines
                Some('\n') | Some('\r') => {
                    continue;
                }
                _  => { panic!("invalid input: {}", buf[0]) }
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
