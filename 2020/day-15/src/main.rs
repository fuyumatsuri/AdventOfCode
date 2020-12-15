use std::collections::HashMap;

static _SAMPLE_INPUT: &str = "0,3,6";

static _INPUT: &str = "6,19,0,5,7,13,1";

fn main() {
    let game = _INPUT
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .enumerate()
        .map(|(i, val)| (val, i));

    let mut prev = game.clone().last().unwrap().0;

    let mut game = game.collect::<HashMap<u32, usize>>();

    let size = 30000000;
    (game.len()..size).into_iter().for_each(|i| {
        let last = game.get_mut(&prev);

        if let Some(last) = last {
            let pos = *last;
            prev = (i - 1 - pos) as u32;

            *last = i - 1;
        } else {
            game.insert(prev, i - 1);
            prev = 0;
        }
    });

    println!("{:#?}", prev);
}
