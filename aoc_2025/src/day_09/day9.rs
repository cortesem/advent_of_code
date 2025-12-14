#[derive(Debug, Clone)]
struct Point(u64, u64);

impl Point {
    fn distance_from(&self, other: &Point) -> u64 {
        self.0.abs_diff(other.0).pow(2) + self.1.abs_diff(other.1).pow(2)
    }
}

#[derive(Debug)]
struct Distance(Point, Point, u64);

pub fn part1(input: &str) -> u64 {
    let mut points = Vec::new();
    for p in input.lines() {
        let (x, y) = p.split_once(',').unwrap();
        points.push(Point(x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut distances = Vec::new();
    for point in points.iter() {
        distances.push(generate_distances(point, &points));
    }

    let mut distances: Vec<Distance> = distances.into_iter().flatten().collect();

    distances.sort_by_key(|a| a.2);

    let largest = distances.last().unwrap();

    (largest.0.0.abs_diff(largest.1.0) + 1) * (largest.0.1.abs_diff(largest.1.1) + 1)
}

pub fn part2(input: &str) -> u64 {
    1
}

fn generate_distances(point: &Point, points_list: &[Point]) -> Vec<Distance> {
    let mut result = Vec::new();
    for p in points_list.iter() {
        result.push(Distance(point.clone(), p.clone(), point.distance_from(p)));
    }

    result.sort_by_key(|a| a.2);
    result.remove(0);

    result
}

#[cfg(test)]
mod tests {
    use crate::day_09::day9::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(50, part1(input));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(1, part2(input));
    }
}
