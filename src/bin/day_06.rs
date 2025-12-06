use aoc_utils::iterator::IterUtils;

advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
enum Operator {
    Add,
    Multiply
}

impl Operator {
    fn from_char_panic(c: char) -> Self {
        match c {
            '+' => Operator::Add,
            '*' => Operator::Multiply,
            _ => panic!("Unexpected operator")
        }
    }

    fn get_functor(&self) -> fn(NumberType, NumberType) -> NumberType {
        match self {
            Operator::Add => std::ops::Add::add,
            Operator::Multiply => std::ops::Mul::mul,
        }
    }
}

type NumberType = u64;

type Problem = (Operator, Vec<NumberType>);

fn parse_input_first_start(input: &str) -> Vec<Problem>{
    let mut lines = input.lines();
    
    let mut numbers: Vec<Vec<NumberType>> = vec![];

    let mut current_line = lines.next();
    let mut next_line = lines.next();

    while current_line.is_some() && next_line.is_some() {
        current_line.unwrap()
            .split_whitespace()
            .map(|num_str| num_str.parse().unwrap())
            .enumerate()
            .for_each(|(i, num)| {
                if numbers.len() <= i {
                    numbers.push(vec![]);
                }
                numbers[i].push(num);
            });
            
        current_line = std::mem::replace(&mut next_line, lines.next());
    }

    let operators = current_line.unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .map(Operator::from_char_panic)
        .collect::<Vec<Operator>>();

    operators.into_iter().zip(numbers.into_iter()).collect()
}

fn parse_input_second_start(input: &str) -> Vec<Problem>{
    let mut lines = input.lines();
    
    let mut chars: Vec<Vec<char>> = vec![];

    let mut current_line = lines.next();
    let mut next_line = lines.next();

    while current_line.is_some() && next_line.is_some() {
        chars.push(current_line.unwrap().chars().collect()); 
        current_line = std::mem::replace(&mut next_line, lines.next());
    }

    let width = chars[0].len();
    let height = chars.len();

    let mut problems_numbers: Vec<Vec<NumberType>> = vec![vec![]];

    let mut problem_numbers: &mut Vec<NumberType> = problems_numbers.last_mut().unwrap();
    for col in 0..width {
        let digits = (0..height)
                .map(|row| chars[row][col])
                .filter(|c| *c != ' ')
                .map(|c| c as NumberType - '0' as NumberType).collect::<Vec<_>>();

        // if we hit a column with only spaces, we move to the next problem
        if digits.is_empty() {
            problems_numbers.push(vec![]);
            problem_numbers = problems_numbers.last_mut().unwrap();
            continue;
        }

        problem_numbers.push(digits.into_iter().fold_decimal());
    }

    let operators = current_line.unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .map(Operator::from_char_panic)
        .collect::<Vec<Operator>>();

    operators.into_iter().zip(problems_numbers.into_iter()).collect()
}

fn compute_problem(problem: &Problem) -> NumberType {
    let (operator, numbers) = problem;
    match operator {
        Operator::Add => numbers.iter().sum(),
        Operator::Multiply => numbers.iter().product()
    }
}
fn part_one(input: &str) -> Option<NumberType> {
    Some(parse_input_first_start(input).iter()
        .map(compute_problem)
        .sum()
    )
}

fn part_two(input: &str) -> Option<NumberType> {
    Some(parse_input_second_start(input).iter()
        .map(compute_problem)
        .sum()
    )
}

#[cfg(test)]
mod tests {
    use aoc_utils::iterator::IterUtils;

    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
    
    #[test]
    fn test_parsing() {
        assert_eq!(parse_input_first_start(TEST_INPUT), vec![
            (Operator::Multiply, vec![123, 45, 6]),
            (Operator::Add, vec![328, 64, 98]),
            (Operator::Multiply, vec![51, 387, 215]),
            (Operator::Add, vec![64, 23, 314])
        ]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(parse_input_first_start(TEST_INPUT).iter()
        .map(compute_problem).collect_vec(), vec![33210, 490, 4243455, 401]);
    }

    #[test]
    fn test_part_two() {
        let problems = parse_input_second_start(TEST_INPUT);
        assert_eq!(problems, vec![
            (Operator::Multiply, vec![1, 24, 356]),
            (Operator::Add, vec![369, 248, 8]),
            (Operator::Multiply, vec![32, 581, 175]),
            (Operator::Add, vec![623, 431, 4]),
        ]);

        assert_eq!(problems.iter().map(compute_problem).collect_vec(), vec![8544, 625, 3253600, 1058]);
    }
}