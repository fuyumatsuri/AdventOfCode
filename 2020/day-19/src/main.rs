use std::{collections::HashMap, str::Chars};

static _SAMPLE_RULES: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"#;

static _SAMPLE_INPUT: &str = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

static _RULES: &str = r#"50: 5 18 | 61 39
79: 118 18 | 125 39
102: 38 39 | 19 18
114: 39 38 | 18 100
119: 39 103
40: 39 72 | 18 38
10: 18 125 | 39 132
44: 51 39 | 118 18
18: "a"
55: 18 33 | 39 4
101: 18 100 | 39 100
28: 110 39 | 96 18
11: 42 31
92: 26 39 | 110 18
2: 18 93 | 39 44
27: 73 18 | 135 39
0: 8 11
8: 42
14: 18 18
41: 109 39 | 58 18
4: 64 39 | 100 18
85: 65 18 | 27 39
122: 39 39 | 18 66
123: 103 39 | 72 18
72: 18 39 | 66 18
130: 18 132 | 39 125
135: 39 74 | 18 32
67: 18 54 | 39 20
133: 39 105 | 18 128
116: 71 18 | 14 39
88: 18 84 | 39 124
111: 100 39 | 100 18
1: 38 18 | 122 39
51: 18 39 | 18 18
109: 16 39 | 87 18
25: 18 56 | 39 97
78: 18 35 | 39 41
12: 18 43 | 39 101
29: 39 37 | 18 10
134: 18 122 | 39 71
5: 39 72 | 18 14
96: 18 125 | 39 14
47: 39 121 | 18 15
63: 13 18 | 1 39
16: 14 18 | 72 39
43: 100 18 | 38 39
115: 18 100
54: 51 66
105: 39 51 | 18 132
107: 82 39 | 50 18
87: 122 39
76: 14 18 | 48 39
30: 39 64 | 18 100
17: 104 39 | 107 18
95: 39 47 | 18 63
34: 134 39 | 130 18
24: 19 18 | 122 39
106: 39 100
121: 38 18 | 71 39
126: 75 39 | 43 18
23: 18 48 | 39 125
90: 18 127 | 39 20
56: 62 39 | 30 18
89: 49 18 | 22 39
113: 87 39 | 59 18
48: 39 39 | 18 39
49: 72 39 | 125 18
68: 18 3 | 39 80
112: 69 18 | 17 39
71: 66 66
58: 114 39 | 111 18
104: 18 90 | 39 2
86: 39 91 | 18 9
99: 18 100 | 39 71
19: 18 18 | 39 66
42: 18 83 | 39 112
31: 85 18 | 94 39
7: 4 39 | 24 18
97: 57 39 | 117 18
75: 100 39 | 122 18
37: 100 18 | 103 39
117: 39 14 | 18 100
129: 103 18 | 125 39
32: 36 18 | 40 39
60: 48 39
128: 18 14 | 39 19
61: 100 39
84: 98 39 | 76 18
93: 39 71 | 18 51
26: 18 72
83: 18 131 | 39 78
59: 18 125 | 39 48
33: 18 71 | 39 48
3: 18 99 | 39 23
20: 39 122 | 18 72
9: 39 133 | 18 108
6: 75 18 | 123 39
52: 39 81 | 18 67
62: 18 125 | 39 72
108: 18 13 | 39 120
81: 18 79 | 39 129
77: 92 18 | 6 39
124: 119 18 | 60 39
82: 116 18 | 10 39
110: 71 39 | 48 18
98: 132 18 | 14 39
74: 18 70 | 39 115
91: 39 21 | 18 12
132: 18 18 | 39 18
100: 39 18
53: 18 29 | 39 34
125: 39 39 | 18 18
131: 39 77 | 18 46
103: 66 39 | 39 18
65: 53 18 | 52 39
38: 39 39 | 39 18
39: "b"
21: 99 18 | 102 39
36: 103 39 | 125 18
73: 18 7 | 39 89
118: 18 39
57: 14 18 | 64 39
35: 39 55 | 18 28
66: 18 | 39
94: 39 86 | 18 45
80: 106 39 | 134 18
127: 39 14 | 18 48
64: 39 18 | 18 39
69: 95 39 | 25 18
45: 88 39 | 68 18
15: 39 38 | 18 71
70: 18 132 | 39 118
13: 100 18 | 132 39
46: 18 126 | 39 113
120: 39 71 | 18 125
22: 39 100 | 18 48"#;

static _INPUT: &str = "abbbbabbbaabababaaabbaabbbbbbbba
bbbabbbbabbabbbbabbbabab
aabbababbbbababbaaabbabb
aababbaaaabbbaaaaabaabbb
aaaabaaababbbbbbbbbbabaa
bbbbbbbaaabbabbbabbaabba
abaaaabbbabaabaaababbbbb
bbababbbaaaaaaabbbababba
bbbaaaaabbabbaabbbbbabaa
aaaabbababbbabbbbbabaabbaaaabbbb
aaaabaaaaabbbbbbbaaaabba
aabaaaabaababbbabababbbbbbbbabab
bbaabbababaaababbababbba
bbabbbaabbabbbbaababababbbaabbbaabbaabbabaaaaaaaabaababa
bbbbbababbaabbabbaaaaaba
babaabbbbbaabbbaaababbaaaaabbbabababbaaa
baaaaaabaaaababbbbbbaabbbaaabaaa
ababbbabababababbbbabaab
bbaaaabbababaaabaaaababbbababaaabbaaaaabbbbaabbbaabaabbbbababaabbbbaaababbbbaabb
bbaaabaaaaaabaaabbbabaaa
aaaaaaabbbaababaaaaaabaa
baabaaaabbbabbaabbaabaaa
aabbbbbbbbaaabbbaabbaaab
bbbaabbabaabbbbabbaabbaa
aababaababababaababbbbaa
aaaaabbaabaababbbbbabbbaabaaabaaaabbaaaaabbbababbaabaabaabbaababbbaaabbbbbbaabaa
abbbbabaaaaababaababaaaaaabbbbba
aabaaaabbaabaabbbbbabbbaabbbbaaaaaaabbaa
ababbbaaaabababaababaabaabbabbbbbaaaabbbbbaaaabbaabaaaaabbbbbbabbaabbababbabaaba
bbbabbabbaabaaaaaabbabaaabbabbaabbbbbaaa
bbbbbbbbbababbababbbbaaabaaabaaa
baaabbbabababaaaaabbaaba
bbabaaabbbabbbbabaababbb
babaabbbaaabaabbbaabababbbbabbbbabbaaaaabbbbbaab
aaabaabbabbbbbbabbbbbbbb
abbababababbbbbabaaabbaa
aaaaababababaabbbaabbbbb
aabbabbbaababbbbbbaaaabbbbaaaabaaabbbababaabbbbbaaabbabb
aabbbbbbabbbabaaaaaabbaa
aabaaabbbbbbbababbbbaaab
aabbbbbbbabaababbaaaabaaabaabbaa
bbababbbbbbabbaaaababbba
aaabaaaaabaaababbaabbaab
babbabbbbabaabbaabaabbbbbabaaaaa
abbbbbbabbaaaabbbaaaaaba
aabbbbbbababaaababbaabbbabbababb
aaaababbaaaaababbbaabaab
baaaababbaaabbabbbabbaab
aabbbbaaababbbaaabaababbbbbbabab
bbbabbbbbbbabbabbbabbaaaabaabbba
babaabaaaababaaaaaaaaaba
babbbbbbbbabbbbabaaabaab
baaababaaaabaabbbbbaaaba
bbbbaabaaaaabaaabbaaaabbbabaabbbabbbaaaa
bbababbbbbaaaaabbbaaaaabaaaabbbabbabbbab
baaaaaabaaabbbbaaabababaabbababb
babbabbabbabbbaaabaaaaba
bbbaaaabbbbabbaaabbaabababbbbabaabaabaababbbbabb
babbbbbabbabaabbbaaaabba
aaabbbbbbbabaaababbaaaaa
baaabbbbabaabbabababbbba
abbbbbbaaabaabaaaabbaabbbaabbbbaabbbabbbbabbabbabbbbbabb
bbaabababbaaabaaaaaaaaba
babaababbbaabbabbbabaaaababaabbabbabbabaaabbbabb
baabaaaabbbaabbbbababbab
aaaabaaaaababbaaabaaaabbaaabbaab
abaaaabbbaabababaabbabba
aababaaabbbbbbbaaabbaaba
ababbabbbaaaaabababbaabaaabbbbaaabbabbabaaaaaabbbaabaababaababaaaaababbbabbabaab
ababaaaabbaaaaabaaaaaaaaabaaabaaabaaaaba
babbbaabababababbabaaaaabbbabbbabaaababbabbabaababbaaabb
aaaaaaabbbaaabbbbbababaa
aaabaaaabaaabbbbbaaaababbaaaabababbbaabaaaaaaaba
abbbabaababaabbabaababbbaaaabbaabaabbaaa
abaabbbbabbaababababaabbaabbabba
aaabababaaaaabbbbabbbaab
aabaaabbabbbbbbabbbbabbbaaabbbabbbbbbbaabaabbbbbabbbaabaaabbbbabbbaabbab
bbaaaababbaaabaabbbbabbaaaaababbaaaabbbb
bbaaabaaaabbbbaaaaabaaab
baabaaaaabbbaaaabbbbabaa
ababbabaabbbbbabbaaaaabbbbbababaaaababbababbaabb
bbaabbbabbaabbabaabbaaaa
abbaabbbbbbaabbbbaaaaaaa
ababababbbabaaaaabababba
abbbbbbababaaabababbbaaa
baabbbbabaabbbbaabbabaaa
aababbbbaabaabbaaabbbaba
bbbbbabaababbbabababbaab
abbbbbababbbaaaabaabbaab
bbaabbabaababaababbabbaa
aabaaabaaaabbbbbabbbbbabaabbabaabbaaabbabaabbabbbabbaaabbaabbaab
bbbbbaaabbbabaaaaaabbaabababbaaaabbbbabb
aabbbaaababbaaaababababaabbabbbbbababababbbbbbaa
babbabbbababaabaabbbbbababbbbbbaaaaaabba
abaaaabbbbbaabbaabaaababaaaabbbabaababaaaabbaaababbbbaab
aabbbaabaabbbbaabaabaabbababbbba
babbaabbaaaababbabaaaaab
bbbaaaabbabaaabaabbaabaa
baabababbbbbbabababbbbaaabaaaaba
babbabbababbaaaabbbbbbbabbabaaaaaabababaaaababaabbbaabaabbbbaabb
bbaabbabbabbaabbaababbaaabbbbabbbaabbabb
abbaaabbaaaababaaaababbbbaaaabbaabbaabaa
baababbaabababbbbbbabbaabaaaaaab
bababababbabbbaababbaaba
bbbaabbabaaabbbbbbbbaabaabaababbaababaaaabbaaabababbbabbbaaabbaa
bbbbabbabbaaaabbbbbbaaaa
ababaaababaabbabaaaabbbabaabbaabbbabbaaaabbabaababbaaabbbaaaabbababbabaabbaaabbabaaabbaaaaababab
abbbbabaaabababbbabbbabb
babbabaaabaaababaababbba
baabbababbaaabbbaaaababbbbbababbabbaabaaaaabaaabbbbaabaaaaaaabbaaaabbbbbaaabaaba
bbaaaabbbabbaaabaaaaaabbaaaabbbb
baaabbabaababbbbbaaabbaa
abbababaabaabbbbbabbaabbababbabbaabbaabbabbabaabbbbbbaab
bbbabbbbbabaabbaaabbabaaaaababaa
ababaaabbbaababaaabbbbbbaababaaaaaababaabaaaaabbbabbbabb
aabbababaaabbbbaaaaaaaba
bbababbbbabbbbabbbbbbaab
babaababaaababbababbaaab
bbaaabbbbbaaabbaaaaaaaaa
aabaaabaabbbabaaaaaababbaaabbbbbaaabaaababaaaaab
babaaabbbbaabbabbabbaaaabbabbbbaabbabbbbbbbababa
abbbbbabbabbabaababaababaaaaababbabbaabbbabbaaaaaaaabaab
aaaabbbaabaaabbaabbaabaa
aaaababbbbaabbbabbabaaba
abbbaaaabbaaaabbbbabbbaabababaaaabaabbbbbbbbbbaaaaabbabb
aaaaaaabaabaaaabbaaaabba
bbbaabbabbbaaaabaabbabbbbaabbabb
abbaababaabaabaabbbabaab
bbabaaabbabbbbbaaabbaaba
bbaaabbbababaabbababbabbbbabaaababbaaabaababbbbbbbababba
babaababbabababbbbababaa
aaabbbbbbbaaabaaabbbbaab
abbbaaaabbaaaabbbababaab
bbabbaabbabbaabbbaaabaab
aaabaaaaaabbbbbbaabababaabbbbabaaaabaaba
aabbbbaaaaabbbbaabababba
aaaaaaabbabaabababbabaaa
bbbbbbbaababaaaaaabbabbbababaabaaaaabbbb
baaabbbaaaabbbaabbbbaaaaaaabaaba
bbbbbbbaaabaaabaaabaabaaabbaaababaaabaaa
aabbbaabaabaabaababbaaababbbabbbabaaaababbabbbaaaaaaaababbaaaaba
babababaaabbabbbbabbabbaabbaaabbbbabaabababbabab
ababababbbaabbbababbbbbabbbbabbb
bbaaabaababbabaabbbababa
bbbbbaaababbbbbaaaabbaabbaabbaabbbbabaaa
bbabbbbbaaabbababbaaaababbaabbaabbbbbbaa
aababbaaaaaabbbaabbabbba
ababaaabbaaabbbbaabbbbab
bbaaaabaaaaaaaabbabbbabb
abbaaaaaaaaaabbaabbbbaba
bbbabbabbbbaaaabbaaaaaaa
baaabbbbbbbabaababbaaaaabaababbbaaaaabababaaabaaaaabbaaabaababba
aababbaaaabbbbbbababbabbbbbabaaa
ababaaaaaabbbaababaaabbbbbbaaabb
abaaaabbabaaababbbababba
babbaaaababaabbaaabaabbabbaabaaaababbaab
bbbabbabbabbabbaaaababaa
bbabbbaaababaabababbbaaa
aabbababbbbabbbaaabbabbbbbabbaaabbbaaabbaaabaaba
babbabbaaaabbbbababbbaaa
babbaaaabbbabbbbbaaabaab
abbaaabbbbbaabbabaabbbbabaaabbaa
bbababbbaabbaabbabbbbbab
aaabbbbbaabbaabbaabbbbba
baabbbaabbbbbabaabbabaab
bbaaabbabaaabababbbaaaba
bbabbaabababaabbbbbbaabb
aaaaababbbabaaaabbabaaaaaaaabbbaabbbbbbb
abaababbbbbabaababbbaabaaaabbbabaaaabbbbbbaabbaaababbaaa
aabaabbaabaababbababbbba
baabbbbababbbbabbabbbaaa
bbbaaaababaaaabbbbbbbabb
bbbbabbabaaaabaaabbaabbbaababbbbaaaaabba
bbbbabbaaabbaaabbababbabbaababba
babaaababaaaaaabbbabaabbbabaaabababaabbbbbbabaab
babbaaaabbaaabaabaabaaba
aabbaabbaaababbbbbabaaba
abaaaabbaaaaaaabaaabbbbaabababaa
abbbbbabbabaaabaaabbaaaa
aabbbaaabaabaaaaabaabbba
baabaabbbabbaaaabbababab
abbaababbbaabaabbaaabaaabaaaabbaaabaaabbbaabababaabbbabb
babbbbbaaabbabbbabbbbbbababbabbaaaaabbaa
baabaaaaabbaabbbbabbbbbbbbaaaabbbaaabbbaabbaabaa
bbbabbbbbbaaaababbbbbbbaabbbabbbaabbbaba
bbababbbabaabbabbbababba
aabbbaabbbbabbaabbbabaab
abaababbababaaabbbbbbbab
aababaaabbaabababbababaa
abaabbbbbbbbbbbabaaabaab
aaabaabbaababbaaaabbbabb
ababbbabbabaabbbabababaa
abaaabaaaabbabbabbbbabab
aaaabbbabbbaaaabaaaaaaba
aababbbbbbaababaaababbba
bbaaababbaabbbbabaabbaaa
bbabaaabbbaaabbbababaaabaaaaaaababaabaaaaabbaaba
aabbabbbbabaabaaabbbbbabbbabbbbbbabaabbabaababba
bbabbaababaabbbbbaabbbbababaaababbaabbbbbbbaaaba
bbbbabbaabaabbbbaabaaaaa
aaaabbbabbabbbbbaabbabba
abbababaaaaabbabbbaaabbaaaaaababbbabaabbaabaabbb
bbaabbbababaabbabaabaabbaabbbbabbaabbbbb
ababaaabbbabbbaaaabaaabbbbbbaaababbbaaab
aaaabbabbaabaabbabababaa
abaaaabbaababbbbbababbbb
babababaabbaabbbaabaabab
bbaaaabbaabbabbbabaaaabbaaabbaababbaabba
babaaabbaababbbbaaaabaaabbabaaba
abababbbbbbabbaabaabbbbaabaaaabbbabababaaaabbbab
aaabbbbbaaababbabaaaaaabaabbbabaabbbaaab
aababaabaaaabaaabbbbaaaa
bbaaaabaababaaabababbabbbbabbaabbababaabaababbbaabbaabba
aabaabbbbaaabbbbaaaaababaabbabbabababaaaabbbbababaaaaaabaaaaabaabbbbaabbaaabbaaa
abbababbbbaabbbabbabaababbabbbaaabbbabbbaaabaaba
bbaaaabaababaabbbbbbbbbaaabbaaaa
abbabbabbabababbaabbbabb
bbabbaabbbbbaabaababaaaababbabab
aaaaaaabaababaababbaabaa
abaaabaaabbaaabaaabaaaaa
babaaababaaabbbabaabbbbaabaabbabababbbaabbbbaaaa
abaaabbbababbbabbabaabaaabaaabaa
bbbaaaabaaaaabababbbbbaa
abbabababaaaabbbbbababab
aaabbbbaaaaaababbabbbbbaabbabaaaaabbbaabbbabaabb
bbabbabaaaaabbbabaaabbbaabaabbba
bbbabbbbabababbbbaaaabbbbaababbb
abaabbbbbaabaaaaabbbaabb
aababbbbbabbabbaababbbba
aabbbbaaaaabbbbaabbbaabb
bbbabbbbbbabbbbbaaababbbbabaabba
baaaabbabbabbaaabbaabaaaaabaabbb
ababbbaabaabaaaabbababab
abaabaaaaababbbaabbbaabaabbababb
abbabbbbaabbabbbbbbbabbabaabbbbaabbabaab
baabbaaababbbbaaaaabbbabaaaaabba
babaabbbaaaabbbaaabbabaabbabbabaababaaabbbaabaaa
abaaaabbaabaabaabbaaababababbaba
bbabbbbbabaababbbaababaa
aababaabababaabbbbabbbbbabbabbababbabbaabbaaaaaaaaabbaaa
bbbaabbababbaabbbbbbbbbb
aaabaabbabbbbbababbabaab
aaabbbbbbbbaabbbaabaabbaabbaabbbaaababababbaabba
baaaabaabbbaabbaaaabbabb
babbaaaaaabababbaaabbbab
aababaaaabaaababbbbbaaab
ababbabbbaabbbbababbabaaababababbbbbaaab
bbaaaabbaabbabaabaaabbabbabbbbabbaaabbabbababaab
aaababbaaabaaaabbabaaaaa
babaaabbbabababbbbbbaabb
aaababbbabbbabbbaaabaaba
abaaabbaabbabbbbbbaabababbababbbaabbbaabbbabababbbaabbaa
bbabbbbabaabbbaaabbaaaba
aababaabaabbbbaabaaabaab
bbbbaabaaaaaababababbbaabbbbbbbb
aaaabaaabbbaababbababbba
bbabbabaaabbbaabaabbbbab
aabababbabbbbbabaaaaabaaabbbaaab
babbbbbbbbaaabaabbbbaaab
ababaaaabaabaabbabababaa
aabababbbbaabbabaabaabbaabbbaabbabbaabaaababbbba
abaabbbbbababbaaabbbbabb
abbaababaabbbbabbbbbaaabaabbbababaaabbbabaabbaab
bbabbababbbabbabbababaaabbaababaaabaabbb
bbaaaaababbaababbbbaabaa
aaababbbbbababbbbbaabbaa
aaaaababbabbabbaaabbaaab
abaabbbbbabbaabbbababaab
aabbbaaabbbaaaababbabaab
abbabaababababbabbabbbaabbbbbaaa
babbabaaabaaababbbbbabbb
aabbbbaaaababbbbabaaabaa
baabaaaaaaabbaabbabaabbbbaababbaaaabbbbababbabaa
abbababababaaabababababbbaabbbbb
abbaababbbabbbaaababbaab
bbabaaaaaabbabaabaabaaba
baabaabbaabaaaababbbabaaaabaabbb
babaabaabbbbabbaabbabbbbabbbaaba
bbabaaabaabaaabbaaaabaab
abbaaabbbbabbbbaabbbbbabbaaaaaabbababbbb
aaabbababbababaaaabbbaba
aaaababbaabbbaaababbabaabbbbaabb
aabaabaabbbbbabaaaaabbabbaababba
aaabbbbbbaababbbabbbbaaabaabbbbabbabaabb
bbaabbbabbbbbbaaabaabbaababaababbababbbabaaaaaaabbaaaaabbaabbaababbbbbba
aaabaabbbaabaaaabbbbabaa
babaabbbaabbbbaaaabaabab
aabaaaabbabaababbaabababbbabbbbbabbbaaba
aabbabbbababaabbbbaabbbababbaababaabbabbabbbbbbb
abbaabbbaaababbbabaaabaa
bbbaabbbabaababbabbbaabb
baaabbbbaababaababaababbbaabaaaaabbaabbbbbababba
aabaaaabababbbaaaaabbbab
bbaaabaabaaaabbbababbaab
aabaaabaabbbabbbbababaaabbaaabbbbbaaaaaabbabbbab
bbaabbbaabaabbbbabbababb
bababaaababababbbaaabbabbbbbbbabaaababaaaaabbaaa
bbbabbbabbbaabbabbbabbbbaabaaaab
aabbababaababababbaabbbabbababbbbaaaabbbbabbbbaa
bbaabbabaaabaaaabbbbbaab
bbbbbbbabaaaababaabaaabbbabaaabbabaaaaba
aaababbbaabbabbbabbbabaaabababbbaabaababaaababab
aaaabbabbbbaaaaaaabbbbbbbbaababaaaabaaaaabaaababbaaaabbaababbbbbbbbababa
aaabbbbaababbabbabbabaab
bbaababbaabbbbabbbababbabbabbaaabaababaaaababbabaaabbbba
baabaaaabbaabbbaaaaaabbb
abbabababbabaaaabbaababb
bbbbabbababbaabbbaaabbbaababaaaabaabbabb
baabbbbbbabbbbaabaabbbab
bbabaabbaabbbaaaaaabaaab
bababbbbabbaababbaaabbaabbabaabbbaaababababbababbbbbbbbaabbbabbbaaaababb
ababababbbbaabbaaabaaabbababaabbbaababaa
abaababbbabbabbababbbabb
bbbabaabbbbaaababababbaaaabbbabbbabaabbbbaaaabaaabbaabbbbaabbbba
abbabbababaaaabbababaabbaabbbbbabaababbb
abaabbbbababaaaaaabababbaabbabba
abaabaaababbaaaaaabbaaaa
bbbbaababaabbaaaabbbbaabbbabbbbababaaaaaaabbabbabaaaaaaaabbbbbab
aabbbbbbbabababbabaaaaba
baaaaaabbbbabbbbaaaaabaa
bbaababaaaaabbbaaabababbaaabbbbbbbaabaaa
aababaaababababaaaaabbabbbbaaaaabaabbabb
abaaabbaabaabaaaabbbabab
abaaabbbabaababbbbaabaaa
ababbbabaabaaaabaababbab
aaaababaabaaaabbabbaaaab
bbaaaababbbbbabaabbbbaaa
bbbababbababaabbaaaabaab
bababaaaabbabbbbabbabbbbabaaababaaabbbbbaababaabbbaabbbb
abbbbabbbbabbaaaaabababbbbabaabb
ababbaabbbabaabbbbaababbbbaabbbbabaabaaa
aaababbabbabbaabbbaabbaa
babababbabaaaabbbbbbbbaa
bbaabbabbbabbbbabbbabbaabaaaaaaaabaababa
aaaababbaabaaabaaaabaaba
baaaaaabbaaabbbaaaaababbaaabbbbbabbabaaa
aabbabbbbbaaabaabaabbbba
aaaabbbabbaaaabbbaabbaaa
abbabbbbabaaaabaabbbbabbabbbbaaaaabaaabbbbbabbbbbabbbbaabbaabbba
bbababbbaabaabaabbbaaaabbbbaaabb
ababaabaaabbabababbbbbaa
abbababaabaaabababbabaaa
babaabaaabaababbbbbabaaa
babbbbbbbabbbbbabbbbaabb
bbbbabbaabbabbbbaaabaabb
bbbaabbbbbaaaababbababbbbbbaaaaabbababab
baabbabaabbbbabbaaaaabba
bbbaaaabbbaabbabbaaabbaabababbbbabbbababaabbabbaaabaabab
bbbabbaababaaabbaababbaabaabaaba
bbbaababbabbbbbbababaabaaababaabaabbabba
abbaababbaabbbaabbaabbaa
babbaabbbabbabaabbbbaabb
aaaabaaaababaaabbbbbaaab
abbbaaaaaabaabaabaababba
aabaaabaaaaababbaaaababbbababbbaaaababaa
aaabbbbabbbabbbabaaaaabb
aabbabababaaabbbabbaaaaa
babaabaabaabaaaabbaaaaabbabbbbaa
bbabbbbabababbbbabababaaabaaaaabbbbbaaab
baababbabbbbabbaaaabbbbbbbabbaabaababbaabaaaaababbabbaaaabababbbbbbbbabbabaaabbbbabbabbb
ababbbaaababbabbbabababbaaabaaaababbababbbaabbaaaaaaabbbbbaababbbbaabbbb
abaabbabaababaababbaaabaaaaabbbbababbbbaababbabb
babbabaaaabaabaababaaabbaaababaa
aaaaababbbbabbaaabbbaaaabbaabbbb
bbbabbbbaabbbaabbbbbabab
babbbbbbbaabbbbaaaabbabb
abbaaabbbbabbaabbbaaaaaa
bbabbaabbabaaababaaaaaabaaabaabaaabbaaab
bbabbabbbbbaaaaabbabaabbabbbbbbaabaabbbbaabaaaaabaaabbaa
bbabbaaaaababbbaaaabbabb
bbaababaabbaaabbbaabbbab
babaaabaaaaaaaaababbbaaa
abbabababaabbbbabbababbbabbabaab
bbbaaaabbabbbbbbabbaabaa
abbaaabbaaaaaaabaaababaa
baabbbbaabaabaaaabbbbbaa
abbbabbbaabbbbbabbbababa
bbabaaabbabababbbbbaaabb
ababababbabababbabababaa
bbaaabaabbabaaabbaaaaaaa
abbaababbbaaabbaababbbaababbbaba
bbababbbaaaabaaaabaaabaa
abbabbabaaabbbbaababaababbbbabbb
baaababababaabbbabbabababbbbbaaa
aabababababababbabbaabbbbbbbbababaaaabaabbabbabbbabbbabb
baaaabaabbaaaaabaabaaaabaaaabaaaaababbbaabaaaaab
baabbaabbaabbbabbbbabaab
abbbabbbbbbaaaaabababaaabbabbbaabaaabaabbbaababb
bbbbbabaabaaaabbbbbabbaaababaaabbbabaaaaaaaabbbbaabbaaaa
bbbbbbbaaaaaababbabbbaab
abaaabbabbaaababbaaababaababbbabbbaaaababbbbabbbaaaabaab
bbabaabbbabbabaaabaababbaaabaaab
abbababababbabbbabbabbbbaaababbbaabbbbababbbaabb
babbaabbbbbaabbbabbbbaaa
abbbabaabbaaabababababababaaabaa
abbbabaaaaaababbabaaabbaabbbbbbbabbbaaab
aabbbbaabbaaaaabababbabbaaaabbbb
ababaaaabbbbbbbaababaabbbbabbbbabbbaaabaaabbbaba
aaabaabbbbaabbbaabbabaab
bbbaaabaaaaaababaaabaaaababbbbaaababbbaaaaaabbaabaaaabbaaaabbaaaabbabaaaabaaababbabbbaaa
ababaaababbabbbbbbbabbbbabababbbbaabbbab
abbbabbbabbabbbbabaaabbbaaaaaaaa
bbbbbbbaaabbbbaabababababbbbbbbb
abbabbabbabaabbbbbababab
bbbbabbabbbabbbaaabbbbbbbabbbaab
baabbbbabbbbbbbabbbbabab
bbaaabaabbabbaabbbbbbbab
baaabababbbaaaaaaababbbbbbbbabaaaabbaababaababbababbaaab
aababaabbaaabababaabaaab
bbbaaabbbabbabbaabbabbbaabaabbabbabaaaaabbbbaaaabbabaabbabaabaab
babaaabbbabbaabbabaaaaaabbbbbbaaabaaabaa
bbaaabbaabaabaaaabbaabaa
bbaaaabbaabbabbbababbaba
baaaaaaabbabaaabbbbbaaaaaaababaaabbbaaaaaaabaababbabbbbbbbbaabaaaaaaabbaabbababb
ababbbabbbaabababbababab
abbbbababbbabbbbbaabbbaabbbbaabbaaaaabbbaaababab
bbaaaabaaabbbaaababbaaba
aabaabbabbbaabbaababaaababaabaabbaaababb
bbabbaabbbaaaabbaaabbaaa
abbbbababaaabbbbababaaaabbabbbaabaababaabaabbbab
bbbbbabababbaaaababaaabbbabbbaba
baabaaaaabababbbabbaaaab
babbabaababaaabbabaaaaab
bbbaaaaaaaaaabbbbbaaaabaabbbbaaababbaaaa
abbabbbbabaaaabbabaabaaaaaabaaaabbaaaaba
bbbaabbbbbabaabbbbabaaaaaaabaaaabbbabaaa
aaaaababababbbabbbababba
aaababbaababababaaaaaaababbbbaab
aaaabaaaabbababaaaabbabababbaaaaaaabaaabbbaabbbbabbaaaaa
aaaaaaababaabaaababbaaab
ababaaaabaaababaabaabaab
babaabaabaababababbababb
bbbabbbbaabbbaaabbbbbbaa
abbbbbababaaababbabbbbaa
bababbaabbbbbaabbbaaaabbaaabbbaabbaaaabbabbaaabb
baaaabbbbbaaaabbaaabbbaaaabbaaaabababbbb
aaaaaaababbabbabbaabbaab
bbaabbabbbababbbaabbabbbbbabaaba
abaabbabaabbbaabaaababaa
bbbabbbbbbaabababbbbaabaabbbabbbbaaabbaa
aabbaabbbbbabbbbabbabbbbbbbabaabbaababba
babaabaabbaaaabbababaabbbbabaaaa
babbaaaabbbaaaabbbbbbaaa
bbabbabbbbababbaabbbbaabbbbbaabbababababaabbbabbaababbbb
bbabbbbabaaabbbbbaabbaba
aaababbbaabbbaabbaababaaabbbbabbbbbbbbaa
baabbbaabbbbabaaaababbababbabbbaabbababbaabbbaaa
aaaabbbabbaaabbaaaaaabaa
bbbbaababbaabbababaaabbbaaababab
aabbaaaaaabbbabbbbbabaabaabbbbaabaabbbba
aaaabbabababaabbaaabbaab
bbbaaaaaabbbbbabbabaababaabaaabaaaabbbbaaaabbaabababbaababaaabaa
abaabbbbbbbabbbbaaaaabbb
ababaabbaaaaababbaaaababababbaabbbbbaaab
aaabbbbababaaaaabbabbbabaaababaa
bbaabbabaaabbbbbbbabbbbbbbbbaaabababbbbb
abbaabbbaabaaabbbbbbbaaa
abaabbbbaaaababbababaabababbbbaa
aababbbbbaabababbabbabbaaaaabaaabbbbbbaa
babbabbbbbbbabbaaaaaaaabababaabbaabbabababaababa
aabbabaabbaaabbbababaaababaaabbbaaabbbab
aababaaabbbaababaababbbbbbaaaaaa
abbaaabbbaaabbbaababbaba
aaaaaabbbaababbabaabbabbaaaabaabbbabbabaabaaaabbbabbbbababbbabab
aaababbbababbabbbbbaabababbababaababaaaabbaabbaa
bbabaabbbabbbbbaaaaaaaabbabaababaaaaaaab
bbaaabbabaabaabbbaaaaaba
bbbbbabaaaaaababbabababaabaaaaab
bbbaabbbbbababbbaababbab
babaabbaabaaabbbabbbabab
abaaaaaababbaaaaaabbabba
babababbababbbaababaababbabbbbbaabbaabbbbbbbbaaababbbaba
ababaaaaabbabbabaaaaabbaabbabbba
bbaaabaabaaaabaababbbaba
ababbabbababbbaababaababaaaaaaaa
baaabbbbabaaabababbbaabb
aaabaabbababbbabaaaabbbb
babbbbabababababaaabbbbbbabbbabb
aaabbbbbaabaaaabbabababbbbababbaaabaaaaa
babbaabbabbaabbbaababbbb
aababaabbabbaaaaaabbbbba
babaabbaabaabaaabbbbaabababaaabaaabbbaaabbabbabbaababbab
abaabaaabaabbbabbbabaababaaabbaabaababbababaabbabbbbabbbabbabaaabaaaabaaaababbbabbbbabaa
bbbabbbaaaabbbbababaaaaa
bbabbaaabaababbbbaaababbaabbabbbaabbbbbbbabaabaaabaabaaaaababbbb
babbbabaabbabbbbbabbbbabaabbaaaa
ababaaaaabbbbabaababbabbbaaababaaaaabaaabaabbaababbababb
abbabababbbabbbbbbbaaabb
bbabaaaabbbbbbbaaabbbaba
bbaabbbababaababbbbbabbb";

fn eight(input: &str, rules42: &[String], rules31: &[String], first: bool) -> bool {
    if !first && eleven(input, rules42, rules31) {
        return true;
    }

    rules42.iter().any(|rule| {
        if let Some(input) = input.strip_prefix(rule) {
            if !input.is_empty() {
                eight(input, rules42, rules31, false)
            } else {
                false
            }
        } else {
            false
        }
    })
}

fn eleven(input: &str, rules42: &[String], rules31: &[String]) -> bool {
    if input.is_empty() {
        return true;
    }

    rules42.iter().any(|rule42| {
        rules31.iter().any(|rule31| {
            if let Some(input) = input.strip_prefix(rule42) {
                if let Some(input) = input.strip_suffix(rule31) {
                    eleven(input, rules42, rules31)
                } else {
                    false
                }
            } else {
                false
            }
        })
    })
}

fn _go<'a, T>(
    rule: usize,
    rules: &HashMap<usize, (T, Option<T>)>,
    input: &Chars,
) -> Result<usize, usize>
where
    T: Iterator<Item = &'a str>,
    T: Clone,
{
    let check = |mut t: T, mut c: Chars| -> (bool, usize) {
        let mut count = 0;
        (
            t.all(|item| {
                let r = if let Ok(i) = item.parse::<usize>() {
                    let result = _go(i, rules, &c);
                    if let Ok(i) = result {
                        count += i;
                        c.nth(i - 1);
                        true
                    } else {
                        false
                    }
                } else if let Some(ch) = c.next() {
                    count += 1;
                    ch == (item as &str).chars().nth(1).unwrap()
                } else {
                    false
                };

                r
            }),
            count,
        )
    };

    let (first, second) = rules[&rule].clone();

    let first_result = check(first, input.clone());
    if first_result.0 {
        return Ok(first_result.1);
    }

    if let Some(second) = second {
        let second_result = check(second, input.clone());
        if second_result.0 {
            return Ok(second_result.1);
        }
    }

    Err(0)
}

fn matches<'a, T>(rule: usize, rules: &HashMap<usize, (T, Option<T>)>) -> Vec<String>
where
    T: Iterator<Item = &'a str>,
    T: Clone,
{
    let (mut first, second) = rules[&rule].clone();

    let a = first.next().unwrap();
    let aa = if let Ok(i) = a.parse::<usize>() {
        matches(i, rules)
    } else {
        return vec![String::from((a as &str).chars().nth(1).unwrap())];
    };

    let bb = if let Some(b) = first.next() {
        let b = b.parse::<usize>().unwrap();
        matches(b, rules)
    } else {
        vec![String::new()]
    };

    let mut c = aa
        .iter()
        .flat_map(|prefix| {
            bb.iter()
                .map(|postfix| prefix.clone() + postfix)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    if let Some(mut second) = second {
        let a = second.next().unwrap().parse::<usize>().unwrap();
        let aa = matches(a, rules);

        let bb = if let Some(b) = second.next() {
            let b = b.parse::<usize>().unwrap();
            matches(b, rules)
        } else {
            vec![String::new()]
        };

        let mut d = aa
            .iter()
            .flat_map(|prefix| {
                bb.iter()
                    .map(|postfix| prefix.clone() + postfix)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        c.append(&mut d);
    }

    c
}

fn main() {
    // let mut rules = _SAMPLE_RULES
    let rules = _RULES
        .lines()
        .filter_map(|line| {
            let mut a = line.split(':');
            let id = a.next().unwrap().parse::<usize>().unwrap();

            let rule = match id {
                8 => "42 | 42 8",
                11 => "42 31 | 42 11 31",
                _ => a.next().unwrap(),
            };

            let mut b = rule.split('|');
            let first = b.next().unwrap().split_ascii_whitespace();
            let second = b.next().map(|rule| rule.split_ascii_whitespace());

            Some((id, (first, second)))
        })
        .collect::<HashMap<_, _>>();

    let count = _INPUT
        .lines()
        .filter(|line| {
            // Part 1
            // let r = _go(0, &rules, &line.chars());
            // r.is_ok() && r.unwrap() == line.len()

            // Part 2
            let rules42 = matches(42, &rules);
            let rules31 = matches(31, &rules);
            eight(line, &rules42, &rules31, true)
        })
        .count();

    println!("{:?}", count);
}
