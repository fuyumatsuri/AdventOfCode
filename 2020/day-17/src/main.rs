use std::collections::HashMap;

use itertools::Itertools;

static _SAMPLE_INPUT: &str = ".#.
..#
###";

static _INPUT: &str = "###...#.
.##.####
.####.##
###.###.
.##.####
#.##..#.
##.####.
.####.#.";

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Debug, Clone, Copy)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64, w: i64) -> Point {
        Point { x, y, z, w }
    }
}

fn neighbors(point: &Point) -> Vec<Point> {
    let mut vec = vec![];
    for x in point.x - 1..=point.x + 1 {
        for y in point.y - 1..=point.y + 1 {
            for z in point.z - 1..=point.z + 1 {
                for w in point.w - 1..=point.w + 1 {
                    if x != point.x || y != point.y || z != point.z || w != point.w {
                        vec.push(Point::new(x, y, z, w));
                    }
                }
            }
        }
    }

    vec
}

fn main() {
    let mut map: HashMap<Point, char> = HashMap::new();

    _INPUT.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, ch)| {
            let point = Point::new(j as i64, i as i64, 0, 0);
            neighbors(&point).iter().for_each(|p| {
                if !map.contains_key(p) {
                    map.insert(*p, '.');
                }
            });
            map.insert(point, ch);
        })
    });

    (0..6).into_iter().for_each(|_| {
        let old_map = map.clone();
        old_map.iter().sorted().for_each(|(point, ch)| {
            let active = neighbors(point)
                .iter()
                .filter(|p| {
                    if let Some(ch) = old_map.get(*p) {
                        *ch == '#'
                    } else {
                        map.insert(**p, '.');
                        false
                    }
                })
                .count();

            match ch {
                '#' => {
                    if !(active == 2 || active == 3) {
                        map.insert(*point, '.');
                    }
                }
                _ => {
                    if active == 3 {
                        map.insert(*point, '#');
                    }
                }
            }
        });

        let count = map.iter().filter(|(_, val)| **val == '#').count();
        println!("{:#?}", count);
    });
}
