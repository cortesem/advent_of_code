#[derive(Debug)]
struct Galaxy(usize, usize);

impl Galaxy {
    /// Returns the shortest distance between two Galaxy without considering expansion
    fn distance_to(&self, other: &Galaxy) -> u64 {
        let (y1, y2): (u64, u64) = (self.0 as u64, other.0 as u64);
        let (x1, x2): (u64, u64) = (self.1 as u64, other.1 as u64);

        let dy = y2.abs_diff(y1);
        let dx = x2.abs_diff(x1);

        dy + dx
    }

    /// Returns the number of times this Galaxy crosses an expanded line to
    /// get to other Galaxy
    fn calculate_expansion_crossings(
        &self,
        other: &Galaxy,
        rows: &Vec<bool>,
        cols: &Vec<bool>,
        exp_val: u64,
    ) -> u64 {
        let (y1, y2) = (self.0, other.0);
        let (x1, x2) = (self.1, other.1);

        let mut crossed: u64 = 0;
        for i in (y1 + 1)..y2 {
            if rows[i] {
                crossed += 1;
            }
        }
        // x needs to go backwards sometimes.
        for i in (x1 + 1)..x2 {
            if cols[i] {
                crossed += 1;
            }
        }
        for i in (x2 + 1)..x1 {
            if cols[i] {
                crossed += 1;
            }
        }

        crossed * exp_val - crossed
    }
}

fn read_input(s: &str) -> Vec<Galaxy> {
    s.split('\n')
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_j, ch)| ch == '#')
                .map(move |(j, _ch)| Galaxy(i, j))
        })
        .flatten()
        .collect::<Vec<Galaxy>>()
}

pub fn solve_q11_p1(s: &str) -> u64 {
    // read in input. add any galaxy to an array of galaxy's
    let galaxys: Vec<Galaxy> = read_input(s);

    // loop the array of galaxy's to find the expanded rows and columns
    let col_len = s.split('\n').count();
    let row_len = s.split('\n').next().unwrap().chars().count();
    let mut expanded_rows: Vec<bool> = vec![true; col_len + 1];
    let mut expanded_cols: Vec<bool> = vec![true; row_len];
    for galaxy in galaxys.iter() {
        expanded_rows[galaxy.0] = false;
        expanded_cols[galaxy.1] = false;
    }

    // loop the galaxy's again and compare each galaxy to all remaining galaxy's
    let mut sum_of_lengths = 0;
    for i in 0..(galaxys.len() - 1) {
        for j in (i + 1)..galaxys.len() {
            sum_of_lengths += galaxys[i].distance_to(&galaxys[j])
                + galaxys[i].calculate_expansion_crossings(
                    &galaxys[j],
                    &expanded_rows,
                    &expanded_cols,
                    2,
                );
        }
    }

    sum_of_lengths
}

pub fn solve_q11_p2(s: &str) -> u64 {
    // read in input. add any galaxy to an array of galaxy's
    let galaxys: Vec<Galaxy> = read_input(s);

    // loop the array of galaxy's to find the expanded rows and columns
    let col_len = s.split('\n').count();
    let row_len = s.split('\n').next().unwrap().chars().count();
    let mut expanded_rows: Vec<bool> = vec![true; col_len + 1];
    let mut expanded_cols: Vec<bool> = vec![true; row_len];
    for galaxy in galaxys.iter() {
        expanded_rows[galaxy.0] = false;
        expanded_cols[galaxy.1] = false;
    }

    // loop the galaxy's again and compare each galaxy to all remaining galaxy's
    let mut sum_of_lengths = 0;
    for i in 0..(galaxys.len() - 1) {
        for j in (i + 1)..galaxys.len() {
            sum_of_lengths += galaxys[i].distance_to(&galaxys[j])
                + galaxys[i].calculate_expansion_crossings(
                    &galaxys[j],
                    &expanded_rows,
                    &expanded_cols,
                    1000000,
                )
        }
    }

    sum_of_lengths
}

#[cfg(test)]
mod tests {
    use super::*;

    // /// An input and expected return
    // // struct TestCase<T, U>(T, U);

    #[test]
    fn test_solve_q11p1() {
        let d11p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q11_p1(d11p1_test), 374);
    }

    // #[test]
    // fn test_solve_q11p2() {
    //     let d11p2_test = include_str!("./input1_test.txt");
    //     assert_eq!(solve_q11_p2(d11p2_test), 4);
    // }

    #[test]
    fn test_answers() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_q11_p1(input), 10276166);
        assert_eq!(solve_q11_p2(input), 598693078798);
    }
}
