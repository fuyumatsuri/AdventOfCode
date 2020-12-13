use itertools::Itertools;

static _SAMPLE_INPUT: &str = "939
7,13,x,x,59,x,31,19";

static _INPUT: &str = "1015292
19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,743,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,x,x,x,x,x,x,x,x,x,x,29,x,643,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23";

fn main() {
    // Part 1
    let start = _INPUT.lines().next().unwrap().parse::<usize>().unwrap();
    let id = _INPUT
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .filter_map(|id| id.parse::<usize>().ok())
        .map(|id| {
            let mut time = id;
            while time < start {
                time += id;
            }
            (id, time - start)
        })
        .sorted_by(|a, b| Ord::cmp(&a.1, &b.1))
        .next()
        .unwrap();
    println!("{:#?}", id.0 * id.1);

    // Part 2
    let ids = _INPUT
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(min, id)| id.parse::<u64>().ok().map(|id| (min, id)));

    // paste into wolfram alpha
    ids.for_each(|(min, id)| {
        print!("(x + {}) mod {} == 0, ", min, id);
    });
}
