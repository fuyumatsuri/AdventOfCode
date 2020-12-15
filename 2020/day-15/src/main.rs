use std::collections::HashMap;

static _SAMPLE_INPUT: &str = "0,3,6";

static _INPUT: &str = "6,19,0,5,7,13,1";

fn main() {
    let game = _INPUT
        .split(',')
        .map(|number| number.parse::<usize>().unwrap())
        .enumerate()
        .map(|(i, val)| (val, i));

    let prev = game.clone().last().unwrap().0;

    let mut game = game.collect::<HashMap<usize, usize>>();

    let size = 30_000_000;
    let result = (game.len()..size).fold(prev, |prev, i| {
        i - 1 - game.insert(prev, i - 1).unwrap_or(i - 1)
    });

    println!("{:#?}", result);
}
