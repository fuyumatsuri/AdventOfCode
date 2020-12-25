fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut value = 1;
    for _ in 1..=loop_size {
        value *= subject;
        value = value % 20201227;
    }
    value
}

fn find_loop(key: u64, subject: u64) -> u64 {
    let mut value = 1;
    for i in 1.. {
        value *= subject;
        value = value % 20201227;
        if value == key {
            return i;
        }
    };

    return 0;
}

fn main() {
    // let card_pub = 5764801;
    // let door_pub = 17807724;
    let card_pub = 18356117;
    let door_pub = 5909654;

    let a = find_loop(card_pub, 7);
    dbg!(a);
    dbg!(transform(door_pub, a));
}