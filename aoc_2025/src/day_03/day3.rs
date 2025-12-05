pub fn part1(input: &str) -> u64 {
    input.trim().split('\n').map(|bank| top_k(bank, 2)).sum()
}

pub fn part2(input: &str) -> u64 {
    input.trim().split('\n').map(|bank| top_k(bank, 12)).sum()
}

// Takes in a string of numbers and the number of elements that need to be kept -> returns the
// largest number made up of k elements in the initial list stably
fn top_k(numbers: &str, mut k: usize) -> u64 {
    // We need to always choose the first largest number in the front range of the list, until we
    // have chosed k, or only k remain in the list.
    let numbers: Vec<u64> = numbers
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    let mut start = 0;
    let mut sum = 0;
    // if k == 0 we found all numbers, if len - k == 0, we should take the remaining numbers
    while k != 0 && numbers.len() - k != 0 {
        let end = numbers.len() - k;
        // get the largest number in the current range we're considering.
        let (index, num) = top_in_range(&numbers, start, end);
        start = index + 1;
        sum *= 10;
        sum += num;
        k -= 1;
    }

    while k != 0 {
        sum *= 10;
        sum += numbers.get(numbers.len() - k).unwrap();
        k -= 1;
    }

    sum
}

// Takes a list of numbers, start and end index of a range and returns the largest number and it's
// index in the list.
fn top_in_range(numbers: &[u64], start: usize, end: usize) -> (usize, u64) {
    let (mut index, mut largest) = (start, numbers.get(start).unwrap());

    for i in (start + 1)..=end {
        let num = numbers.get(i).unwrap();
        if num > largest {
            largest = num;
            index = i;
        }
    }

    (index, *largest)
}

#[cfg(test)]
mod tests {
    use crate::day_03::day3::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(357, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(3121910778619, part2(input));
    }
}
