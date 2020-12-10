use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

static _SAMPLE_INPUT: &str = "16
10
15
5
1
11
7
19
6
12
4";

static _SAMPLE_INPUT2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

static _INPUT: &str = "56
139
42
28
3
87
142
57
147
6
117
95
2
112
107
54
146
104
40
26
136
127
111
47
8
24
13
92
18
130
141
37
81
148
31
62
50
80
91
33
77
1
96
100
9
120
27
97
60
102
25
83
55
118
19
113
49
133
14
119
88
124
110
145
65
21
7
74
72
61
103
20
41
53
32
44
10
34
121
114
67
69
66
82
101
68
84
48
73
17
43
140";

fn main() {
    let mut input = _INPUT
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let device_volt = input.iter().max().unwrap() + 3;

    input.push(0);
    input.push(device_volt);

    input.sort();
    
    // Part 1
    // input.reverse();
    // let mut diff = HashMap::<i32, i32>::new();

    // (0..input.len()-1).into_iter().for_each(|i| {
    //     *diff.entry(input[i] - input[i+1]).or_insert(0) += 1;
    // });

    // println!("{:#?}", diff[&1] * diff[&3]);

    // Part 2
    // let mut ways = HashMap::<usize,i64>::new();
    let mut ways = vec![0 as u64; input.len()];
    ways[0] = 1;

    input.iter().enumerate().for_each(|(i, volt)| {
        (i+1..=i+3).into_iter().for_each(|j| {
            if j < input.len() && (input[j] - volt) <= 3 {
                ways[j] += ways[i];
            }
        });
    });

    println!("{:#?}", ways.last().unwrap());
}
