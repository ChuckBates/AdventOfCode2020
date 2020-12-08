use super::*;

#[test]
fn when_executing_a_step_and_the_step_is_nop() {
    let input_step = "nop +2";
    let accumulator = 5;
    let position = 5;
    let expected = (5, 6);

    assert_eq!(execute_step(input_step.to_string(), accumulator, position), expected);
} 

#[test]
fn when_executing_a_step_and_the_step_is_acc_of_plus_one() {
    let input_step = "acc +1";
    let accumulator = 5;
    let position = 5;
    let expected = (6, 6);

    assert_eq!(execute_step(input_step.to_string(), accumulator, position), expected);
} 

#[test]
fn when_executing_a_step_and_the_step_is_acc_of_minus_four() {
    let input_step = "acc -4";
    let accumulator = 5;
    let position = 5;
    let expected = (1, 6);

    assert_eq!(execute_step(input_step.to_string(), accumulator, position), expected);
} 

#[test]
fn when_executing_a_step_and_the_step_is_jmp_of_plus_two() {
    let input_step = "jmp +2";
    let accumulator = 5;
    let position = 5;
    let expected = (5, 7);

    assert_eq!(execute_step(input_step.to_string(), accumulator, position), expected);
} 

#[test]
fn when_executing_a_step_and_the_step_is_jmp_of_minus_two() {
    let input_step = "jmp -2";
    let accumulator = 5;
    let position = 5;
    let expected = (5, 3);

    assert_eq!(execute_step(input_step.to_string(), accumulator, position), expected);
} 

#[test]
fn when_executing_part_one_example() {
    let input_file_path = "src/bin/tests/inputs/day_eight/input_example.txt";
    let expected = 5;

    assert_eq!(execute_part_one(input_file_path), expected);
}

#[test]
fn when_executing_part_two_example() {
    let input_file_path = "src/bin/tests/inputs/day_eight/input_example.txt";
    let expected = 8;

    assert_eq!(execute_part_two(input_file_path), expected);
}