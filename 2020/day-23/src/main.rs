fn main() {
    let input = "158937462";
    // let input = "389125467";

    let mut cups = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    cups.extend(cups.len() + 1..=1_000_000);

    let mut linked_list = vec![0; cups.len() + 1];

    for window in cups.windows(2) {
        linked_list[window[0]] = window[1];
    }
    linked_list[cups[cups.len() - 1]] = cups[0];

    let mut current = cups[0];
    for _ in 0..10_000_000 {
        let a = linked_list[current];
        let b = linked_list[a];
        let c = linked_list[b];

        let new_next = linked_list[c];
        linked_list[current] = new_next;

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

        let after_dest = linked_list[dest];
        linked_list[dest] = a;
        linked_list[c] = after_dest;
    }

    let next = linked_list[1];
    let next2 = linked_list[next];

    println!("{} * {} = {}", next, next2, next * next2);
}
