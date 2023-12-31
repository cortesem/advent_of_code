#[derive(Clone, Debug)]
struct Hand {
    cards: Vec<u32>,
    bid: u64,
    hand_type: u32,
}

impl Hand {
    fn new(s: &str, joker: bool) -> Self {
        let mut kind: Vec<u32> = vec![0; 15];
        let mut jokers = 0; // this is all so gross

        let (hand, bid) = s.split_once(' ').unwrap();
        let bid = bid.parse::<u64>().unwrap();

        // Convert string to u32 values
        let cards: Vec<u32> = hand
            .chars()
            .map(|c| {
                if c.is_digit(10) {
                    return c.to_digit(10).unwrap();
                }
                match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => {
                        if joker {
                            jokers += 1;
                            1
                        } else {
                            11
                        }
                    }
                    'T' => 10,
                    _ => 0,
                }
            })
            .rev() // reverse this vec for sorting later
            .collect();

        // add cards to the lookup table
        for c in &cards {
            kind[*c as usize] += 1;
        }

        // We don't want to consider jokers yet.
        if joker {
            kind[1] = 0;
        }

        // sort lookup table. We only need to check the first two indecies to know what kind of hand we have.
        kind.sort_by(|a, b| b.cmp(a));

        // Jokers will always pad the largest number of duplicate cards.
        if joker {
            kind[0] += jokers;
        }

        let mut it = kind.iter();

        // Classify the type of hand so we can sort them later. adding jokers made this gross to read :)
        let hand_type = match (it.next().unwrap(), it.next().unwrap()) {
            (1, _) => 1,
            (2, 2) => 3,
            (2, _) => 2,
            (3, 2) => 5,
            (3, _) => 4,
            (4, _) => 6,
            (5, _) => 7,
            _ => 0,
        };

        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

struct Game {
    hands: Vec<Vec<Hand>>,
}

impl Game {
    fn new(s: &str, joker: bool) -> Self {
        // each type of hand gets its own vec
        let mut hands: Vec<Vec<Hand>> = vec![vec![]; 7];

        for line in s.split('\n') {
            let hand = Hand::new(line, joker);
            hands[hand.hand_type as usize - 1].push(hand);
        }

        // kind of gross but simple.
        // Sort each types vec (sort the hands with eachother) from last to first card, which gives us the correct ordering.
        for v in hands.iter_mut() {
            for i in 0..5 {
                v.sort_by(|a, b| a.cards[i as usize].cmp(&b.cards[i as usize]));
            }
        }

        Self { hands }
    }

    fn count_hands(&self) -> u64 {
        let mut multiplier: u64 = 0;
        self.hands
            .iter()
            .map(|v| {
                v.iter()
                    .map(|h| {
                        multiplier += 1;
                        h.bid * multiplier
                    })
                    .sum::<u64>()
            })
            .sum()
    }
}

pub fn solve_q7_p1(s: &str) -> u64 {
    let game = Game::new(s, false);
    game.count_hands()
}

pub fn solve_q7_p2(s: &str) -> u64 {
    let game = Game::new(s, true);
    game.count_hands()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_q7p1() {
        let d7p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q7_p1(d7p1_test), 6440);
    }

    #[test]
    fn test_solve_q7p2() {
        let d7p2_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q7_p2(d7p2_test), 5905);
    }
}
