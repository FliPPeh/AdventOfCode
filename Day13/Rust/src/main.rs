#![feature(slice_patterns)]
#![feature(convert)]

use std::collections::{HashMap, HashSet};

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
    let mut rel: HashMap<(String, String), i32> = HashMap::new();
    let mut uniq: HashSet<String> = HashSet::new();

    loop {
        let mut line = String::new();
        let res = match std::io::stdin().read_line(&mut line).expect("error") {
            0 => break,
            _ => line.trim().split(" ").collect::<Vec<&str>>(),
        };

        match res.as_slice() {
            [a, "would", o, n, "happiness", "units",
                "by", "sitting", "next", "to", b] => {
                let nr = if o == "gain" {
                    n.parse().unwrap()
                } else {
                    -n.parse::<i32>().unwrap()
                };

                let mut br = b.to_owned();
                br.remove(b.len() - 1);

                rel.insert((a.to_owned(), br.clone()), nr);

                uniq.insert(a.to_owned());
                uniq.insert(br);
            }

            _ => panic!("wat"),
        }
    }

    println!("{:#?}", rel);
    println!("{:#?}", uniq);

    let p = {
        let mut t = uniq.into_iter().collect::<Vec<String>>();
        t.sort();

        t.into_boxed_slice()
    };

    let mut pi = PermutationIterator { s: p };
    let mut max = std::i32::MIN;

    while let Some(perm) = pi.next() {
        let mut sum = 0_i32;

        for i in 0..perm.len() {
            // Add happiness to the left
            if i > 0 {
                sum += *rel.get(&(perm[i].clone(), perm[i - 1].clone()))
                           .unwrap();
            } else {
                sum += *rel.get(&(perm[i].clone(), perm[perm.len() - 1].clone()))
                           .unwrap();
            }

            // And then look to the right
            if i < perm.len() - 1 {
                sum += *rel.get(&(perm[i].clone(), perm[i + 1].clone()))
                           .unwrap();
            } else {
                sum += *rel.get(&(perm[i].clone(), perm[0].clone()))
                           .unwrap();
            }
        }

        if sum > max {
            max = sum;
        }
    }

    println!("{}", max);
}
