use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.replace("\r", "");
    let input = input.trim();

    let mut notes = input.split("\n\n");

    let fields = notes.next().unwrap();
    let mine = notes.next().unwrap();
    let near = notes.next().unwrap();

    let rules = fields.lines().map(|line| {
        let mut split = line.split(':');

        let name = split.next().unwrap();

        let mut ranges = split.next().unwrap().trim().split(' ');

        let f = |range: &str| {
            let mut range = range.split('-');
            let a = range.next().unwrap().parse::<usize>().unwrap();
            let b = range.next().unwrap().parse::<usize>().unwrap();
            a..=b
        };

        let r1 = f(ranges.next().unwrap());
        ranges.next();
        let r2 = f(ranges.next().unwrap());

        (name, r1, r2)
    });

    // let sum = near
    //     .lines()
    //     .skip(1)
    //     .map(|line| line.split(',').map(|val| val.parse::<usize>().unwrap()))
    //     .flat_map(|ticket| {
    //         ticket.filter(|val| {
    //             !rules.clone().any(|(_, r1, r2)| {
    //                 r1.contains(&val) || r2.contains(&val)
    //             })
    //         })
    //     }).sum::<usize>();

    let near = near
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|val| val.parse::<usize>().unwrap()))
        .filter(|ticket| {
            ticket.clone().all(|val| {
                rules
                    .clone()
                    .any(|(_, r1, r2)| r1.contains(&val) || r2.contains(&val))
            })
        });

    let field_count = rules.clone().count();

    let mut possible = rules
        .clone()
        .map(|(name, r1, r2)| {
            (
                name,
                (0..field_count)
                    .into_iter()
                    .filter(|i| {
                        near.clone().all(|ticket| {
                            let val = ticket.clone().nth(*i).unwrap();
                            r1.contains(&val) || r2.contains(&val)
                        })
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    possible.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mine = mine
        .lines()
        .skip(1)
        .map(|line| line.split(',').map(|val| val.parse::<usize>().unwrap()))
        .next()
        .unwrap()
        .collect::<Vec<_>>();

    let mut result = 1usize;
    let mut used = vec![];
    possible.iter().for_each(|(name, p)| {
        let val = p.iter().find(|p| !used.contains(p)).unwrap();
        used.push(val);

        if name.contains("departure") {
            result *= mine[*val];
        }
    });

    dbg!(result);
}
