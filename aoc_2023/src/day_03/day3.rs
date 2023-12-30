#[derive(Debug)]
enum Sector {
    Symbol(char),
    Number(u32),
    Blank,
}

struct Schematic {
    layout: Vec<Vec<Sector>>,
    symbol_location: Vec<(usize, usize)>,
    gear_location: Vec<(usize, usize)>,
}

impl Schematic {
    /// Read in a schematic and store it as a 2d vec of Sector
    fn new(s: &str) -> Self {
        let mut layout: Vec<Vec<Sector>> = vec![vec![]];
        let _ = layout.pop();
        let mut symbols: Vec<(usize, usize)> = vec![];
        let mut gears: Vec<(usize, usize)> = vec![];
        let mut i = 0;
        let it = s.split('\n');
        for l in it.into_iter() {
            let mut line: Vec<Sector> = vec![];
            let mut j = 0;
            let ch = l.chars();
            for c in ch.into_iter() {
                if c.is_digit(10) {
                    line.push(Sector::Number(c.to_digit(10).unwrap()));
                } else if c == '.' {
                    line.push(Sector::Blank);
                } else if c.is_ascii_punctuation() {
                    line.push(Sector::Symbol(c));
                    symbols.push((i, j));
                    if c == '*' {
                        gears.push((i, j));
                    }
                }
                j += 1;
            }
            if line.len() > 0 {
                layout.push(line);
            }
            i += 1;
        }

        Self {
            layout: layout,
            symbol_location: symbols,
            gear_location: gears,
        }
    }

    /// Returns the part number given a 2d index or 0 if no part exists. The index can be anywhere 'in' the part number.
    fn get_part_number(&self, start_index: (usize, usize)) -> u32 {
        // First check out of bounds
        if start_index.0 >= self.layout.len() || start_index.1 >= self.layout[0].len() {
            return 0;
        }
        // find the first digit to the left
        let mut start = start_index;
        loop {
            match self.layout[start.0][start.1] {
                Sector::Number(_) => {
                    if start.1 == 0 {
                        break;
                    }
                    start.1 -= 1;
                }
                _ => {
                    // If not number, we check if we need to move forward to the last number
                    if start.1 != start_index.1 {
                        start.1 += 1;
                    }
                    break;
                }
            };
        }
        // count each digit from left to right
        let mut part_no = 0;
        while start.1 < self.layout[start.0].len() {
            match self.layout[start.0][start.1] {
                Sector::Number(x) => {
                    part_no *= 10;
                    part_no += x;
                }
                _ => {
                    break;
                }
            }
            start.1 += 1;
        }
        part_no
    }

    /// Checks for adjacent part numbers and returns a vec containing any that were found.
    fn check_adjacent_parts(&self, location: (usize, usize)) -> Vec<u32> {
        let mut part_numbers: Vec<u32> = vec![];
        // Check left and right
        if location.1 != 0 {
            part_numbers.push(self.get_part_number((location.0, location.1 - 1)));
        }
        part_numbers.push(self.get_part_number((location.0, location.1 + 1)));
        // Check top and bottom. If directly above or below contains a number, no need to check diagonals.
        // Top
        if location.0 != 0 {
            let part_no = self.get_part_number((location.0 - 1, location.1));
            if part_no != 0 {
                part_numbers.push(part_no);
            } else {
                if location.1 != 0 {
                    part_numbers.push(self.get_part_number((location.0 - 1, location.1 - 1)));
                }
                part_numbers.push(self.get_part_number((location.0 - 1, location.1 + 1)));
            }
        }
        // Bottom
        let part_no = self.get_part_number((location.0 + 1, location.1));
        if part_no != 0 {
            part_numbers.push(part_no);
        } else {
            if location.1 != 0 {
                part_numbers.push(self.get_part_number((location.0 + 1, location.1 - 1)));
            }
            part_numbers.push(self.get_part_number((location.0 + 1, location.1 + 1)));
        }

        part_numbers.into_iter().filter(|&p| p != 0).collect()
    }

    fn get_gear_ratio(&self, location: (usize, usize)) -> u32 {
        let parts = self.check_adjacent_parts(location);
        // println!("({}, {}) : {:?}", location.0, location.1, parts);
        if parts.len() != 2 {
            return 0;
        }
        parts[0] * parts[1]
    }
}

/// Returns the sum of part numbers
pub fn solve_q3_p1(s: &str) -> u32 {
    // Read schematic
    let schematic = Schematic::new(s);
    // loop over the symbols array
    let mut sum_of_parts: u32 = 0;
    for (i, j) in schematic.symbol_location.iter() {
        // check for adjacent part numbers
        sum_of_parts += schematic.check_adjacent_parts((*i, *j)).iter().sum::<u32>();
    }

    sum_of_parts
}

pub fn solve_q3_p2(s: &str) -> u32 {
    // Read schematic
    let schematic = Schematic::new(s);
    let mut sum_of_gear_ratio = 0;
    for (i, j) in schematic.gear_location.iter() {
        sum_of_gear_ratio += schematic.get_gear_ratio((*i, *j));
    }
    sum_of_gear_ratio
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An input and expected return
    struct TestCase<T, U>(T, U);

    #[test]
    fn test_symbols() {
        let test_cases = vec![
            TestCase(String::from("467..114..\n...*......\n..35..633."), 502),
            TestCase(String::from("..35..633.\n......#...\n617......."), 633),
            TestCase(String::from("617*......"), 617),
            TestCase(String::from(".....+.58.\n..592....."), 592),
            TestCase(String::from("...$......\n.664.598.."), 664),
            TestCase(String::from("......755.\n.....*....\n.664.598.."), 1353),
        ];

        for case in test_cases {
            let result = solve_q3_p1(&case.0);
            assert_eq!(
                result, case.1,
                "solve_q3_p1(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_solve_q3p1() {
        let d3p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q3_p1(d3p1_test), 4361);
    }

    #[test]
    fn test_gears() {
        let test_cases = vec![
            TestCase(String::from("467..114..\n...*......\n..35..633."), 16345),
            TestCase(String::from("......755.\n.....*....\n.664.598.."), 451490),
        ];

        for case in test_cases {
            let result = solve_q3_p2(&case.0);
            assert_eq!(
                result, case.1,
                "solve_q3_p2(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_solve_q3p2() {
        let d3p2_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q3_p2(d3p2_test), 467835);
    }

    #[test]
    fn test_answers() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_q3_p1(input), 546312);
        assert_eq!(solve_q3_p2(input), 87449461);
    }
}
