use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Tile {
    point: (usize, usize),
    direction: Direction,
    steps: u32,
}

impl Tile {
    fn move_next(&mut self) {
        match self.direction {
            Direction::Up => self.point.0 -= 1,
            Direction::Down => self.point.0 += 1,
            Direction::Left => self.point.1 -= 1,
            Direction::Right => self.point.1 += 1,
        }
    }
}

fn find_start(start_symbol: char, maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == start_symbol {
                return Some((i, j));
            }
        }
    }
    None
}

fn get_direction(tile: &Tile, maze: &Vec<Vec<char>>) -> Direction {
    let current = maze[tile.point.0][tile.point.1];
    // match on the previous direction
    match tile.direction {
        Direction::Up => {
            if current == '|' {
                return Direction::Up;
            } else if current == '7' {
                return Direction::Left;
            }
            Direction::Right
        }
        Direction::Down => {
            if current == '|' {
                return Direction::Down;
            } else if current == 'L' {
                return Direction::Right;
            }
            Direction::Left
        }
        Direction::Left => {
            if current == '-' {
                return Direction::Left;
            } else if current == 'L' {
                return Direction::Up;
            }
            Direction::Down
        }
        Direction::Right => {
            if current == '-' {
                return Direction::Right;
            } else if current == 'J' {
                return Direction::Up;
            }
            Direction::Down
        }
    }
}

fn get_first_tiles(start: (usize, usize), maze: &Vec<Vec<char>>) -> Option<Tile> {
    // check left
    if start.1 != 0 {
        let left = maze[start.0][start.1 - 1];
        if "FL-".contains(left) {
            return Some(Tile {
                point: (start.0, start.1 - 1),
                direction: Direction::Left,
                steps: 0,
            });
        }
    }

    // check right
    if start.1 < maze[0].len() {
        let right = maze[start.0][start.1 + 1];
        if "J7-".contains(right) {
            return Some(Tile {
                point: (start.0, start.1 + 1),
                direction: Direction::Right,
                steps: 0,
            });
        }
    }

    // check up
    if start.0 != 0 {
        let up = maze[start.0 - 1][start.1];
        if "F7|".contains(up) {
            return Some(Tile {
                point: (start.0 - 1, start.1),
                direction: Direction::Up,
                steps: 0,
            });
        }
    }

    // check down
    if start.0 < maze.len() {
        let down = maze[start.0 + 1][start.1];
        if "LJ|".contains(down) {
            return Some(Tile {
                point: (start.0 + 1, start.1),
                direction: Direction::Down,
                steps: 0,
            });
        }
    }

    None
}

fn find_mid(maze: &mut Vec<Vec<char>>) -> u32 {
    // get starting point
    let start = find_start('S', maze).unwrap();

    // get the first tile
    // check the 4 tiles around S.. if their directions 'fits' it's one of the two starting points
    let mut tile = get_first_tiles(start, maze).unwrap();
    tile.direction = get_direction(&tile, maze);

    while maze[tile.point.0][tile.point.1] != 'S' {
        // move tile
        tile.move_next();
        // increment steps
        tile.steps += 1;
        // get next directions
        tile.direction = get_direction(&tile, maze);
    }
    (tile.steps + 1) / 2
}

// It looks kinda cool
#[allow(dead_code)]
fn print_maze(maze: &Vec<Vec<char>>) {
    for line in maze {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }
}
// At first I thought it would be 'easy' to use a polygon algorithm! Looks like we'll just flood fill it
fn count_enclosed_tiles(maze: &mut Vec<Vec<char>>) -> u32 {
    // expand the maze to make life easy
    let mut loop_cache: Vec<Vec<char>> = vec![vec![' '; maze[0].len() * 2]; maze.len() * 2];
    for i in 0..maze.len() {
        for j in 0..maze[0].len() {
            loop_cache[i * 2][j * 2] = '0';
        }
    }
    // get starting point
    let start = find_start('S', maze).unwrap();
    loop_cache[start.0 * 2][start.1 * 2] = 'S';

    // get the first tile
    // check the 4 tiles around S.. if their direction 'fits' it's one of the two starting points
    let mut tile = get_first_tiles(start, maze).unwrap();
    tile.direction = get_direction(&tile, maze);

    // check around the start point to fill in gaps. we don't actually care what it is, only that it blocks the fill
    // forgot to error check but seems like it was ok
    if maze[start.0 + 1][start.1] != '.' {
        loop_cache[start.0 * 2 + 1][start.1 * 2] = '-';
    }
    if maze[start.0 - 1][start.1] != '.' {
        loop_cache[start.0 * 2 - 1][start.1 * 2] = '-';
    }
    if maze[start.0][start.1 + 1] != '.' {
        loop_cache[start.0 * 2][start.1 * 2 + 1] = '|';
    }
    if maze[start.0][start.1 - 1] != '.' {
        loop_cache[start.0 * 2][start.1 * 2 - 1] = '|';
    }

    // update the cache with loop segments, the cache will only contain the loop and 0's (and X's (and an S))
    while maze[tile.point.0][tile.point.1] != 'S' {
        // fill in the gap ahead, we've expanded the maze
        if tile.direction == Direction::Up {
            loop_cache[tile.point.0 * 2 - 1][tile.point.1 * 2] = '|'
        } else if tile.direction == Direction::Down {
            loop_cache[tile.point.0 * 2 + 1][tile.point.1 * 2] = '|'
        } else if tile.direction == Direction::Left {
            loop_cache[tile.point.0 * 2][tile.point.1 * 2 - 1] = '-'
        } else {
            loop_cache[tile.point.0 * 2][tile.point.1 * 2 + 1] = '-'
        }

        // mark our loop segment
        loop_cache[tile.point.0 * 2][tile.point.1 * 2] = maze[tile.point.0][tile.point.1];
        tile.move_next();
        // get next directions
        tile.direction = get_direction(&tile, maze);
    }
    // print_maze(&loop_cache);
    // println!("\n\n");

    // flood fill '0' and ' '
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    // Just to be safe, get all 4 corners in. If we really wanted to be super duper safe
    // we would pad a line of spaces around the entire maze.
    queue.push_back((0, 0));
    queue.push_back((0, loop_cache[0].len() as i32 - 1));
    queue.push_back((loop_cache.len() as i32 - 1, 0));
    queue.push_back((loop_cache.len() as i32 - 1, loop_cache[0].len() as i32 - 1));
    while queue.len() != 0 {
        let (y, x) = queue.pop_front().unwrap();
        if y < 0
            || y >= loop_cache.len() as i32
            || x < 0
            || x >= loop_cache[0].len() as i32
            || !" 0".contains(loop_cache[y as usize][x as usize])
        {
            continue;
        }

        // fill this element
        loop_cache[y as usize][x as usize] = 'X';

        // queue up the next elements. up, down, left, right
        queue.push_back((y - 1, x));
        queue.push_back((y + 1, x));
        queue.push_back((y, x - 1));
        queue.push_back((y, x + 1));
    }

    // print_maze(&loop_cache);

    // count 0 in each line
    (loop_cache
        .iter_mut()
        .map(|l| l.iter().filter(|ch| *ch == &'0').count())
        .sum::<usize>()) as u32
}

pub fn solve_q10_p1(s: &str) -> u32 {
    let mut maze: Vec<Vec<char>> = s.split('\n').map(|l| l.chars().collect()).collect();
    find_mid(&mut maze)
}

pub fn solve_q10_p2(s: &str) -> u32 {
    let mut maze: Vec<Vec<char>> = s.split('\n').map(|l| l.chars().collect()).collect();
    count_enclosed_tiles(&mut maze)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An input and expected return
    // struct TestCase<T, U>(T, U);

    #[test]
    fn test_solve_q10p1() {
        let d10p1_test1 = include_str!("./input1_test.txt");
        let d10p1_test2 = include_str!("./input2_test.txt");
        assert_eq!(solve_q10_p1(d10p1_test1), 4);
        assert_eq!(solve_q10_p1(d10p1_test2), 8);
    }

    #[test]
    fn test_solve_q10p2() {
        let d10p2_test = include_str!("./input3_test.txt");
        let d10p2_test2 = include_str!("./input4_test.txt");
        assert_eq!(solve_q10_p2(d10p2_test), 4);
        assert_eq!(solve_q10_p2(d10p2_test2), 8);
    }

    #[test]
    fn test_answers() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_q10_p1(input), 6754);
        assert_eq!(solve_q10_p2(input), 567);
    }
}
