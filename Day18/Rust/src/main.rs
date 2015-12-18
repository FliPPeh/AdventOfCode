use std::io;
use std::fmt::{self, Display};
use std::str::FromStr;

const STEPS: i32 = 5;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Light {
    Off,
    On,
    Sticky,
}

impl Light {
    fn is_on(&self) -> bool {
        return *self == Light::On || *self == Light::Sticky;
    }

    fn tick(&mut self, n: i32) {
        *self = match *self {
            Light::On if n != 2 && n != 3 => Light::Off,
            Light::Off if n == 3 => Light::On,

            _ => *self,
        }
    }
}


impl Display for Light {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let repr = match *self {
            Light::Off => '.',
            Light::On => '#',
            Light::Sticky => 'O',
        };

        write!(f, "{}", repr)
    }
}

impl FromStr for Light {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().nth(0) {
            None => Err("empty string"),
            Some('.') => Ok(Light::Off),
            Some('#') => Ok(Light::On),
            Some('O') => Ok(Light::Sticky),

            _ => Err("bad grid character"),
        }
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: i32,
    height: i32,

    grid: Vec<Vec<Light>>,
}

impl Grid {
    fn from_strings(input: &[String]) -> Option<Grid> {
        let h = input.len();
        let w = input[0].len();
        let mut grid = Vec::new();

        let mut offrow = Vec::new();

        for _ in 0..w + 2 {
            offrow.push(Light::Off);
        }

        grid.push(offrow.clone());

        for l in input {
            if l.len() != w {
                return None;
            }

            let mut row = l.chars()
                           .map(|c| c.to_string().parse().expect("bad grid char"))
                           .collect::<Vec<_>>();
            row.insert(w, Light::Off);
            row.insert(0, Light::Off);

            grid.push(row);
        }

        grid.push(offrow);

        Some(Grid {
            width: w as i32,
            height: h as i32,
            grid: grid,
        })
    }

    fn count_neighbors(&self, i: i32, j: i32) -> i32 {
        let mut n = 0;

        for k in -1..2 {
            for l in -1..2 {
                if !(k == 0 && l == 0) {
                    if self.grid[(i + k) as usize][(j + l) as usize].is_on() {
                        n += 1;
                    }
                }
            }
        }

        n
    }

    fn step(&mut self) {
        let old = self.clone();

        for i in 1..self.height + 1 {
            for j in 1..self.width + 1 {
                let (iu, ju) = (i as usize, j as usize);

                self.grid[iu][ju].tick(old.count_neighbors(i, j));
            }
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 1..self.width + 1 {
            for j in 1..self.height + 1 {
                try!(write!(f, "{}", self.grid[i as usize][j as usize]));
            }

            try!(write!(f, "\n"));
        }

        Ok(())
    }
}

fn main() {
    let mut lines = Vec::<String>::new();

    loop {
        let mut line = String::new();
        let res = match io::stdin().read_line(&mut line).expect("I/O error") {
            0 => break,
            _ => line.trim().to_owned(),
        };

        if lines.len() > 0 && lines[0].len() != res.len() {
            println!("inconsistent grid size! ({} != {})",
                     lines[0].len(),
                     res.len());

            return;
        }

        lines.push(res);
    }

    let mut grid = Grid::from_strings(&lines).unwrap();

    println!("Initial:\n{}", grid);

    for i in 0..STEPS {
        grid.step();

        println!("Step {}:\n{}", i + 1, grid);
    }
}
