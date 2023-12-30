struct Race {
    // time: u64,
    // distance_record: u64,
    winning_holds: (u64, u64),
}

impl Race {
    fn new(time: u64, distance_record: u64) -> Self {
        let winning_holds = Race::get_roots(1.0, -(time as f64), distance_record as f64);

        Self {
            // time,
            // distance_record,
            winning_holds,
        }
    }

    fn get_number_of_winnng_holds(&self) -> u64 {
        self.winning_holds.1 - self.winning_holds.0 + 1
    }

    fn get_roots(a: f64, b: f64, c: f64) -> (u64, u64) {
        // no error checking here :) maybe it'll work!
        let dividend = (b.powi(2) - (4.0 * a * c)).sqrt();
        let divisor = 2.0 * a;
        let x1 = (-b + dividend) / divisor;
        let x2 = (-b - dividend) / divisor;

        // add and subtract a small amount for 'edge' cases where the roots are whole numbers (we need to beat the time!)
        if x1 > x2 {
            return ((x2 + 0.01).ceil() as u64, (x1 - 0.01).floor() as u64);
        }

        ((x1 + 0.01).ceil() as u64, (x2 - 0.01).floor() as u64)
    }
}

// x^2−Tx+D=0
// solve roots
// ceiling of lesser root and floor of greater root are the bounds.
pub fn solve_q6_p1(s: &str) -> u64 {
    let (time, distance) = s.split_once('\n').unwrap();
    let time = time.split_once(':').unwrap().1.trim();
    let time: Vec<u64> = time
        .split(' ')
        .map(|n| n.trim().parse::<u64>().unwrap_or(0))
        .filter(|&n| n != 0)
        .collect();

    let distance = distance.split_once(':').unwrap().1.trim();
    let distance: Vec<u64> = distance
        .split(' ')
        .map(|n| n.trim().parse::<u64>().unwrap_or(0))
        .filter(|&n| n != 0)
        .collect();

    let mut races = vec![];
    for i in 0..time.len() {
        races.push(Race::new(time[i], distance[i]));
    }

    races
        .iter()
        .map(|r| r.get_number_of_winnng_holds())
        .product()
}

pub fn solve_q6_p2(s: &str) -> u64 {
    let (time, distance) = s.split_once('\n').unwrap();
    let time = time.split_once(':').unwrap().1.trim();
    let time: String = time.chars().filter(|c| !c.is_whitespace()).collect();
    let time = time.parse::<u64>().unwrap();

    let distance = distance.split_once(':').unwrap().1.trim();
    let distance: String = distance.chars().filter(|c| !c.is_whitespace()).collect();
    let distance = distance.parse::<u64>().unwrap();

    let race = Race::new(time, distance);
    race.get_number_of_winnng_holds()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_q6p1() {
        let d6p1_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q6_p1(d6p1_test), 288);
    }

    #[test]
    fn test_solve_q6p2() {
        let d6p2_test = include_str!("./input1_test.txt");
        assert_eq!(solve_q6_p2(d6p2_test), 71503);
    }
}
