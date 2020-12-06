use super::*;

#[test]
fn test_part_one_example() {
    let input = vec![1721,979,366,299,675,1456];
    let expected = vec![1721,299];
    assert_eq!(execute_part_one(&input), expected);
}

#[test]
fn test_part_two_example() {
    let input = vec![1721,979,366,299,675,1456];
    let expected = vec![979,366,675];
    assert_eq!(execute_part_two(&input), expected);
}