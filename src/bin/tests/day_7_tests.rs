use super::*;

#[test]
fn when_parsing_input_and_there_is_one_line() {
    let input_file_path = "src/bin/tests/inputs/day_7/input_example_one.txt";
    let expected = vec!["light red bags contain 1 bright white bag, 2 muted yellow bags."];

    assert_eq!(parse_input(input_file_path), expected);
} 

#[test]
fn when_parsing_input_and_there_many_line() {
    let input_file_path = "src/bin/tests/inputs/day_7/input_example_full.txt";
    let expected = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags."
    ];

    assert_eq!(parse_input(input_file_path), expected);
} 

#[test]
fn when_parsing_lines_and_there_is_only_a_parent_bag() {
    let input = "faded blue bags contain no other bags.".to_string();
    let mut expected = HashMap::new();
    expected.insert("faded blue".to_string(), HashMap::new());

    assert_eq!(parse_lines(vec![input]), expected);
}

#[test]
fn when_parsing_lines_and_there_is_a_parent_bag_with_two_child_bags() {
    let input = "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string();
    let mut expected = HashMap::new();
    let mut children = HashMap::new();
    children.insert("bright white".to_string(), 3);
    children.insert("muted yellow".to_string(), 4);
    expected.insert("dark orange".to_string(), children);

    assert_eq!(parse_lines(vec![input]), expected);
}

#[test]
fn when_parsing_lines_and_there_are_multiple_parent_bags_with_children_bags() {
    let input_vector = vec![
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string()
    ];
    let mut expected = HashMap::new();
    let mut children1 = HashMap::new();
    children1.insert("bright white".to_string(), 3);
    children1.insert("muted yellow".to_string(), 4);
    let mut children2 = HashMap::new();
    children2.insert("shiny gold".to_string(), 2);
    children2.insert("faded blue".to_string(), 9);
    let mut children3 = HashMap::new();
    children3.insert("dark olive".to_string(), 1);
    children3.insert("vibrant plum".to_string(), 2);

    expected.insert("dark orange".to_string(), children1);
    expected.insert("muted yellow".to_string(), children2);
    expected.insert("shiny gold".to_string(), children3);

    assert_eq!(parse_lines(input_vector), expected);
}

#[test]
fn when_parsing_children_bags_and_there_is_no_child_bags() {
    let input = "no other bags.".to_string();
    let expected = HashMap::new();

    assert_eq!(parse_children_bags(&input), expected);
}

#[test]
fn when_parsing_children_bags_and_there_is_one_child_bag_of_quantity_one() {
    let input = "1 shiny gold bag.".to_string();
    let mut children = HashMap::new();
    children.insert("shiny gold".to_string(), 1);
    let expected = children;

    assert_eq!(parse_children_bags(&input), expected);
}

#[test]
fn when_parsing_children_bags_and_there_is_three_child_bags_of_various_quantities() {
    let input = "5 shiny beige bags, 2 shiny silver bags, 5 dull fuchsia bags.".to_string();
    let mut children = HashMap::new();
    children.insert("shiny beige".to_string(), 5);
    children.insert("shiny silver".to_string(), 2);
    children.insert("dull fuchsia".to_string(), 5);
    let expected = children;

    assert_eq!(parse_children_bags(&input), expected);
}

#[test]
fn when_executing_part_one_example() {
    let input_file_path = "src/bin/tests/inputs/day_7/input_example_full.txt";
    let expected = 4;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_getting_child_bag_counts_and_there_are_no_child_bags() {
    let mut input_map = HashMap::new();
    input_map.insert("dark orange".to_string(), HashMap::new());
    let expected = 0;

    assert_eq!(get_child_bag_count(input_map, "dark orange".to_string()), expected);
}

#[test]
fn when_getting_child_bag_counts_and_there_is_one_child_bag_of_count_one() {
    let mut input_map = HashMap::new();
    let mut child = HashMap::new();
    child.insert("shiny beige".to_string(), 1);
    input_map.insert("dark orange".to_string(), child);
    input_map.insert("shiny beige".to_string(), HashMap::new());
    let expected = 1;

    assert_eq!(get_child_bag_count(input_map, "dark orange".to_string()), expected);
}

#[test]
fn when_getting_child_bag_counts_and_there_are_two_child_bags_of_count_two_each() {
    let mut input_map = HashMap::new();
    let mut children = HashMap::new();

    children.insert("shiny beige".to_string(), 2);
    children.insert("dotted black".to_string(), 2);
    input_map.insert("dark orange".to_string(), children);

    input_map.insert("shiny beige".to_string(), HashMap::new());
    input_map.insert("dotted black".to_string(), HashMap::new());

    let expected = 4;

    assert_eq!(get_child_bag_count(input_map, "dark orange".to_string()), expected);
}

#[test]
fn when_getting_child_bag_counts_and_there_are_two_child_bags_with_two_children_each_count_two_each() {
    let mut input_map = HashMap::new();
    let mut children = HashMap::new();

    children.insert("shiny beige".to_string(), 2);
    children.insert("dotted black".to_string(), 2);
    input_map.insert("dark orange".to_string(), children);

    let mut sub_child_one = HashMap::new();
    sub_child_one.insert("muted yellow".to_string(), 2);
    sub_child_one.insert("shiny gold".to_string(), 2);

    let mut sub_child_two = HashMap::new();
    sub_child_two.insert("muted yellow".to_string(), 2);
    sub_child_two.insert("shiny gold".to_string(), 2);

    input_map.insert("shiny beige".to_string(), sub_child_one);
    input_map.insert("dotted black".to_string(), sub_child_two);
    input_map.insert("muted yellow".to_string(), HashMap::new());
    input_map.insert("shiny gold".to_string(), HashMap::new());

    let expected = 20;

    assert_eq!(get_child_bag_count(input_map, "dark orange".to_string()), expected);
}

#[test]
fn when_getting_child_bag_counts_and_there_are_many_child_bags_with_many_children_of_various_counts() {
    let mut input_map = HashMap::new();
    let mut children = HashMap::new();

    children.insert("shiny beige".to_string(), 4);
    children.insert("dotted black".to_string(), 7);
    input_map.insert("dark orange".to_string(), children);

    let mut sub_child_one = HashMap::new();
    sub_child_one.insert("muted yellow".to_string(), 1);
    sub_child_one.insert("shiny gold".to_string(), 8);

    let mut sub_child_two = HashMap::new();
    sub_child_two.insert("muted yellow".to_string(), 3);
    sub_child_two.insert("shiny gold".to_string(), 4);

    let mut sub_sub_child_one = HashMap::new();
    sub_sub_child_one.insert("vibrant plum".to_string(), 5);

    input_map.insert("shiny beige".to_string(), sub_child_one);
    input_map.insert("dotted black".to_string(), sub_child_two);
    input_map.insert("muted yellow".to_string(), HashMap::new());
    input_map.insert("shiny gold".to_string(), sub_sub_child_one);
    input_map.insert("vibrant plum".to_string(), HashMap::new());

    let expected = 396;

    assert_eq!(get_child_bag_count(input_map, "dark orange".to_string()), expected);
}

#[test]
fn when_executing_part_two_example() {
    let input_file_path = "src/bin/tests/inputs/day_7/input_example_full.txt";
    let expected = 32;

    assert_eq!(execute_part_two(input_file_path), expected);
}