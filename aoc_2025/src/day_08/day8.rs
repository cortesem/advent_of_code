use std::{cmp::Reverse, collections::HashSet};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Position(i64, i64, i64);

impl Position {
    fn distance_to(&self, other: &Position) -> f64 {
        (((other.0 - self.0).pow(2) + (other.1 - self.1).pow(2) + (other.2 - self.2).pow(2)) as f64)
            .sqrt()
    }
}

#[derive(Debug)]
struct Connection(Position, Position, f64);

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.2 == other.2
            && (self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0)
    }
}

pub fn part1(input: &str, num_of_conn: usize) -> u64 {
    // get all positions
    let mut locations: Vec<Position> = vec![];
    for location in input.lines() {
        let xyz: Vec<&str> = location.split(',').collect();
        locations.push(Position(
            xyz[0].parse().unwrap(),
            xyz[1].parse().unwrap(),
            xyz[2].parse().unwrap(),
        ));
    }

    // calculate distances for each position
    let mut positions_with_distance = Vec::<Vec<Connection>>::new();
    for position in locations.iter() {
        positions_with_distance.push(calc_distances(position, &locations));
    }
    let mut positions_with_distance: Vec<Connection> =
        positions_with_distance.into_iter().flatten().collect();
    // this might contain duplicates when two points are both closest to eachother.
    positions_with_distance.sort_by(|a, b| a.2.total_cmp(&b.2));
    positions_with_distance.dedup();

    // create the connections
    let mut connections = get_n_closest_connections(&positions_with_distance, num_of_conn);
    connections.sort_by_key(|a| Reverse(a.len()));

    connections
        .iter()
        .take(3)
        .map(|c| c.len())
        .product::<usize>() as u64
}

pub fn part2(input: &str) -> u64 {
    // get all positions
    let mut locations: Vec<Position> = vec![];
    for location in input.lines() {
        let xyz: Vec<&str> = location.split(',').collect();
        locations.push(Position(
            xyz[0].parse().unwrap(),
            xyz[1].parse().unwrap(),
            xyz[2].parse().unwrap(),
        ));
    }

    // calculate distances for each position
    let mut positions_with_distance = Vec::<Vec<Connection>>::new();
    for position in locations.iter() {
        positions_with_distance.push(calc_distances(position, &locations));
    }
    let mut positions_with_distance: Vec<Connection> =
        positions_with_distance.into_iter().flatten().collect();
    // this might contain duplicates when two points are both closest to eachother.
    positions_with_distance.sort_by(|a, b| a.2.total_cmp(&b.2));
    positions_with_distance.dedup();

    create_full_circuit(&positions_with_distance, locations.len())
}

// calculate distances between a position and every other position. Returns a list of distances to
// the other positions, sorted ascending order.
fn calc_distances(box_position: &Position, other_boxes: &[Position]) -> Vec<Connection> {
    let mut result = Vec::<Connection>::with_capacity(other_boxes.len());

    for other_position in other_boxes {
        result.push(Connection(
            box_position.clone(),
            other_position.clone(),
            box_position.distance_to(other_position),
        ));
    }

    // sort and remove the first element since it will be a 0 ie. the distance to itself
    result.sort_by(|a, b| a.2.total_cmp(&b.2));
    result.remove(0);

    result
}

fn get_n_closest_connections(
    connections: &[Connection],
    num_of_conn: usize,
) -> Vec<HashSet<Position>> {
    let mut result = Vec::<HashSet<Position>>::new();

    for connection in connections.iter().take(num_of_conn) {
        let mut found = false;
        for circuit in result.iter_mut() {
            if circuit.contains(&connection.0) || circuit.contains(&connection.1) {
                found = true;
                circuit.insert(connection.0.clone());
                circuit.insert(connection.1.clone());
                break;
            }
        }

        if !found {
            let mut circuit = HashSet::new();
            circuit.insert(connection.0.clone());
            circuit.insert(connection.1.clone());
            result.push(circuit);
        } else {
            // check for any circuits that have an intersection
            for i in 0..result.len() - 1 {
                for j in (i + 1)..result.len() {
                    if !result.get(i).unwrap().is_disjoint(result.get(j).unwrap()) {
                        let b = result.remove(j);
                        let mut a = result.remove(i);
                        a.extend(b);
                        result.push(a);
                        break;
                    }
                }
            }
        }
    }

    result
}

fn create_full_circuit(connections: &[Connection], num_of_junctions: usize) -> u64 {
    let mut result = Vec::<HashSet<Position>>::new();

    for connection in connections.iter() {
        let mut found = false;
        for circuit in result.iter_mut() {
            if circuit.contains(&connection.0) || circuit.contains(&connection.1) {
                found = true;
                circuit.insert(connection.0.clone());
                circuit.insert(connection.1.clone());
                break;
            }
        }

        if !found {
            let mut circuit = HashSet::new();
            circuit.insert(connection.0.clone());
            circuit.insert(connection.1.clone());
            result.push(circuit);
        } else {
            // check for any circuits that have an intersection
            for i in 0..result.len() - 1 {
                for j in (i + 1)..result.len() {
                    if !result.get(i).unwrap().is_disjoint(result.get(j).unwrap()) {
                        let b = result.remove(j);
                        let mut a = result.remove(i);
                        a.extend(b);
                        result.push(a);
                        break;
                    }
                }
            }
        }
        // Now we need to check if we have a single circuit of 1000
        if result.len() == 1 && result.first().unwrap().len() == num_of_junctions {
            return (connection.0.0 * connection.1.0) as u64;
        }
    }

    1
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
        assert_eq!(25272, part2(input));
    }
}
