use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::ops::RangeInclusive;

advent_of_code::solution!(5);

fn parse_ingredients(input: &str) -> (Vec::<RangeInclusive<u64>>, Vec<u64>) {
    let mut lines = input.lines();
    let ingredient_ranges = lines.by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split('-');
            let start = parts.next().unwrap().parse().unwrap();
            let end = parts.next().unwrap().parse().unwrap();
            start..=end
        })
        .collect();
    let ingredient_ids = lines.map(|line| line.parse().unwrap()).collect();

    (ingredient_ranges, ingredient_ids)
}

fn part_one(input: &str) -> Option<usize> {
    let (ingredient_ranges, ingredient_ids) = parse_ingredients(input);

    Some(
        ingredient_ids.into_par_iter()
        .filter(|id| {
            ingredient_ranges.par_iter().any(|range| range.contains(id))
        })
        .count()
    )
}

fn merge_intervals(ranges: &mut Vec<RangeInclusive<u64>>) -> Vec<RangeInclusive<u64>> {
    // sort ranges by start to make insertion easier
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    let mut merged_ranges: Vec<RangeInclusive<u64>> = vec![];

    // as ranges to insert are sorted by start, we can just try to merge with the last range
    for new_range in ranges {
        if let Some(last_range) = merged_ranges.last_mut() {
            if new_range.start() <= last_range.end() {
                *last_range = *last_range.start()..=(*std::cmp::max(new_range.end(), last_range.end()));
                continue;
            }
        }
        // otherwise just add the new range
        merged_ranges.push(new_range.clone());
    }

    merged_ranges
}

fn get_range_size(range: &RangeInclusive<u64>) -> u64 {
    range.end() - range.start() + 1
}

fn part_two(input: &str) -> Option<u64> {
    let (mut ingredient_ranges, _) = parse_ingredients(input);

    let merged_ranges = merge_intervals(&mut ingredient_ranges);

    Some(
        merged_ranges.iter().map(get_range_size).sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
    
    #[test]
    fn test_parse() {
        let (ingredient_ranges, ingredient_ids) = parse_ingredients(TEST_INPUT);
        assert_eq!(ingredient_ranges, vec![3..=5, 10..=14, 16..=20, 12..=18]);
        assert_eq!(ingredient_ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_part_one() { 
        assert_eq!(part_one(TEST_INPUT), Some(3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(14));
    }
}