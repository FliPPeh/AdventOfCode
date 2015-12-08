fn parse_coords(c: &str) -> (usize, usize) {
    let mut p = c.split(",").map(|s| s.parse::<usize>().ok().unwrap());

    (p.next().unwrap(), p.next().unwrap())
}

fn get_rect(start: &str, end: &str) -> (usize, usize, usize, usize) {
    let start_coord = parse_coords(start);
    let end_coord = parse_coords(end);

    (start_coord.0, start_coord.1, end_coord.0, end_coord.1)
}

const W: usize = 1000;
const H: usize = 1000;

type Grid = [[u32; W]; H];
type Rect = (usize, usize, usize, usize);
enum Action { Toggle, Activate, Deactivate }

fn apply(a: Action, r: Rect, grid: &mut Grid) {
    for y in r.0 .. r.2 + 1 {
        for x in r.1 .. r.3 + 1 {
            match a {
                Action::Toggle => grid[y][x] += 2,
                Action::Activate => grid[y][x] += 1,
                Action::Deactivate => if grid[y][x] > 0 { grid[y][x] -= 1; }
            }
        }
    }
}

fn main() {
    let mut grid: Grid = [[0; W]; H];

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
                            let act = if r[1] == "on" {
                                Action::Activate
                            } else {
                                Action::Deactivate
                            };

                            apply(act, rect, &mut grid);
                        }

                        _ => panic!("bad command 'turn {}'", r[1]),
                    },

                    "toggle" =>
                        apply(Action::Toggle, get_rect(r[1], r[3]), &mut grid),

                    _ => panic!("bad command '{}'", r[0])
                }
            }

            Err(e) => panic!("error: {}", e)
        }
    }

    let mut brightness = 0;
    let mut count = 0;

    for r in grid.iter() {
        for l in r.iter() {
            brightness += *l;

            if *l > 0 {
                count += 1;
            }
        }
    }

    println!("Total brightness: {} ({} lights turned on)", brightness, count);
}
