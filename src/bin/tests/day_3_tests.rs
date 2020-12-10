use super::*;

#[test]
fn test_part_one_example() {
    let input_file_path = "src/bin/tests/inputs/day_3/day_three_input_example.txt";
    let input_slope = vec![3,1];
    let expected = 7;

    assert_eq!(execute_part_one(input_file_path, input_slope), expected);
}

#[test]
fn test_part_two_example() {
    let input_file_path = "src/bin/tests/inputs/day_3/day_three_input_example.txt";
    let input_slopes = vec![vec![1,1],vec![3,1],vec![5,1],vec![7,1],vec![1,2]];
    let expected = 336;

    assert_eq!(execute_part_two(input_file_path, input_slopes), expected);
}

#[test]
fn test_parse_input() {
    let input_file_path = "src/bin/tests/inputs/day_3/day_three_input_example_simple.txt";
    let expected = vec![vec![".","#"], vec!["#","."]];

    assert_eq!(parse_input(input_file_path), expected);
}