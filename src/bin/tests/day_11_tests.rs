use super::*;

#[test]
fn when_executing_part_one_example() {
    let input = "src/bin/tests/inputs/day_11/input_example.txt";
    let expected = 37;

    assert_eq!(execute_part_one(input), expected);
}

#[test]
fn when_executing_part_two_example() {
    let input = "src/bin/tests/inputs/day_11/input_example.txt";
    let expected = 26;

    assert_eq!(execute_part_two(input), expected);
}