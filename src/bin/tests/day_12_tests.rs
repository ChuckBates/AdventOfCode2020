use super::*;

// #[test]
// fn when_executing_part_one() {
//     let input = "";
//     let expected = 0;

//     assert_eq!(execute_part_one(input), expected);
// }


#[test]
fn when_handling_an_instruction_and_it_is_a_forward_instruction() {
    let instruction = "F2".to_string();
    let direction = "E".to_string();
    let position = (0,0);
    let expected = (2,0,"E".to_string());

    assert_eq!(handle_instruction(instruction, direction, position), expected);
}

#[test]
fn when_handling_an_instruction_and_it_is_a_north_instruction() {
    let instruction = "N2".to_string();
    let direction = "E".to_string();
    let position = (0,0);
    let expected = (0,2,"E".to_string());

    assert_eq!(handle_instruction(instruction, direction, position), expected);
}

#[test]
fn when_handling_an_instruction_and_it_is_a_south_instruction() {
    let instruction = "S2".to_string();
    let direction = "E".to_string();
    let position = (0,0);
    let expected = (0,-2,"E".to_string());

    assert_eq!(handle_instruction(instruction, direction, position), expected);
}

#[test]
fn when_handling_an_instruction_and_it_is_a_west_instruction() {
    let instruction = "W2".to_string();
    let direction = "E".to_string();
    let position = (0,0);
    let expected = (-2,0,"E".to_string());

    assert_eq!(handle_instruction(instruction, direction, position), expected);
}

#[test]
fn when_handling_an_instruction_and_it_is_an_east_instruction() {
    let instruction = "E2".to_string();
    let direction = "E".to_string();
    let position = (0,0);
    let expected = (2,0,"E".to_string());

    assert_eq!(handle_instruction(instruction, direction, position), expected);
}

#[test]
fn when_handling_an_instruction_and_it_is_a_right_turn_instruction() {
    let instruction = "R90".to_string();
    let direction = "E".to_string();
    let position = (0,0);
    let expected = (0,0,"S".to_string());

    assert_eq!(handle_instruction(instruction, direction, position), expected);
}

#[test]
fn when_handling_an_instruction_and_it_is_a_left_turn_instruction() {
    let instruction = "L270".to_string();
    let direction = "E".to_string();
    let position = (0,0);
    let expected = (0,0,"S".to_string());

    assert_eq!(handle_instruction(instruction, direction, position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_north_instruction() {
    let instruction = "N3".to_string();
    let ship_position = (1,2);
    let waypoint_position = (10,1);

    let expected = (1,2,10,4);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_south_instruction() {
    let instruction = "S10".to_string();
    let ship_position = (1,2);
    let waypoint_position = (10,1);

    let expected = (1,2,10,-9);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_east_instruction() {
    let instruction = "E7".to_string();
    let ship_position = (1,2);
    let waypoint_position = (10,1);

    let expected = (1,2,17,1);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_west_instruction() {
    let instruction = "W13".to_string();
    let ship_position = (1,2);
    let waypoint_position = (10,1);

    let expected = (1,2,-3,1);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_forward_instruction() {
    let instruction = "F10".to_string();
    let ship_position = (0,0);
    let waypoint_position = (10,1);

    let expected = (100,10,110,11);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_forward_instruction_not_at_origin() {
    let instruction = "F7".to_string();
    let ship_position = (100,10);
    let waypoint_position = (110,14);

    let expected = (170,38,180,42);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_forward_instruction_not_at_origin_2() {
    let instruction = "F11".to_string();
    let ship_position = (170,38);
    let waypoint_position = (174,28);

    let expected = (214,-72,218,-82);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_right_90_instruction() {
    let instruction = "R90".to_string();
    let ship_position = (170,38);
    let waypoint_position = (180,42);

    let expected = (170,38,174,28);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_right_90_instruction_2() {
    let instruction = "R90".to_string();
    let ship_position = (0,0);
    let waypoint_position = (5,-2);

    let expected = (0,0,-2,-5);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_right_90_instruction_3() {
    let instruction = "R90".to_string();
    let ship_position = (0,0);
    let waypoint_position = (-5,2);

    let expected = (0,0,2,5);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_left_90_instruction() {
    let instruction = "L90".to_string();
    let ship_position = (0,0);
    let waypoint_position = (5,2);

    let expected = (0,0,-2,5);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}
#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_left_90_instruction_2() {
    let instruction = "L90".to_string();
    let ship_position = (0,0);
    let waypoint_position = (5,-2);

    let expected = (0,0,2,5);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}
#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_left_90_instruction_3() {
    let instruction = "L90".to_string();
    let ship_position = (0,0);
    let waypoint_position = (-5,2);

    let expected = (0,0,-2,-5);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_right_180_instruction() {
    let instruction = "R180".to_string();
    let ship_position = (0,0);
    let waypoint_position = (5,2);

    let expected = (0,0,-5,-2);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_right_270_instruction() {
    let instruction = "R270".to_string();
    let ship_position = (0,0);
    let waypoint_position = (5,2);

    let expected = (0,0,-2,5);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_left_180_instruction() {
    let instruction = "L180".to_string();
    let ship_position = (0,0);
    let waypoint_position = (5,2);

    let expected = (0,0,-5,-2);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_handling_an_instruction_part_two_and_it_is_a_waypoint_rotate_left_270_instruction() {
    let instruction = "L270".to_string();
    let ship_position = (0,0);
    let waypoint_position = (5,2);

    let expected = (0,0,2,-5);

    assert_eq!(handle_instruction_part_two(instruction, ship_position, waypoint_position), expected);
}

#[test]
fn when_executing_part_two_example() {
    let input = "src/bin/tests/inputs/day_12/input_example.txt";
    let expected = 286;

    assert_eq!(execute_part_two(input), expected);
}
