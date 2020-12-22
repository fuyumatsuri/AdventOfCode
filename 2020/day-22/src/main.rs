use std::{
    collections::HashSet,
    io::{self, Read},
};

use itertools::Itertools;

fn play(mut player1: Vec<usize>, mut player2: Vec<usize>) -> Result<Vec<usize>, Vec<usize>> {
    let mut past = HashSet::new();

    while !player1.is_empty() && !player2.is_empty() {
        if !past.insert((player1.clone(), player2.clone())) {
            return Ok(player1);
        }

        let p1 = player1.remove(0);
        let p2 = player2.remove(0);

        let winner = if p1 <= player1.len() && p2 <= player2.len() {
            play(player1[0..p1].to_vec(), player2[0..p2].to_vec()).is_ok()
        } else {
            p1 > p2
        };

        if winner {
            player1.push(p1);
            player1.push(p2);
        } else {
            player2.push(p2);
            player2.push(p1);
        }
    }

    if !player1.is_empty() {
        Ok(player1)
    } else {
        Err(player2)
    }
}

fn score(player: Vec<usize>) -> usize {
    player
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| card * (i + 1))
        .sum::<usize>()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let input = input.replace("\r", "");
    let input = input.trim();

    let (player1, player2) = input
        .split("\n\n")
        .map(|player| {
            player
                .lines()
                .skip(1)
                .map(|line| line.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .tuples()
        .next()
        .unwrap();

    let r = match play(player1, player2) {
        Ok(p) => score(p),
        Err(p) => score(p),
    };

    println!("{:?}", r);
}
