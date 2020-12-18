use super::*;

// #[test]
// fn when_executing_part_two() {
//     let input = "";
//     let expected = 0;

//     assert_eq!(execute_part_one(input), expected);
// }

#[test]
fn when_parsing_the_ticket_information_and_there_are_only_rules() {
    let ticket_information = vec![
        "class: 1-3 or 5-7".to_string()
    ];
    let expected = (vec!["class: 1-3 or 5-7".to_string()], vec![], vec![vec![]]);

    assert_eq!(parse_ticket_information(ticket_information), expected);
}

#[test]
fn when_parsing_the_ticket_information_and_there_are_only_rules_and_own_ticket() {
    let ticket_information = vec![
        "class: 1-3 or 5-7".to_string(),
        "row: 6-11 or 33-44".to_string(),
        "".to_string(),
        "your ticket:".to_string(),
        "7,1,14".to_string()
    ];
    let expected_rules = vec![
        "class: 1-3 or 5-7".to_string(),
        "row: 6-11 or 33-44".to_string()
    ];
    let expected_own_ticket = vec![7,1,14];
    let expected = (expected_rules, expected_own_ticket, vec![vec![]]);

    assert_eq!(parse_ticket_information(ticket_information), expected);
}

#[test]
fn when_parsing_the_ticket_information_and_there_are_many_rules_and_tickets() {
    let ticket_information = vec![
        "class: 1-3 or 5-7".to_string(),
        "row: 6-11 or 33-44".to_string(),
        "seat: 13-40 or 45-50".to_string(),
        "".to_string(),
        "your ticket:".to_string(),
        "7,1,14".to_string(),
        "".to_string(),
        "nearby tickets:".to_string(),
        "7,3,47".to_string(),
        "40,4,50".to_string(),
        "55,2,20".to_string(),
        "38,6,12".to_string()
    ];
    let expected_rules = vec![
        "class: 1-3 or 5-7".to_string(),
        "row: 6-11 or 33-44".to_string(),
        "seat: 13-40 or 45-50".to_string()
    ];
    let expected_own_ticket = vec![7,1,14];
    let expected_other_tickets = vec![
        vec![7,3,47],
        vec![40,4,50],
        vec![55,2,20],
        vec![38,6,12]
    ];
    let expected = (expected_rules, expected_own_ticket, expected_other_tickets);

    assert_eq!(parse_ticket_information(ticket_information), expected);
}

#[test]
fn when_evaluating_rules_for_invalid_values_and_there_is_no_rules_or_tickets() {
    let rules = vec![];
    let tickets = vec![];
    let expected = vec![];

    assert_eq!(evaluate_rules_for_invalid_values(rules, tickets), expected);
}

#[test]
fn when_evaluating_rules_for_invalid_values_and_there_is_one_rule_and_one_ticket_and_the_ticket_is_valid() {
    let rules = vec!["class: 1-3 or 5-7".to_string()];
    let tickets = vec![vec![2,6,7]];
    let expected = vec![];

    assert_eq!(evaluate_rules_for_invalid_values(rules, tickets), expected);
}

#[test]
fn when_evaluating_rules_for_invalid_values_and_there_is_one_rule_and_one_ticket_and_the_ticket_is_invalid() {
    let rules = vec!["class: 1-3 or 5-7".to_string()];
    let tickets = vec![vec![4,6,7]];
    let expected = vec![4];

    assert_eq!(evaluate_rules_for_invalid_values(rules, tickets), expected);
}

#[test]
fn when_evaluating_rules_for_invalid_values_and_there_many_rules_and_tickets_and_three_tickets_are_invalid() {
    let rules = vec![
        "class: 1-3 or 5-7".to_string(),
        "row: 6-11 or 33-44".to_string(),
        "seat: 13-40 or 45-50".to_string()
    ];
    let tickets = vec![
        vec![7,3,47],
        vec![40,4,50],
        vec![55,2,20],
        vec![38,6,12]
    ];
    let expected = vec![4,55,12];

    assert_eq!(evaluate_rules_for_invalid_values(rules, tickets), expected);
}

#[test]
fn when_evaluating_rules_for_valid_only_tickets_and_there_are_three_rules_and_four_tickets_and_one_tickets_is_valid() {
    let rules = vec![
        "class: 1-3 or 5-7".to_string(),
        "row: 6-11 or 33-44".to_string(),
        "seat: 13-40 or 45-50".to_string()
    ];
    let tickets = vec![
        vec![7,3,47],
        vec![40,4,50],
        vec![55,2,20],
        vec![38,6,12]
    ];
    let expected = vec![vec![7,3,47]];

    assert_eq!(evaluate_rules_for_valid_only_tickets(rules, tickets), expected);
}

#[test]
fn when_executing_part_one() {
    let input = "src/bin/tests/inputs/day_16/input_example.txt";
    let expected = 71;

    assert_eq!(execute_part_one(input), expected);
}

#[test]
fn when_removing_a_rule_from_positions_and_there_is_one_position_with_three_rules() {
    let rules_for_positions = vec![
        vec![
            "rule_1".to_string(),
            "rule_2".to_string(),
            "rule_3".to_string()
        ]
    ];
    let rule = "rule_2".to_string();
    let expected = vec![
        vec![
            "rule_1".to_string(),
            "rule_3".to_string()
        ]
    ];

    assert_eq!(remove_rule_from_positions(rule, rules_for_positions), expected);
}

#[test]
fn when_removing_a_rule_from_positions_and_there_many_positions_with_varied_rules() {
    let rules_for_positions = vec![
        vec![
            "rule_1".to_string(),
            "rule_2".to_string(),
            "rule_3".to_string()
        ],
        vec![
            "rule_2".to_string(),
            "rule_6".to_string()
        ],
        vec![
            "rule_3".to_string(),
            "rule_1".to_string()
        ],
    ];
    let rule = "rule_2".to_string();
    let expected = vec![
        vec![
            "rule_1".to_string(),
            "rule_3".to_string()
        ],
        vec![
            "rule_6".to_string()
        ],
        vec![
            "rule_3".to_string(),
            "rule_1".to_string()
        ],
    ];

    assert_eq!(remove_rule_from_positions(rule, rules_for_positions), expected);
}

// #[test]
// fn when_executing_part_two() {
//     let input = "src/bin/tests/inputs/day_16/input_example_two.txt";
//     let expected = 71;

//     assert_eq!(execute_part_two(input), expected);
// }