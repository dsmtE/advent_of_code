use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Point2(pub [i32; 2]);

fn area(a: &Point2, b: &Point2) -> u64 {
    a.0.iter().zip(b.0.iter()).map(|(a, b)| a.abs_diff(*b) as u64 + 1).product()
}

fn parse_points(input: &str) -> Vec<Point2> {
    input.lines().map(|line| {
        let mut coords = line.split(',').map(|s| s.parse().unwrap());
        Point2([coords.next().unwrap(), coords.next().unwrap()])
    }).collect()
}

fn part_one(input: &str) -> Option<u64> {
    parse_points(input).iter().tuple_combinations().map(|(a, b)| area(&a, &b)).max()
}

fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
    
    #[test]
    fn test_part_one() {
        assert_eq!(parse_points(TEST_INPUT), vec![
            Point2([7, 1]),
            Point2([11, 1]),
            Point2([11, 7]),
            Point2([9, 7]),
            Point2([9, 5]),
            Point2([2, 5]),
            Point2([2, 3]),
            Point2([7, 3]),
        ]);
        assert_eq!(part_one(TEST_INPUT), Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(TEST_INPUT);
        assert_eq!(result, None);
    }
}