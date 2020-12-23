fn main() {
    let input = "158937462";
    // let input = "389125467";

    let mut cups = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    cups.extend(cups.len() + 1..=1_000_000);

    let mut linked_list = vec![(0, 0); cups.len() + 1];

    for window in cups.windows(3) {
        linked_list[window[1]] = (window[0], window[2]);
    }
    linked_list[cups[0]] = (cups[cups.len() - 1], cups[1]);
    linked_list[cups[cups.len() - 1]] = (cups[cups.len() - 2], cups[0]);

    let mut current = cups[0];
    for _ in 0..10_000_000 {
        let a = linked_list[current].1;
        let b = linked_list[a].1;
        let c = linked_list[b].1;

        let new_next = linked_list[c].1;
        linked_list[current].1 = new_next;
        linked_list[new_next].0 = current;

        let mut dest = current - 1;

        current = new_next;

        if dest == 0 {
            dest = cups.len();
        }

        while dest == a || dest == b || dest == c {
            if dest == 1 {
                dest = cups.len() + 1;
            }
            dest -= 1;
        }

        let after_dest = linked_list[dest].1;
        linked_list[dest].1 = a;
        linked_list[c].1 = after_dest;

        linked_list[after_dest].0 = c;
    }

    let next = linked_list[1].1;
    let next2 = linked_list[next].1;

    println!("{} * {} = {}", next, next2, next * next2);
}
