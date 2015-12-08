enum ReadState {
    Simple,
    EscapeInit,
    EscapeSequence(i8)
}

#[derive(Copy, Clone, Debug)]
struct Triplet(i32, i32, i32);

impl std::ops::Add for Triplet {
    type Output = Triplet;

    fn add(self, rhs: Triplet) -> Triplet {
        Triplet(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

fn count(s: &str) -> Triplet {
    let mut state = ReadState::Simple;
    let mut r = Triplet(0, s.len() as i32, s.len() as i32 + 2);

    for c in s.chars() {
        match state {
            ReadState::Simple => {
                match c {
                    '"' => { r.2 += 1; },

                    '\\' => {
                        r.2 += 1; // \\
                        r.0 += 1; // result of escape

                        state = ReadState::EscapeInit;
                    },

                    _ => { // any old character
                        r.0 += 1;
                    }
                }
            },

            ReadState::EscapeInit => {
                state = match c {
                    '"' | '\\' => {
                        r.2 += 1; // \\ | \"
                        ReadState::Simple
                    },

                    // Start of \x sequence, goes on for 2 more chars
                    'x' => ReadState::EscapeSequence(2),
                    _   => panic!("illegal escape: {}", c)
                }
            },

            ReadState::EscapeSequence(n) =>
                state = if n == 1 {
                    ReadState::Simple
                } else {
                    ReadState::EscapeSequence(n - 1)
                }
        }
    }

    println!("{}: {:?}", s, r);

    r
}

fn main() {
    let mut total = Triplet(0, 0, 0);

    loop {
        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                total = total + count(&line.trim());

            },

            Err(e) => panic!("error: {}", e)
        }
    }

    println!("Result 1: {:?}", total.1 - total.0);
    println!("Result 1: {:?}", total.2 - total.1);
}
