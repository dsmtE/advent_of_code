use aoc_utils::{cartesian::{Point2, Vector2}, grid::{Grid, parse_grid}};
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Roll,
    Empty
}

fn parse_roll_grid(input: &str) -> Grid<Cell> {
    parse_grid(input, |c| {
        match c {
            '@' => Cell::Roll,
            '.' => Cell::Empty,
            _ => panic!("")
        }
    })
}
fn roll_should_be_removed(roll_grid: &Grid<Cell>, position: &Point2) -> bool {
    Vector2::ALL_DIRECTIONS.iter()
        .map(|&dir| dir + *position)
        .filter(|adj_pos| roll_grid.in_bound(adj_pos))
        .filter(|adj_pos| roll_grid[adj_pos] == Cell::Roll)
        .count() < 4
}

fn part_one(input: &str) -> Option<usize> {
    let roll_grid: Grid<Cell> = parse_roll_grid(input);
    Some(
        roll_grid.data.par_iter()
            .enumerate()
            .filter(|(_, &cell)| cell == Cell::Roll)
            .map(|(i, v)| (roll_grid.index_to_point(i), v))
            .filter(|(position, _)| {
                roll_should_be_removed(&roll_grid, position)
        })
        .count()
    )
}

fn part_two(input: &str) -> Option<usize> {
    let mut roll_grid: Grid<Cell> = parse_roll_grid(input);

    let mut removed = 0;
    // work on a copy of the grid each iteration to avoid modifying while iterating
    let mut new_data: Vec<Cell> = roll_grid.data.clone();
    loop {
        let removed_this_step = new_data.par_iter_mut()
            .enumerate()
            .filter(|(_, &mut cell)| cell == Cell::Roll)
            .map(|(i, cell)| (roll_grid.index_to_point(i), cell))
            .filter_map(|(position, cell)| {
                roll_should_be_removed(&roll_grid, &position).then(|| {
                    *cell = Cell::Empty;
                })
            })
            .count();

        if removed_this_step == 0 {
            break;
        }

        removed += removed_this_step;

        // update the roll grid for the next iteration
        roll_grid.data = new_data.clone();
    }

    Some(removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    
    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), Some(13));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), Some(43));
    }
}