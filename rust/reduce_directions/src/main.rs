#[derive(Debug, Copy, Clone)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn main() {
    use Direction::*;
    let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
    println!("{:?}", dir_reduc(&a));
}

fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
    let mut a: Vec<Direction> = arr.to_vec();
    let mut index = 0;

    while index < a.len() - 1 {
        println!("{:?}", a);
        if is_match(&a[index..index + 2]) {
            a.remove(index);
            a.remove(index);
            index = 0;
            continue;
        }
        index += 1;
    }

    return a;
}

fn is_match(a: &[Direction]) -> bool {
    use Direction::*;
    match (a[0], a[1]) {
        (NORTH, SOUTH) | (SOUTH, NORTH) | (WEST, EAST) | (EAST, WEST) => true,
        _ => false,
    }
}

// fn dir_reduc(arr: &[Direction]) -> Vec<Direction> {
//     let mut result: Vec<Direction> = Vec::new();
//     for &s in arr {
//         if !result.is_empty() && can_be_reduced(s, *result.last().unwrap()) {
//             result.pop();
//         } else {
//             result.push(s);
//         }
//     }
//     result
// }

// fn can_be_reduced(elem: Direction, last: Direction) -> bool {
//     use Direction::*;
//     match (elem, last) {
//         (NORTH, SOUTH) | (SOUTH, NORTH) | (WEST, EAST) | (EAST, WEST) => true,
//         _ => false,
//     }
// }

// #[test]
// fn returns_expected() {
//     use Direction::*;
//     let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
//     assert_eq!(dir_reduc(&a), [WEST]);
//     let a = [NORTH, WEST, SOUTH, EAST];
//     assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
// }

// #[test]
// fn returns_advanced() {
//     use Direction::*;
//     let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, WEST];
//     assert_eq!(dir_reduc(&a), [WEST]);
//     let a = [NORTH, WEST, SOUTH, EAST];
//     assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
//     let a = [NORTH, SOUTH, SOUTH, EAST, WEST, NORTH, NORTH];
//     assert_eq!(dir_reduc(&a), [NORTH]);
//     let a = [
//         EAST, EAST, WEST, NORTH, WEST, EAST, EAST, SOUTH, NORTH, WEST,
//     ];
//     assert_eq!(dir_reduc(&a), [EAST, NORTH]);
//     let a = [
//         NORTH, EAST, NORTH, EAST, WEST, WEST, EAST, EAST, WEST, SOUTH,
//     ];
//     assert_eq!(dir_reduc(&a), [NORTH, EAST]);
//     let a = [NORTH, WEST, SOUTH, EAST];
//     assert_eq!(dir_reduc(&a), [NORTH, WEST, SOUTH, EAST]);
// }
