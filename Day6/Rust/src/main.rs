fn parse_coords(c: &str) -> (usize, usize) {
    let mut p = c.split(",").map(|s| s.parse::<usize>().ok().unwrap());

    (p.next().unwrap(), p.next().unwrap())
}

fn get_rect(start: &str, end: &str) -> (usize, usize, usize, usize) {
    let start_coord = parse_coords(start);
    let end_coord = parse_coords(end);

    (start_coord.0, start_coord.1, end_coord.0, end_coord.1)
}

type Rect = (usize, usize, usize, usize);

enum Action {
    ToggleRect,
    ActivateRect,
    DeactivateRect
}

const W: usize = 1000;
const H: usize = 1000;


fn apply(a: Action, r: Rect, grid: &mut [[u32; W]; H]) {
    for y in r.0 .. r.2 + 1 {
        for x in r.1 .. r.3 + 1 {
            match a {
                Action::ToggleRect => grid[y][x] += 2,
                Action::ActivateRect => grid[y][x] += 1,
                Action::DeactivateRect => if grid[y][x] > 0 {
                    grid[y][x] -= 1;
                }
            }
        }
    }
}

fn main() {
    let mut grid = [[0_u32; W]; H];

    loop {
        let mut line = String::new();

        match std::io::stdin().read_line(&mut line) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let r: Vec<&str> = line.trim().split(" ").collect();

                match r[0] {
                    "turn" => match r[1] {
                        "on" | "off" => {
                            let rect = get_rect(r[2], r[4]);

                            if r[1] == "on" {
                                apply(Action::ActivateRect, rect, &mut grid);
                            } else {
                                apply(Action::DeactivateRect, rect, &mut grid);
                            }
                        }

                        _ => panic!("bad command 'turn {}'", r[1]),
                    },

                    "toggle" => {
                        apply(Action::ToggleRect, get_rect(r[1], r[3]),
                            &mut grid);
                    },

                    _ => panic!("bad command '{}'", r[0])
                }
            }

            Err(e) => panic!("error: {}", e)
        }

        line.clear();
    }

    let mut brightness = 0;

    for r in grid.iter() {
        for l in r.iter() {
            brightness += *l;
        }
    }

    println!("Total brightness: {}", brightness);
}
