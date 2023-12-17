use std::cmp;

#[derive(Debug)]
struct Bag(u32, u32, u32);

/// Returns the sum of game id's of possible games.
pub fn solve_q2_p1(s: &str) -> u32 {
    let bag = Bag(12, 13, 14);
    s.split('\n').map(|l| is_possible(l, &bag)).sum()
}

/// Returns the id of a game if it's possible given the bag, otherwise 0.
fn is_possible(game: &str, bag: &Bag) -> u32 {
    let (id, hands) = game.split_once(":").unwrap();
    let id = id.split_once(" ").unwrap().1.parse::<u32>().unwrap();

    let it = hands.split(";");
    for hand in it.into_iter() {
        let (r, g, b) = read_hand(hand);
        if r > bag.0 || g > bag.1 || b > bag.2 {
            return 0;
        }
    }

    id
}

/// Returns the sum of the power of all games
pub fn solve_q2_p2(s: &str) -> u32 {
    s.split('\n').map(|l| min_power(l)).sum()
}

/// Returns the power of the minimum bag of cubes given a game
fn min_power(game: &str) -> u32 {
    let (_, hands) = game.split_once(":").unwrap();

    let mut min_bag = Bag(0, 0, 0);

    let it = hands.split(";");
    for hand in it.into_iter() {
        let (r, g, b) = read_hand(hand);
        min_bag.0 = cmp::max(min_bag.0, r);
        min_bag.1 = cmp::max(min_bag.1, g);
        min_bag.2 = cmp::max(min_bag.2, b);
    }

    min_bag.0 * min_bag.1 * min_bag.2
}

/// Returns the count of red green and blue cubes as a tuple (r, g, b), given a single handfull of cubes
fn read_hand(hand: &str) -> (u32, u32, u32) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;

    let it = hand.split(",");
    for color in it.into_iter() {
        let color = color.trim();
        let (n, c) = color.split_once(" ").unwrap();
        match c {
            "red" => r += n.parse::<u32>().unwrap(),
            "green" => g += n.parse::<u32>().unwrap(),
            "blue" => b += n.parse::<u32>().unwrap(),
            _ => {}
        };
    }

    (r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// An input and expected return
    struct TestCase<T, U>(T, U);

    #[test]
    fn test_is_possible() {
        let test_cases = vec![
            TestCase(
                String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
                1,
            ),
            TestCase(
                String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
                2,
            ),
            TestCase(
                String::from(
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                ),
                0,
            ),
            TestCase(
                String::from(
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                ),
                0,
            ),
            TestCase(
                String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
                5,
            ),
        ];

        let bag = Bag(12, 13, 14);

        for case in test_cases {
            let result = is_possible(&case.0, &bag);
            assert_eq!(
                result, case.1,
                "is_possible(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_solve_q2p1() {
        let d2p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q2_p1(d2p1_test), 8)
    }

    #[test]
    fn test_min_power() {
        let test_cases = vec![
            TestCase(
                String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
                48,
            ),
            TestCase(
                String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
                12,
            ),
            TestCase(
                String::from(
                    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                ),
                1560,
            ),
            TestCase(
                String::from(
                    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
                ),
                630,
            ),
            TestCase(
                String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
                36,
            ),
        ];

        for case in test_cases {
            let result = min_power(&case.0);
            assert_eq!(
                result, case.1,
                "min_power(\"{}\"): Expected: {}, Got: {}",
                case.0, case.1, result
            );
        }
    }

    #[test]
    fn test_solve_q2p2() {
        let d2p2_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q2_p2(d2p2_test), 2286)
    }
}
