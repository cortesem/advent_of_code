use std::{cmp::Reverse, collections::HashSet};

#[derive(Debug, Clone)]
struct Position(i64, i64, i64);

impl Position {
    fn distance_to(&self, other: &Position) -> f64 {
        (((other.0 - self.0).pow(2) + (other.1 - self.1).pow(2) + (other.2 - self.2).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug)]
struct JunctionBox {
    pos: Position,
    closest: Position,
    dist: f64,
}

impl JunctionBox {
    fn new(pos: Position, closest: Position, dist: f64) -> Self {
        Self { pos, closest, dist }
    }
}

pub fn part1(input: &str, connections: usize) -> u64 {
    let mut locations: Vec<Position> = vec![];
    for location in input.lines() {
        let xyz: Vec<&str> = location.split(',').collect();
        locations.push(Position(
            xyz[0].parse().unwrap(),
            xyz[1].parse().unwrap(),
            xyz[2].parse().unwrap(),
        ));
    }

    // locations.sort_by(|a, b| a.0.cmp(&b.0));
    // locations.sort_by(|a, b| a.1.cmp(&b.1));
    // locations.sort_by(|a, b| a.2.cmp(&b.2));

    // for location in locations {
    //     println!("{:?}", location);
    // }

    let mut junctions = vec![];
    for (i, pos) in locations.iter().enumerate() {
        let mut closest = if i < locations.len() - 1 {
            &locations[i + 1]
        } else {
            &locations[i - 1]
        };
        let mut dist = pos.distance_to(closest);
        for (j, other) in locations.iter().enumerate() {
            if i == j {
                continue;
            }

            let other_dist = pos.distance_to(other);
            if other_dist < dist {
                dist = other_dist;
                closest = other;
            }
        }
        junctions.push(JunctionBox::new(pos.clone(), closest.clone(), dist));
    }

    junctions.sort_by(|a, b| a.dist.total_cmp(&b.dist));

    calc_circuits(&junctions, connections)
}

pub fn part2(input: &str) -> u64 {
    1
}

fn calc_circuits(junctions: &[JunctionBox], connections: usize) -> u64 {
    let mut circuits = Vec::<HashSet<(i64, i64, i64)>>::new();
    let mut initial_circuit = HashSet::<(i64, i64, i64)>::new();
    initial_circuit.insert((junctions[0].pos.0, junctions[0].pos.1, junctions[0].pos.2));
    initial_circuit.insert((
        junctions[0].closest.0,
        junctions[0].closest.1,
        junctions[0].closest.2,
    ));
    circuits.push(initial_circuit);
    // god this sucks
    for junction in junctions.iter().take(connections).skip(1) {
        let t1 = (junction.pos.0, junction.pos.1, junction.pos.2);
        let t2 = (junction.closest.0, junction.closest.1, junction.closest.2);
        let mut found = false;

        for c in circuits.iter_mut() {
            if c.contains(&t1) || c.contains(&t2) {
                found = true;
                c.insert(t1);
                c.insert(t2);
                break;
            }
        }

        if !found {
            let mut circuit = HashSet::<(i64, i64, i64)>::new();
            circuit.insert(t1);
            circuit.insert(t2);
            circuits.push(circuit);
        }
    }

    circuits.sort_by_key(|a| Reverse(a.len()));

    println!("{:?}", circuits);

    (circuits[0].len() * circuits[1].len() * circuits[2].len()) as u64
}

#[cfg(test)]
mod tests {
    use crate::day_08::day8::*;

    #[test]
    fn part1_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(40, part1(input, 10));
    }

    #[test]
    fn part2_example() {
        let input = include_str!("input/test_input.txt");
        assert_eq!(1, part2(input));
    }
}
