use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(2);

fn parse_input_iterator(input: &str) -> impl Iterator<Item = (u64, u64)> + '_ {
    input.split(',')
        .map(|str| str.split('-'))
        .map(|mut bounds| {
            (
                bounds.next().unwrap().parse().ok().unwrap(),
                bounds.next().unwrap().parse().ok().unwrap(),
            )
        })
}

fn number_of_digits(number: &u64) -> u32 {
    number.checked_ilog10().unwrap_or(0) + 1
}

fn number_split_half(number: &u64) -> Option<(u64, u64)> {
    let digits: u32 = number_of_digits(number);
    if digits % 2 != 0 {
        return None;
    }

    let half_digits = digits / 2;
    let divisor = 10u64.pow(half_digits);
    Some((number / divisor, number % divisor))
}

fn invalid_ids_in_range_iter(&(start, end): &(u64, u64)) -> impl rayon::iter::ParallelIterator<Item = u64> {
    (start..=end).into_par_iter()
    .filter(|n| {
        number_of_digits(n) % 2 == 0
    })
    .filter(|n| {
        // can unwrap here because we already filtered to even digit numbers
        let (first_half, second_half) = number_split_half(n).unwrap();
        first_half == second_half
    })
}

fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_input_iterator(input).collect::<Vec<_>>().par_iter()
            .flat_map(invalid_ids_in_range_iter)
            .map(|n| n as usize)
            .sum()
    )
}

fn sub_sequence_repeat_at_least_twice(str: &str) -> bool {
    let len = str.len();
    let chars_slice = str.as_bytes();

    // for each possible chunk length
    // check if any chunk length divides the string into equal chunks
    (1..=(len / 2)).filter(|chunk_len| len % chunk_len == 0)
        .any(|chunk_len| {
            // test if all chunks of this length are equal
            let mut chunks = chars_slice.chunks(chunk_len);
            let first_chunk = chunks.next().unwrap();
            chunks.all(|chunk| chunk == first_chunk)
        })
}

fn part_two(input: &str) -> Option<usize> {
    Some(
        parse_input_iterator(input)
            .collect::<Vec<_>>()
            .par_iter()
            .flat_map(|&(start, end)|
                (start..=end).into_par_iter()
                .filter(|n| { sub_sequence_repeat_at_least_twice(&n.to_string()) })
            )
            .map(|n| n as usize)
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    const TEST_INPUT_2: &str = "3737332285-3737437376,1-1234";
    
    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input_iterator(TEST_INPUT).collect::<Vec<_>>(),
            vec![
                (11,22),(95,115),(998,1012),(1188511880,1188511890),(222220,222224),(1698522,1698528),(446443,446449),(38593856,38593862),(565653,565659),(824824821,824824827),(2121212118,2121212124),
            ]
        );
    }

    fn get_invalid_ids_as_vec_of_vec(range_iter: impl Iterator<Item = (u64, u64)>) -> Vec<Vec<u64>> {
        range_iter
            .collect::<Vec<_>>()
            .par_iter()
            .map(|x| invalid_ids_in_range_iter(x).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn get_invalid_ids_star2(input: impl Iterator<Item = (u64, u64)>) -> Vec<Vec<u64>> {
        input
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&(start, end)|
                (start..=end).into_par_iter()
                .filter(|n| { sub_sequence_repeat_at_least_twice(&n.to_string()) })
                .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_part_one() {
        assert_eq!(get_invalid_ids_as_vec_of_vec(parse_input_iterator(TEST_INPUT)),
        vec![
            vec![11,22], vec![99], vec![1010], vec![1188511885], vec![222222], vec![], vec![446446], vec![38593859], vec![], vec![], vec![],
            ]);

        assert_eq!(part_one(TEST_INPUT), Some(1227775554));

        assert_eq!(get_invalid_ids_as_vec_of_vec(parse_input_iterator(TEST_INPUT_2)),
        vec![
            vec![3737337373, 3737437374], vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212],
            ]);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_invalid_ids_star2(parse_input_iterator(TEST_INPUT)),
        vec![
            vec![11,22], vec![99, 111], vec![999, 1010], vec![1188511885], vec![222222], vec![], vec![446446], vec![38593859], vec![565656], vec![824824824], vec![2121212121],
            ]);
    }
}