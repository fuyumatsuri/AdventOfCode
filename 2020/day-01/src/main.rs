use std::io::Read;
use std::{collections::HashSet, io};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.replace("\r", "");
    let input = input.trim();

    let entries = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<HashSet<_>>();

    let result = entries
        .iter()
        .find_map(|entry1| {
            let result = entries
                .iter()
                .find(|entry2| entries.contains(&(2020i32 - entry1 - *entry2)));
            result.map(|result| (result, entry1))
        })
        .unwrap();

    println!("{}", result.1 * result.0 * (2020 - result.0 - result.1));
}
