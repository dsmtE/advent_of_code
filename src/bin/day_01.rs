advent_of_code::solution!(1);

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right
}

#[derive(Debug, PartialEq)]
struct DialInstruction {
    direction: Direction,
    distance: u32,
}

impl DialInstruction {
    fn from_str(s: &str) -> Option<DialInstruction> {
        let mut chars = s.chars();
        let direction = chars.next()?;
        let direction = match direction {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => return None,
        };
        let distance: u32 = chars.collect::<String>().parse().ok()?;
        Some(DialInstruction { direction, distance })
    }
    
    fn as_integer_offset(&self) -> i32 {
        match self.direction {
            Direction::Left => -(self.distance as i32),
            Direction::Right => self.distance as i32,
        }
    }
}

fn parse_input_iterator(input: &str) -> impl Iterator<Item = DialInstruction> + use<'_>  {
    input.lines().map(DialInstruction::from_str).map(|x| x.unwrap())
}

fn part_one(input: &str) -> Option<u32> {
    Some(parse_input_iterator(input).fold((50i32, 0u32), |(mut dial, zeroed_count), instruction| {
        dial += instruction.as_integer_offset();
        dial = dial.rem_euclid(100);
        (dial, zeroed_count + (dial == 0) as u32)
    }).1)
}

fn part_two(input: &str) -> Option<u32> {
    Some(parse_input_iterator(input).fold((50i32, 0u32), |(dial, click_count), instruction| {
        let dial_was_at_zero = dial == 0;
        let mut new_dial = dial + instruction.as_integer_offset();

        let mut click = 0;

        if new_dial >= 100 {
            click += (new_dial / 100) as u32;
        } else if new_dial <= 0 {
            click += ((new_dial / 100).abs() + 1) as u32;
            // if we were at zero and moved left, we shouldn't count that as a click
            if dial_was_at_zero {
                click -= 1;
            }
        }
        new_dial = new_dial.rem_euclid(100);

        (new_dial, click_count + click)
    }).1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_parse_input() {
        let input = parse_input_iterator(TEST_INPUT).collect::<Vec<_>>();

        let expected = vec![
            DialInstruction { direction: Direction::Left, distance: 68 },
            DialInstruction { direction: Direction::Left, distance: 30 },
            DialInstruction { direction: Direction::Right, distance: 48 },
            DialInstruction { direction: Direction::Left, distance: 5 },
            DialInstruction { direction: Direction::Right, distance: 60 },
            DialInstruction { direction: Direction::Left, distance: 55 },
            DialInstruction { direction: Direction::Left, distance: 1 },
            DialInstruction { direction: Direction::Left, distance: 99 },
            DialInstruction { direction: Direction::Right, distance: 14 },
            DialInstruction { direction: Direction::Left, distance: 82 },
        ];

        assert_eq!(input, expected);
    }
    
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), Some(3));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(6));
    }
}