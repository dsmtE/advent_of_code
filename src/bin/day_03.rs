use aoc_utils::iterator::IterUtils;

advent_of_code::solution!(3);

fn parse_input_iterator(input: &str) -> impl Iterator<Item = &[u8]> + use<'_>  {
    input.lines().map(|line| line.as_bytes())
}

fn find_max_batteries<const N: usize>(bank: &[u8]) ->Result<[u8; N], &'static str> {
    let bank_size = bank.len();

    if bank_size < N {
        return Err("Bank size is smaller than number of batteries to select");
    }

    // copy the last two batteries as the initial selected batteries
    let mut selected_batteries: [u8; N] = bank[bank_size - N..].try_into().unwrap();

    // iterate through the rest of the bank from the end to the start
    for &(mut next_battery) in bank[..bank_size - N].iter().rev() {
        for selected_battery in &mut selected_batteries {
            if next_battery < *selected_battery {
                break;
            }

            // shift down the batteries by copying the current selected battery
            next_battery = std::mem::replace(selected_battery, next_battery)
            //by assigning the old value to next_battery it will continue to be compared with the rest
            // and shift down the values as needed
        }

        // shift down : first start version
        // if next_battery >= selected_batteries[0] {
        //     // shift down the max battery to second max only if first is greater
        //     if selected_batteries[0] >= selected_batteries[1] {
        //         selected_batteries[1] = selected_batteries[0];
        //     }
        //     selected_batteries[0] = next_battery;
        // }
    }

    Ok(selected_batteries)
}

fn compute_joltage(batteries: &[u8]) -> u64 {
    batteries.iter().map(|b| (b - b'0') as u64).fold_decimal()
}

fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input_iterator(input)
        .map(|bank| find_max_batteries::<2>(&bank).unwrap())
        .map(|batteries| compute_joltage(&batteries))
        .sum()
    )
}

fn part_two(input: &str) -> Option<u64> {
    Some(
        parse_input_iterator(input)
        .map(|bank| find_max_batteries::<12>(&bank).unwrap())
        .map(|batteries| compute_joltage(&batteries))
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";
    
    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input_iterator(TEST_INPUT).collect::<Vec<_>>(),
            vec![
                b"987654321111111",
                b"811111111111119",
                b"234234234234278",
                b"818181911112111",
            ]
        );
    }

    #[test]
    fn test_part_one() {
        let batteries_joltage= parse_input_iterator(TEST_INPUT)
        .map(|bank| find_max_batteries::<2>(&bank).unwrap())
        .map(|batteries| compute_joltage(&batteries)).collect::<Vec<_>>();
        assert_eq!(batteries_joltage, vec![98, 89, 78, 92]);
    }

    #[test]
    fn test_part_two() {
        let batteries_joltage= parse_input_iterator(TEST_INPUT)
        .map(|bank| find_max_batteries::<12>(&bank).unwrap())
        .map(|batteries| compute_joltage(&batteries)).collect::<Vec<_>>();
        assert_eq!(batteries_joltage, vec![987654321111, 811111111119, 434234234278, 888911112111]);
    }
}