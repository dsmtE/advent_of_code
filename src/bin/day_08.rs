advent_of_code::solution!(8);

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
struct Point3(pub [i32; 3]);

fn square_distance(a: &Point3, b: &Point3) -> f32 {
    a.0.iter().zip(b.0.iter()).map(|(x, y)| *x as f32 - *y as f32).map(|x| x * x).sum()
}

fn parse_points(input: &str) -> Vec<Point3> {
    input.lines().map(|line| {
        let mut coords = line.split(',').map(|s| s.parse().unwrap());
        Point3([coords.next().unwrap(), coords.next().unwrap(), coords.next().unwrap()])
    }).collect()
}

fn sorted_edges(points: &Vec<Point3>) -> Vec<(usize, usize)> {
    let mut edges_with_distance = (0..points.len()).tuple_combinations().map(|(i, j)| {
        (square_distance(&points[i], &points[j]), i, j)
    }).collect::<Vec<_>>();

    edges_with_distance.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    edges_with_distance.iter().map(|&(_, i, j)| (i, j)).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Node {
    Child(usize), // parent index
    Root(u32), // size
}

fn create_circuits(n: usize) -> Vec<Node> {
    // initially, each node is its own circuit of size 1
    (0..n).map(|_| Node::Root(1)).collect()
}

fn find_root(circuits: &Vec<Node>, node_index: usize) -> usize {
    let mut node = &circuits[node_index];
    // keep track of current index to return it
    let mut current_index = node_index;
    loop {
        match node {
            Node::Root(_) => return current_index,
            Node::Child(parent) => {
                node = &circuits[*parent];
                current_index = *parent;
            }
        }
    }
}

// return the size of the circuit after adding the edge
fn add_edge(circuits: &mut Vec<Node>, a: usize, b: usize) -> usize {
    let mut root_a = find_root(circuits, a);
    let mut root_b = find_root(circuits, b);

    assert!( matches!(circuits[root_a], Node::Root(_)) && matches!(circuits[root_b], Node::Root(_)) );

    // if they are already in the same circuit, do nothing
    if root_a == root_b {
        return match circuits[root_a] {
            Node::Root(size) => size as usize,
            _ => unreachable!(),
        }
    }

    let size_a = match circuits[root_a] {
        Node::Root(size) => size,
        _ => unreachable!(),
    };

    let size_b = match circuits[root_b] {
        Node::Root(size) => size,
        _ => unreachable!(),
    };
    
    // to keep tree flat, always attach smaller tree to larger tree
    // and only update size of the larger tree (avoid updating size on each node of the circuit)
    if size_a < size_b {
        (root_a, root_b) = (root_b, root_a);
    }

    circuits[root_a] = Node::Root(size_a + size_b);
    circuits[root_b] = Node::Child(root_a);

    (size_a + size_b) as usize

}

fn part_one(input: &str) -> Option<u32> {
    let points = parse_points(input);
    let edges = sorted_edges(&points);
    let mut circuits = create_circuits(points.len());

    for edge in edges.iter().take(1000) {
        add_edge(&mut circuits, edge.0, edge.1);
    }
    
    circuits.sort_unstable();

    // it should work as there are at least 3 circuits in the input
    Some(circuits.iter().rev().map(|c| {
        match c {
            Node::Root(size) => *size,
            Node::Child(_) => panic!("Expected root node"),
        }
    }).take(3).product())
}

fn part_two(input: &str) -> Option<i32> {
    let points = parse_points(input);
    let edges = sorted_edges(&points);
    let point_count = points.len();
    let mut circuits = create_circuits(point_count);

    let mut edge = (0,0);
    for i in 0..edges.len() {
        edge = edges[i];
        let last_edge_size = add_edge(&mut circuits, edge.0, edge.1);
        // keep adding edges until all points are connected
        if last_edge_size >= point_count {
            break;
        }
    }

    Some(points[edge.0].0[0] * points[edge.1].0[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn debug_print_circuits(circuits: &Vec<Node>) {
        print!("i: ");
        for i in 0..circuits.len(){
            print!("{:^4}", i);
        }
        println!();
        print!("   ");
        for node in circuits.iter() {
            match node {
                Node::Root(size) => print!("{:^4}", size),
                Node::Child(parent) => print!("P{:>2} ", parent),
            }
        }
        println!();
    }

    const TEST_INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
    
    #[test]
    fn test_parse() {
        assert_eq!(parse_points(TEST_INPUT), vec![
            Point3([162, 817, 812]),
            Point3([57, 618, 57]),
            Point3([906, 360, 560]),
            Point3([592, 479, 940]),
            Point3([352, 342, 300]),
            Point3([466, 668, 158]),
            Point3([542, 29, 236]),
            Point3([431, 825, 988]),
            Point3([739, 650, 466]),
            Point3([52, 470, 668]),
            Point3([216, 146, 977]),
            Point3([819, 987, 18]),
            Point3([117, 168, 530]),
            Point3([805, 96, 715]),
            Point3([346, 949, 466]),
            Point3([970, 615, 88]),
            Point3([941, 993, 340]),
            Point3([862, 61, 35]),
            Point3([984, 92, 344]),
            Point3([425, 690, 689]),
        ]);
    }

    #[test]
    fn test_sort() {
        let points =parse_points(TEST_INPUT);
        let edges = sorted_edges(&points);
        assert_eq!(edges.iter().take(4).map(|&(i, j)| (points[i], points[j]) ).collect::<Vec<(Point3, Point3)>>(), vec![
            (Point3([162,817,812]), Point3([425,690,689])),
            (Point3([162,817,812]), Point3([431,825,988])),
            (Point3([906,360,560]), Point3([805,96,715])),
            (Point3([431,825,988]), Point3([425,690,689])),
        ]);
    }

    #[test]
    fn test_part_one() {
        let points = parse_points(TEST_INPUT);
        let edges = sorted_edges(&points);
        let mut circuits = create_circuits(points.len());

        for edge in edges.iter().take(10) {
            add_edge(&mut circuits, edge.0, edge.1);
            println!("After adding edge {:?}:", edge);
            debug_print_circuits(&circuits);
        }
    
        circuits.sort_unstable();

        println!("after sorting:");

        debug_print_circuits(&circuits);

        let result: u32 = circuits.iter().rev().take(3).map(|c| {
            match c {
                Node::Root(size) => *size,
                Node::Child(_) => panic!("Expected root node"),
            }
        }).product();

        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(25272));
    }
}