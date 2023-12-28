use std::collections::HashMap;

struct Card {
    card_id: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    matches: u32,
}

impl Card {
    fn new(s: &str) -> Self {
        let mut winning_numbers = vec![];
        let mut numbers = vec![];
        let mut winning_numbers_lookup = vec![false; 100];

        let (id, card) = s.split_once(':').unwrap();
        // Get Card no.
        let id = id.split_once(' ').unwrap().1.trim().parse::<u32>().unwrap();

        let (w_num, num) = card.split_once('|').unwrap();
        // Parse winning numbers
        let it = w_num.trim().split(' ');
        for n in it.into_iter() {
            let n = n.trim().parse::<u32>().unwrap_or(0);
            if n == 0 {
                continue;
            }
            winning_numbers.push(n);
            winning_numbers_lookup[n as usize] = true;
        }

        // Parse our numbers
        let it = num.trim().split(' ');
        for n in it.into_iter() {
            let n = n.trim().parse::<u32>().unwrap_or(0);
            if n == 0 {
                continue;
            }
            numbers.push(n);
        }

        // Count matches
        let mut matches: u32 = 0;
        for n in numbers.iter_mut() {
            if winning_numbers_lookup[*n as usize] {
                matches += 1;
            }
        }

        Self {
            card_id: id,
            winning_numbers,
            numbers,
            matches,
        }
    }

    // Returns the number of points this card is worth, 2^(matches - 1)
    fn get_points(&self) -> u32 {
        let base: u32 = 2;
        if self.matches == 0 {
            return 0;
        }
        base.pow(self.matches - 1)
    }
}

pub fn solve_q4_p1(s: &str) -> u32 {
    let cards: Vec<Card> = s.split('\n').map(|l| Card::new(l)).collect();

    cards.iter().map(|c| c.get_points()).sum()
}

// pub fn solve_q4_p2(s: &str) -> u32 {
//     let cards: Vec<Card> = s.split('\n').map(|l| Card::new(l)).collect();
//     let mut total_cards: u32 = 0;
//     // lookup table
//     let mut card_winnings = HashMap::<u32, u32>::new();
//     for c in cards.iter_mut() {}
// }

#[cfg(test)]
mod tests {
    use super::*;

    /// An input and expected return
    struct TestCase<T, U>(T, U);

    #[test]
    fn test_get_winning_numbers_matches() {
        let test_cases = vec![
            TestCase(
                String::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
                8,
            ),
            TestCase(
                String::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
                2,
            ),
            TestCase(
                String::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
                2,
            ),
            TestCase(
                String::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
                1,
            ),
            TestCase(
                String::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
                0,
            ),
            TestCase(
                String::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
                0,
            ),
        ];

        for case in test_cases {
            let card = Card::new(&case.0);
            let result = card.get_points();
            assert_eq!(
                result, case.1,
                "solve_q3_p1(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_solve_q4p1() {
        let d4p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q4_p1(d4p1_test), 13);
    }
}
