use std::collections::HashMap;
use super::*;

#[test]
fn when_mutating_a_value_with_a_mask_and_the_mask_is_all_blank() {
    let value = 101;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
    let expected = 101;

    assert_eq!(mutate_value_with_mask(value, mask.to_string()), expected);
}

#[test]
fn when_mutating_a_value_with_a_mask_and_the_mask_is_not_blank_but_the_mask_lines_up() {
    let value = 101;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let expected = 101;

    assert_eq!(mutate_value_with_mask(value, mask.to_string()), expected);
}

#[test]
fn when_mutating_a_value_with_a_mask_and_the_mask_is_not_blank_and_mutates_the_value() {
    let value = 11;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let expected = 73;

    assert_eq!(mutate_value_with_mask(value, mask.to_string()), expected);
}

#[test]
fn when_mutating_a_value_with_a_mask_and_the_mask_is_not_blank_but_the_value_is_zero() {
    let value = 0;
    let mask = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X";
    let expected = 64;

    assert_eq!(mutate_value_with_mask(value, mask.to_string()), expected);
}

#[test]
fn when_executing_blocks_part_one_and_there_is_only_one_memory_address_and_one_mask() {
    let lines = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        "mem[8] = 11".to_string()
    ];
    let mut expected = HashMap::new();
    expected.insert(8,73);

    assert_eq!(execute_blocks_part_one(lines), expected);
}

#[test]
fn when_executing_blocks_part_one_and_there_is_two_different_memory_addresses_and_one_mask() {
    let lines = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        "mem[8] = 11".to_string(),
        "mem[7] = 101".to_string()
    ];
    let mut expected = HashMap::new();
    expected.insert(8,73);
    expected.insert(7,101);

    assert_eq!(execute_blocks_part_one(lines), expected);
}

#[test]
fn when_executing_blocks_part_one_and_there_is_three_different_memory_addresses_and_two_masks() {
    let lines = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        "mem[8] = 11".to_string(),
        "mem[7] = 101".to_string(),
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXX101".to_string(),
        "mem[8] = 11".to_string(),
    ];
    let mut expected = HashMap::new();
    expected.insert(8,77);
    expected.insert(7,101);

    let result = execute_blocks_part_one(lines);
    assert_eq!(result.len(), 2);
    assert_eq!(result, expected);
}

#[test]
fn when_executing_part_one() {
    let input = "src/bin/tests/inputs/day_14/input_example_part_one.txt";
    let expected = 165;

    assert_eq!(execute_part_one(input), expected);
}

#[test]
fn when_mutating_an_address_with_a_mask_and_the_mask_is_all_zeros() {
    let address = 101;
    let mask = "000000000000000000000000000000000000";
    let expected = vec![101];

    assert_eq!(mutate_address_with_mask(address, mask.to_string()), expected);
}

#[test]
fn when_mutating_an_address_with_a_mask_and_the_mask_has_a_one() {
    let address = 101;
    let mask = "000000000000000000000000000000000010";
    let expected = vec![103];

    assert_eq!(mutate_address_with_mask(address, mask.to_string()), expected);
}

#[test]
fn when_mutating_an_address_with_a_mask_and_the_mask_has_an_x() {
    let address = 42;
    let mask = "000000000000000000000000000000X1001X";
    let expected = vec![27,26,58,59];

    assert_eq!(mutate_address_with_mask(address, mask.to_string()), expected);
}

#[test]
fn when_getting_binary_combinations_and_the_count_is_two() {
    let count = 2;
    let expected = vec![
        vec![0,1],
        vec![0,0],
        vec![1,0],
        vec![1,1]
    ];

    assert_eq!(get_all_binary_combinations(count), expected);
}

#[test]
fn when_getting_binary_combinations_and_the_count_is_three() {
    let count = 3;
    let expected = vec![
        [0, 1, 0], 
        [0, 1, 1], 
        [0, 0, 1], 
        [0, 0, 0], 
        [1, 0, 1], 
        [1, 0, 0], 
        [1, 1, 0], 
        [1, 1, 1]
    ];

    assert_eq!(get_all_binary_combinations(count), expected);
}

#[test]
fn when_filling_binary_combinations_into_a_result_and_there_are_four_combos() {
    let combinations = vec![
        vec![0,1],
        vec![0,0],
        vec![1,0],
        vec![1,1]
    ];
    let result = "000000000000000000000000000000X1101X".to_string();
    let expected = vec![
        "000000000000000000000000000000011011".to_string(),
        "000000000000000000000000000000011010".to_string(),
        "000000000000000000000000000000111010".to_string(),
        "000000000000000000000000000000111011".to_string()
    ];

    assert_eq!(fill_binary_combinations_into_result(combinations, result), expected);
}

#[test]
fn when_filling_binary_combinations_into_a_result_and_there_are_eight_combos() {
    let combinations = vec![
        vec![0, 1, 0], 
        vec![0, 1, 1], 
        vec![0, 0, 1], 
        vec![0, 0, 0], 
        vec![1, 0, 1], 
        vec![1, 0, 0], 
        vec![1, 1, 0], 
        vec![1, 1, 1]
    ];
    let result = "00000000000000000000000000000001X0XX".to_string();
    let expected = vec![
        "000000000000000000000000000000010010".to_string(),
        "000000000000000000000000000000010011".to_string(),
        "000000000000000000000000000000010001".to_string(),
        "000000000000000000000000000000010000".to_string(),
        "000000000000000000000000000000011001".to_string(),
        "000000000000000000000000000000011000".to_string(),
        "000000000000000000000000000000011010".to_string(),
        "000000000000000000000000000000011011".to_string()
    ];

    assert_eq!(fill_binary_combinations_into_result(combinations, result), expected);
}

#[test]
fn when_executing_blocks_part_two_with_four_permutations() {
    let lines = vec![
        "mask = 000000000000000000000000000000X1001X".to_string(),
        "mem[42] = 100".to_string()
    ];
    let mut expected = HashMap::new();
    expected.insert(27,100);
    expected.insert(26,100);
    expected.insert(58,100);
    expected.insert(59,100);

    assert_eq!(execute_blocks_part_two(lines), expected);
}

#[test]
fn when_executing_blocks_part_two_with_eight_permutations() {
    let lines = vec![
        "mask = 00000000000000000000000000000000X0XX".to_string(),
        "mem[26] = 1".to_string()
    ];
    let mut expected = HashMap::new();
    expected.insert(16,1);
    expected.insert(17,1);
    expected.insert(18,1);
    expected.insert(19,1);
    expected.insert(24,1);
    expected.insert(25,1);
    expected.insert(26,1);
    expected.insert(27,1);

    assert_eq!(execute_blocks_part_two(lines), expected);
}

#[test]
fn when_executing_part_two() {
    let input = "src/bin/tests/inputs/day_14/input_example_part_two.txt";
    let expected = 208;

    assert_eq!(execute_part_two(input), expected);
}