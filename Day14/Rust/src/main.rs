#![feature(slice_patterns)]

use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
enum ReindeerState {
    Flying(i32),
    Resting(i32),
}

#[derive(Debug)]
struct Reindeer {
    speed: i32,

    flying_time: i32,
    resting_time: i32,

    distance: i32,
    points: i32,

    state: ReindeerState,
}

impl Reindeer {
    fn new(speed: i32, flying_time: i32, resting_time: i32) -> Reindeer {
        Reindeer {
            speed: speed,
            flying_time: flying_time,
            resting_time: resting_time,
            distance: 0,
            points: 0,
            state: ReindeerState::Flying(flying_time),
        }
    }

    fn tick(&mut self) {
        match self.state {
            ReindeerState::Flying(0) => {
                self.state = ReindeerState::Resting(self.resting_time - 1);
            }

            ReindeerState::Resting(0) => {
                self.distance += self.speed;
                self.state = ReindeerState::Flying(self.flying_time - 1);
            }

            ReindeerState::Flying(n) => {
                self.distance += self.speed;
                self.state = ReindeerState::Flying(n - 1);
            }
            ReindeerState::Resting(n) => {
                self.state = ReindeerState::Resting(n - 1);
            }
        }
    }
}

fn main() {
    let mut reindeer = HashMap::new();

    loop {
        let mut line = String::new();
        let res = match std::io::stdin().read_line(&mut line).expect("error") {
            0 => break,
            _ => line.trim().split(" ").collect::<Vec<&str>>(),
        };

        match &res[..] {
            [name, "can", "fly", kms, "km/s", "for", n, "seconds,", "but",
            "then", "must", "rest", "for", r, "seconds."] => {
                reindeer.insert(name.to_owned(),
                                RefCell::new(Reindeer::new(kms.parse().unwrap(),
                                                           n.parse().unwrap(),
                                                           r.parse().unwrap())));
            }

            _ => panic!("unexpected input: {:?}", res),
        }
    }

    for _ in 0..2503 {
        for (_, deer_cell) in &reindeer {
            deer_cell.borrow_mut().tick();
        }

        let leading = reindeer.iter().fold(0, |acc, (_, deer)| {
            if deer.borrow().distance > acc {
                deer.borrow().distance
            } else {
                acc
            }
        });

        for (_, deer_cell) in &reindeer {
            let mut deer = deer_cell.borrow_mut();

            if deer.distance == leading {
                deer.points += 1;
            }
        }
    }

    for (name, deer_cell) in &reindeer {
        let deer = deer_cell.borrow();

        println!("{}: {} km, {} points", name, deer.distance, deer.points);
    }
}
