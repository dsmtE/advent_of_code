use std::collections::HashSet;

advent_of_code::solution!(7);

fn parse_input(input: &str) -> (usize, usize, impl Iterator<Item = Vec<usize>> + use<'_>) {
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();

    let size = first_line.len();
    let starting_position = first_line.chars().position(|c| c == 'S').unwrap();

    let splitter_positions = lines.skip(1).map(|line| {
        line.chars()
            .enumerate()
            .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
            .collect()
    });

    (starting_position, size, splitter_positions)
}

fn part_one(input: &str) -> Option<u32> {
    let (starting_position, size, iter) = parse_input(input);

    let mut beam_positions = HashSet::new();
    beam_positions.insert(starting_position);
    let mut next_beam_positions = HashSet::new();
    let mut split_count = 0;

    for splitter_positions in iter.filter(|x| !x.is_empty()) {
        for beam_pos in beam_positions.iter() {
            if splitter_positions.contains(beam_pos) {
                // split the beam
                if *beam_pos > 0 {
                    next_beam_positions.insert(*beam_pos - 1);
                }
                if *beam_pos < size {
                    next_beam_positions.insert(*beam_pos + 1);
                }
                split_count += 1;
            }
            else {
                // continue straight
                next_beam_positions.insert(*beam_pos);
            }
        }

        beam_positions = std::mem::replace(&mut next_beam_positions, HashSet::new());
    }

    Some(split_count)
}

fn part_two(input: &str) -> Option<u64> {
    let (starting_position, size, iter) = parse_input(input);

    let mut beam_counts = vec![0; size];
    beam_counts[starting_position] = 1;
    let mut next_beam_counts = vec![0; size];

    for splitter_positions in iter.filter(|x| !x.is_empty()) {
        for (beam_pos, &count) in beam_counts.iter().enumerate().filter(|(_, &count)| count > 0) {
            if splitter_positions.binary_search(&beam_pos).is_ok() {
                // split the beam
                if let Some(next_count) = next_beam_counts.get_mut(beam_pos - 1) {
                    if *next_count == 0 {
                        *next_count = count;
                    }
                    else {
                        *next_count += count;
                    }
                }

                if let Some(next_count) = next_beam_counts.get_mut(beam_pos + 1) {
                    if *next_count == 0 {
                        *next_count = count;
                    }
                    else {
                        *next_count += count;
                    }
                }
            }
            else {
                // continue straight
                next_beam_counts[beam_pos] += count;
            }
        }

        beam_counts = std::mem::replace(&mut next_beam_counts, vec![0; size]);
    }

    Some(beam_counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
    
    #[test]
    fn test_parsing() {
        let (pos, size, iter) = parse_input(TEST_INPUT);

        assert_eq!(pos, 7);
        assert_eq!(size, 15);
        assert_eq!(iter.filter(|x| !x.is_empty()).collect::<Vec<_>>(), vec![
            vec![7],
            vec![6, 8],
            vec![5, 7, 9],
            vec![4, 6, 10],
            vec![3, 5, 9, 11],
            vec![2, 6, 12],
            vec![1, 3, 5, 7, 9, 13],
        ]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), Some(21));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(40));
    }
}