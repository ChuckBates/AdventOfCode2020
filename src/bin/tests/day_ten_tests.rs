use super::*;

#[test]
fn when_executing_part_one_for_small_example() {
    let input_file_path = "src/bin/tests/inputs/day_ten/input_example_small.txt";
    let expected = 35;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_executing_part_one_for_example() {
    let input_file_path = "src/bin/tests/inputs/day_ten/input_example.txt";
    let expected = 220;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_executing_part_two_for_small_example() {
    let input_file_path = "src/bin/tests/inputs/day_ten/input_example_small.txt";
    let expected = 9;

    assert_eq!(execute_part_two(input_file_path), expected);
}

#[test]
fn when_executing_part_two_for_example() {
    let input_file_path = "src/bin/tests/inputs/day_ten/input_example.txt";
    let expected = 19208;

    assert_eq!(execute_part_two(input_file_path), expected);
}