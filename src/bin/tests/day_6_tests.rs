use super::*;

#[test]
fn when_part_one_parsing_input_and_there_is_one_single_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_one.txt";
    let expected = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]];

    assert_eq!(part_one_parse_input(input_file_path), expected);
}  

#[test]
fn when_part_one_parsing_input_and_there_is_one_multi_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_two.txt";
    let expected = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]];

    assert_eq!(part_one_parse_input(input_file_path), expected);
}  

#[test]
fn when_part_one_parsing_input_and_there_are_two_single_line_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_three.txt";
    let expected = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()], vec!["a".to_string(), "e".to_string(), "f".to_string()]];

    assert_eq!(part_one_parse_input(input_file_path), expected);
}  

#[test]
fn when_part_one_parsing_input_and_there_are_two_multi_line_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_four.txt";
    let expected = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()], vec!["e".to_string(), "f".to_string(), "g".to_string()]];

    assert_eq!(part_one_parse_input(input_file_path), expected);
}  

#[test]
fn when_part_one_parsing_input_and_there_are_many_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_full.txt";
    let expected = vec![
        vec!["a".to_string(), "b".to_string(), "c".to_string()], 
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        vec!["a".to_string()],
        vec!["b".to_string()]
    ];

    assert_eq!(part_one_parse_input(input_file_path), expected);
}  

#[test]
fn when_executing_part_one_and_there_is_one_single_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_one.txt";
    let expected = 3;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_executing_part_one_and_there_is_one_multi_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_two.txt";
    let expected = 3;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_executing_part_one_and_there_are_two_single_line_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_three.txt";
    let expected = 6;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_executing_part_one_and_there_are_two_multi_line_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_four.txt";
    let expected = 6;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_executing_part_one_and_there_are_many_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_full.txt";
    let expected = 11;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_part_two_parsing_input_and_there_is_one_single_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_one.txt";
    let expected = 
    vec![
        vec![
            vec![
                "a".to_string(), 
                "b".to_string(), 
                "c".to_string()
            ]
        ]
    ];

    assert_eq!(part_two_parse_input(input_file_path), expected);
}

#[test]
fn when_part_two_parsing_input_and_there_is_one_multi_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_two.txt";
    let expected = 
    vec![
        vec![
            vec![
                "a".to_string()
            ], 
            vec![
                "b".to_string()
            ], 
            vec![
                "c".to_string()
            ]
        ]
    ];

    assert_eq!(part_two_parse_input(input_file_path), expected);
}

#[test]
fn when_part_two_parsing_input_and_there_are_two_single_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_three.txt";
    let expected = 
    vec![
        vec![
            vec![
                "a".to_string(), 
                "b".to_string(), 
                "c".to_string()
            ]
        ], 
        vec![
            vec![
                "a".to_string(), 
                "e".to_string(), 
                "f".to_string()
            ]
        ]
    ];

    assert_eq!(part_two_parse_input(input_file_path), expected);
}

#[test]
fn when_part_two_parsing_input_and_there_are_two_multi_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_four.txt";
    let expected = 
    vec![
        vec![
            vec![
                "a".to_string()
            ],
            vec![
                "b".to_string()
            ],
            vec![
                "c".to_string()
            ]
        ], 
        vec![
            vec![
                "e".to_string()
            ],
            vec![
                "f".to_string()
            ],
            vec![
                "g".to_string()
            ]
        ]
    ];

    assert_eq!(part_two_parse_input(input_file_path), expected);
}

#[test]
fn when_part_two_parsing_input_and_there_are_many_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_full.txt";
    let expected = 
    vec![
        vec![
            vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string()
            ]
        ], 
        vec![
            vec![
                "a".to_string()
            ],
            vec![
                "b".to_string()
            ],
            vec![
                "c".to_string()
            ]
        ], 
        vec![
            vec![
                "a".to_string(),
                "b".to_string()
            ],
            vec![
                "a".to_string(),
                "c".to_string()
            ]
        ], 
        vec![
            vec![
                "a".to_string()
            ],
            vec![
                "a".to_string()
            ],
            vec![
                "a".to_string()
            ],
            vec![
                "a".to_string()
            ]
        ], 
        vec![
            vec![
                "b".to_string()
            ]
        ]
    ];

    assert_eq!(part_two_parse_input(input_file_path), expected);
}

#[test]
fn when_getting_answer_intersection_for_two_duplicate_vectors() {
    let first = vec!["a".to_string()];
    let second = vec!["a".to_string()];
    let expected = vec!["a".to_string()];

    assert_eq!(get_answer_intersection(first, second), expected);
}

#[test]
fn when_getting_answer_intersection_for_two_differnet_vectors() {
    let first = vec!["a".to_string()];
    let second = vec!["b".to_string()];
    let expected: Vec<String> = vec![];

    assert_eq!(get_answer_intersection(first, second), expected);
}

#[test]
fn when_getting_answer_intersection_for_two_partial_duplicate_vectors() {
    let first = vec!["a".to_string(), "b".to_string()];
    let second = vec!["b".to_string(), "c".to_string()];
    let expected = vec!["b".to_string()];

    assert_eq!(get_answer_intersection(first, second), expected);
}

#[test]
fn when_counting_unanimous_answers_for_group_and_there_is_one_member_line() {
    let group = 
    vec![
        vec![
            "a".to_string(), 
            "b".to_string(), 
            "c".to_string()
        ]
    ];
    let expected = 3;

    assert_eq!(count_unanimous_answers_for_group(group), expected);
}

#[test]
fn when_counting_unanimous_answers_for_group_and_there_are_two_agreeing_member_line() {
    let group = 
    vec![
        vec![
            "a".to_string()
        ],
        vec![
            "a".to_string()
        ]
    ];
    let expected = 1;

    assert_eq!(count_unanimous_answers_for_group(group), expected);
}

#[test]
fn when_counting_unanimous_answers_for_group_and_there_are_two_partially_agreeing_member_line() {
    let group = 
    vec![
        vec![
            "a".to_string(),
            "b".to_string()
        ],
        vec![
            "a".to_string(),
            "c".to_string()
        ]
    ];
    let expected = 1;

    assert_eq!(count_unanimous_answers_for_group(group), expected);
}

#[test]
fn when_counting_unanimous_answers_for_group_and_there_are_three_disagreeing_member_line() {
    let group = 
    vec![
        vec![
            "a".to_string()
        ],
        vec![
            "b".to_string()
        ],
        vec![
            "c".to_string()
        ]
    ];
    let expected = 0;

    assert_eq!(count_unanimous_answers_for_group(group), expected);
}  

#[test]
fn when_executing_part_two_and_there_is_one_single_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_one.txt";
    let expected = 3;

    assert_eq!(execute_part_two(input_file_path), expected);
}

#[test]
fn when_executing_part_two_and_there_is_one_multi_line_group() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_two.txt";
    let expected = 0;

    assert_eq!(execute_part_two(input_file_path), expected);
}

#[test]
fn when_executing_part_two_and_there_are_two_single_line_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_three.txt";
    let expected = 6;

    assert_eq!(execute_part_two(input_file_path), expected);
}

#[test]
fn when_executing_part_two_and_there_are_two_multi_line_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_four.txt";
    let expected = 0;

    assert_eq!(execute_part_two(input_file_path), expected);
}

#[test]
fn when_executing_part_two_and_there_are_many_groups() {
    let input_file_path = "src/bin/tests/inputs/day_6/input_example_full.txt";
    let expected = 6;

    assert_eq!(execute_part_two(input_file_path), expected);
}