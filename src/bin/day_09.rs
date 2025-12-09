use itertools::Itertools;

advent_of_code::solution!(9);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

fn area(a: &Point2, b: &Point2) -> u64 {
    let width = a.x.abs_diff(b.x) as u64 + 1;
    let height = a.y.abs_diff(b.y) as u64 + 1;
    width * height
}

fn parse_points(input: &str) -> Vec<Point2> {
    input.lines().map(|line| {
        let mut coords = line.split(',').map(|s| s.parse().unwrap());
        Point2{x: coords.next().unwrap(), y: coords.next().unwrap() }
    }).collect()
}

fn part_one(input: &str) -> Option<u64> {
    parse_points(input).iter().tuple_combinations().map(|(a, b)| area(&a, &b)).max()
}


fn inside(edges: &Vec<(&Point2, &Point2)>, a: &Point2, b: &Point2) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    edges.iter().all(|(c, d)| {
        let edge_min_x = c.x.min(d.x);
        let edge_max_x = c.x.max(d.x);
        let edge_min_y = c.y.min(d.y);
        let edge_max_y = c.y.max(d.y);

        // check if the edge (c, d) is completely outside the rectangle defined by (a, b)
        let left_of_rect = max_x <= edge_min_x;
        let right_of_rect = min_x >= edge_max_x;
        let above = max_y <= edge_min_y;
        let below = min_y >= edge_max_y;
        left_of_rect || right_of_rect || above || below
    })
}

fn part_two(input: &str) -> Option<u64> {
    let points = parse_points(input);
    let edges = points
        .iter()
        .circular_tuple_windows()
        .collect::<Vec<_>>();

    Some(points
        .iter()
        .tuple_combinations()
        .map(|(a, b)| { (a, b, area(&a, &b)) })
        .sorted_by_key(|&(_, _, area)| area)
        .rev()
        .find(|(a, b, _)| inside(&edges, a, b))
        .unwrap().2)
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
            Point2{x:7, y:1},
            Point2{x:11, y:1},
            Point2{x:11, y:7},
            Point2{x:9, y:7},
            Point2{x:9, y:5},
            Point2{x:2, y:5},
            Point2{x:2, y:3},
            Point2{x:7, y:3},
        ]);
        assert_eq!(part_one(TEST_INPUT), Some(50));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(24));
    }
}