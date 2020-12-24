use std::{
    collections::HashMap,
    io::{self, Read},
};

#[derive(Debug)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.replace("\r", "");
    let input = input.trim();

    let tiles = input.lines().map(|line| {
        let mut chs = line.chars().peekable();

        let mut result = vec![];
        while chs.peek().is_some() {
            let dir = match chs.next().unwrap() {
                'e' => Dir::E,
                's' => match chs.next().unwrap() {
                    'e' => Dir::SE,
                    _ => Dir::SW,
                },
                'w' => Dir::W,
                _ => match chs.next().unwrap() {
                    'e' => Dir::NE,
                    _ => Dir::NW,
                },
            };
            result.push(dir);
        }
        result
    });
    // .collect::<Vec<_>>();

    let mut map = HashMap::new();
    // map.insert((0, 0), true);

    tiles.clone().for_each(|directions| {
        let mut x = 0;
        let mut y = 0;
        directions.iter().for_each(|dir| {
            let d = match dir {
                Dir::E => (x + 1, y),
                Dir::SE => {
                    if y % 2 == 0 {
                        (x, y + 1)
                    } else {
                        (x + 1, y + 1)
                    }
                }
                Dir::SW => {
                    if y % 2 == 0 {
                        (x - 1, y + 1)
                    } else {
                        (x, y + 1)
                    }
                }
                Dir::W => (x - 1, y),
                Dir::NW => {
                    if y % 2 == 0 {
                        (x - 1, y - 1)
                    } else {
                        (x, y - 1)
                    }
                }
                Dir::NE => {
                    if y % 2 == 0 {
                        (x, y - 1)
                    } else {
                        (x + 1, y - 1)
                    }
                }
            };
            // println!("{:?}, {:?}", dir, d);

            x = d.0;
            y = d.1;
        });

        let tile = map.get(&(x, y)).unwrap_or(&true).to_owned();
        map.insert((x, y), !tile);
    });

    // let count = map.iter().filter(|(_, val)| !*val).count();

    let adjacent_even = [(1, 0), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1)];
    let adjacent_odd = [(1, 0), (1, 1), (0, 1), (-1, 0), (0, -1), (1, -1)];

    let count = map.iter().filter(|(_, val)| !*val).count();
    println!("{:?}", count);

    for _ in 0..100 {
        map.clone().keys().for_each(|(x, y)| {
            let adj = if *y % 2 == 0 {
                &adjacent_even
            } else {
                &adjacent_odd
            };

            adj.iter().for_each(|(adj_x, adj_y)| {
                let tile = (*x + adj_x, *y + adj_y);
                if map.get(&tile).is_none() {
                    map.insert(tile, true);
                }
            })
        });

        let old_map = map.clone();
        old_map.iter().for_each(|((x, y), val)| {
            let adj = if *y % 2 == 0 {
                &adjacent_even
            } else {
                &adjacent_odd
            };

            let black_count = adj
                .iter()
                .filter(|(adj_x, adj_y)| {
                    let tile = (*x + adj_x, *y + adj_y);
                    !old_map.get(&tile).unwrap_or(&true)
                })
                .count();

            if *val && black_count == 2 {
                map.insert((*x, *y), false);
            } else if !*val && (black_count == 0 || black_count > 2) {
                map.insert((*x, *y), true);
            }
        });
    }

    let count = map.iter().filter(|(_, val)| !*val).count();
    println!("{:?}", count);
}
