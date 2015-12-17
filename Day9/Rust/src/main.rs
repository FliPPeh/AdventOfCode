#![feature(slice_patterns)]
#![feature(convert)]
#![feature(set_recovery)]

use std::collections::{HashSet, HashMap};

trait StreamingIterator<'a> {
    type Item: Sized;

    fn next(&'a mut self) -> Option<Self::Item>;
}

struct PermutationIterator<T> {
    s: Box<[T]>,
}

impl<'a, T> StreamingIterator<'a> for PermutationIterator<T> where T: Clone + Ord + 'a
{
    type Item = &'a [T];

    fn next(&'a mut self) -> Option<Self::Item> {
        let mut i = self.s.len() - 1;

        while i > 0 && self.s[i - 1] >= self.s[i] {
            i -= 1;
        }

        if i <= 0 {
            None
        } else {
            let mut j = self.s.len() - 1;

            while self.s[j] <= self.s[i - 1] {
                j -= 1;
            }

            let temp = self.s[i - 1].clone();
            self.s[i - 1] = self.s[j].clone();
            self.s[j] = temp;

            j = self.s.len() - 1;
            while i < j {
                let temp = self.s[i].clone();
                self.s[i] = self.s[j].clone();
                self.s[j] = temp;

                i += 1;
                j -= 1;
            }

            Some(&*self.s)
        }
    }
}

fn main() {
    let mut places = HashSet::<String>::new();
    let mut distances = HashMap::<(String, String), i32>::new();

    loop {
        let mut line = String::new();
        let res = match std::io::stdin().read_line(&mut line).expect("error") {
            0 => break,
            _ => line.trim().split(" ").collect::<Vec<&str>>(),
        };

        match res.as_slice() {
            [start, "to", end, "=", cost] => {
                println!("'{}' -> '{}' ({})",
                         start,
                         end,
                         cost.parse::<i32>().unwrap());

                places.insert(start.to_owned());
                places.insert(end.to_owned());

                distances.insert((places.get(end).unwrap().clone(),
                                  places.get(start).unwrap().clone()),
                                 cost.parse().unwrap());

                distances.insert((places.get(start).unwrap().clone(),
                                  places.get(end).unwrap().clone()),
                                 cost.parse().unwrap());
            }

            _ => panic!("bad input: {:?}", res),
        }
    }

    // Get a non-hashed, sorted list of places that we can use to permute
    let p = {
        let mut t = places.into_iter().collect::<Vec<String>>();
        t.sort();

        t.into_boxed_slice()
    };

    let mut pi = PermutationIterator { s: p };
    let mut min = std::i32::MAX;
    let mut max = std::i32::MIN;

    while let Some(perm) = pi.next() {
        // println!("{:?}", perm);

        let mut sum = 0_i32;

        for i in 0..perm.len() - 1 {
            sum += *distances.get(&(perm[i].clone(), perm[i + 1].clone()))
                             .unwrap();
        }

        if sum > max {
            max = sum;
        }
        if sum < min {
            min = sum;
        }
    }

    println!("Min: {}, Max: {}", min, max);
}
