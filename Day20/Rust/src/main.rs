#![feature(step_by, box_syntax)]

const HOUSES: usize = 1_000_000;
const MAX: i64 = 36_000_000;

fn part1(presents: &mut [i64]) {
    for i in 1..presents.len() {
        for j in (i..presents.len()).step_by(i) {
            presents[j - 1] += i as i64 * 10;
        }
    }
}

fn part2(presents: &mut [i64]) {
    for i in 1..presents.len() {
        let mut steps = 50;

        for j in (i..presents.len()).step_by(i) {
            if steps > 0 {
                presents[j - 1] += i as i64 * 11;
                steps -= 1;
            }
        }
    }
}

fn main() {
    let mut presents1 = box [0_i64; HOUSES];
    let mut presents2 = box [0_i64; HOUSES];

    part1(&mut presents1[..]);
    part2(&mut presents2[..]);

    let pred = |&(_, n): &(usize, &i64)| *n >= MAX;

    let p1res = presents1.iter().enumerate().find(&pred).expect("no result for part 1?");
    let p2res = presents2.iter().enumerate().find(&pred).expect("no result for part 2?");

    println!("Part 1: {} ({}), Part 2: {} ({})",
             p1res.0 + 1, p1res.1,
             p2res.0 + 1, p2res.1);
}
