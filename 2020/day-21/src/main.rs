use std::io::{self, Read};

use itertools::Itertools;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let input_str = input.trim();

    let input = input_str.lines().map(|line| {
        let mut split = line.split('(');
        let ingr = split.next().unwrap().split_ascii_whitespace();
        let aller = split
            .next()
            .unwrap()
            .strip_suffix(')')
            .unwrap()
            .split_ascii_whitespace()
            .skip(1)
            .map(|aller| {
                if let Some(aller) = aller.strip_suffix(',') {
                    aller
                } else {
                    aller
                }
            });

        (ingr.collect::<Vec<_>>(), aller.collect::<Vec<_>>())
    });

    let all_ingr = input
        .clone()
        .map(|(ingr, _)| ingr)
        .flatten()
        .sorted()
        .dedup();
    let all_aller = input
        .clone()
        .map(|(_, aller)| aller)
        .flatten()
        .sorted()
        .dedup();

    let no_aller = all_ingr
        .clone()
        .filter(|check_ingr| {
            all_aller.clone().all(|check_aller| {
                !input.clone().all(|(ingrs, allers)| {
                    !(!ingrs.contains(&check_ingr) && allers.contains(&check_aller))
                })
            })
        })
        .collect::<Vec<_>>();

    // println!("{:?}", no_aller);

    let count: usize = no_aller
        .iter()
        .map(|ingr| {
            input
                .clone()
                .map(|(ingr, _)| ingr)
                .flatten()
                .filter(|i| i == ingr)
                .count()
        })
        .sum();

    println!("{:?}", count);

    // let mut pairs = vec![];
    input.clone().for_each(|(ingrs, _)| {
        let a = ingrs
            .iter()
            .filter(|ingr| !no_aller.contains(ingr))
            .collect::<Vec<_>>();
        println!("{:?}", a);
    });

    let has_aller = all_ingr
        .clone()
        .filter(|ingr| !no_aller.contains(ingr))
        .collect::<Vec<_>>();

    let size = all_aller.clone().count();
    let mut possible = all_aller.clone().permutations(size);

    let allers = possible
        .find(|allers| {
            input.clone().all(|(input_ingrs, input_allers)| {
                input_allers.iter().all(|aller| {
                    // println!("{:?}, ||||, {}", allers, aller);
                    let i = allers
                        .iter()
                        .enumerate()
                        .filter(|(_, a)| &aller == a)
                        .map(|(i, _)| i)
                        .next()
                        .unwrap();
                    input_ingrs.contains(&has_aller[i])
                })
            })
        })
        .unwrap();

    let a = allers
        .iter()
        .zip(has_aller.iter())
        .sorted_by(|a, b| Ord::cmp(a.0, b.0))
        .map(|(_, ingr)| String::from(*ingr) + ",")
        .collect::<String>();

    println!("{:?}", a);
    Ok(())
}
