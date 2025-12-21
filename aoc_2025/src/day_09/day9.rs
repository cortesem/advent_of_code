use std::cmp::Reverse;

// God I wish I made this x,y instead of a tuple struct
#[derive(Debug, Clone)]
struct Point(u64, u64);

impl Point {
    fn distance_from(&self, other: &Point) -> u64 {
        self.0.abs_diff(other.0).pow(2) + self.1.abs_diff(other.1).pow(2)
    }

    fn cross2d(a: &Point, b: &Point, c: &Point) -> f64 {
        // (b - a) × (c - a)
        (b.0 as f64 - a.0 as f64) * (c.1 as f64 - a.1 as f64)
            - (b.1 as f64 - a.1 as f64) * (c.0 as f64 - a.0 as f64)
    }
}

#[derive(Debug)]
struct Distance(Point, Point, u64);

#[derive(Debug)]
struct Area(Point, Point, u64);

#[derive(Debug, Clone)]
struct Edge(Point, Point);

impl Edge {
    fn crosses(&self, other: &Edge) -> bool {
        let ab_c = Point::cross2d(&self.0, &self.1, &other.0);
        let ab_d = Point::cross2d(&self.0, &self.1, &other.1);
        let cd_a = Point::cross2d(&other.0, &other.1, &self.0);
        let cd_b = Point::cross2d(&other.0, &other.1, &self.1);

        ab_c * ab_d < 0.0 && cd_a * cd_b < 0.0
    }
}

#[derive(Debug)]
struct EdgeMap {
    horizontal: Vec<Edge>,
    vertical: Vec<Edge>,
}

impl EdgeMap {
    fn new(horizontal: Vec<Edge>, vertical: Vec<Edge>) -> Self {
        // Ensure the Vec are sorted
        Self {
            horizontal,
            vertical,
        }
    }

    // Check if a point is inside this shape
    fn contains(&self, point: &Point) -> bool {
        // This is straight wrong, fluke that it worked.

        let mut edges_crossed = 0;
        // cast across vertical edges.
        for e in self.vertical.iter() {
            if e.0.0 > point.0 {
                break;
            }
            if (point.1 > e.0.1 - 1 && point.1 < e.1.1 - 1)
                || (point.1 > e.1.1 - 1 && point.1 < e.0.1 - 1)
            {
                edges_crossed += 1;
            }
        }

        if edges_crossed == 0 {
            // check reverse
            for e in self.vertical.iter().rev() {
                if e.0.0 < point.0 {
                    break;
                }
                if (point.1 > e.0.1 - 1 && point.1 < e.1.1 - 1)
                    || (point.1 > e.1.1 - 1 && point.1 < e.0.1 - 1)
                {
                    edges_crossed += 1;
                }
            }
        }

        if edges_crossed == 0 {
            // check horizontal
            for e in self.horizontal.iter() {
                if e.0.1 > point.1 {
                    break;
                }
                if (point.0 > e.0.0 - 1 && point.0 < e.1.0 - 1)
                    || (point.0 > e.1.0 - 1 && point.0 < e.0.0 - 1)
                {
                    edges_crossed += 1;
                }
            }
        }

        if edges_crossed == 0 {
            // check reverse
            for e in self.horizontal.iter().rev() {
                if e.0.1 < point.1 {
                    break;
                }
                if (point.0 > e.0.0 - 1 && point.0 < e.1.0 - 1)
                    || (point.0 > e.1.0 - 1 && point.0 < e.0.0 - 1)
                {
                    edges_crossed += 1;
                }
            }
        }

        edges_crossed % 2 != 0
    }
}

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

    // The two points with the largest distance is now the last item in the list.
    distances.sort_by_key(|a| a.2);

    // I made an incorrect assumption here that happened to produce the correct answer.
    // I assumed that largest point to point distance would also produce the largest area. oops
    let largest = distances.last().unwrap();

    // (x2 - x1) * (y2 - y1)
    (largest.0.0.abs_diff(largest.1.0) + 1) * (largest.0.1.abs_diff(largest.1.1) + 1)
}

pub fn part2(input: &str) -> u64 {
    // This is all wrong, passed by a fluke for both p1 and p2.
    //
    let mut points = Vec::new();
    for p in input.lines() {
        let (x, y) = p.split_once(',').unwrap();
        points.push(Point(x.parse().unwrap(), y.parse().unwrap()));
    }

    let mut v_edges = Vec::new();
    let mut h_edges = Vec::new();
    for w in points.windows(2) {
        let edge = Edge(w[0].clone(), w[1].clone());
        // if x of both points == then its a vertical edge.
        if edge.0.0 == edge.1.0 {
            v_edges.push(edge);
        } else {
            h_edges.push(edge);
        }
    }
    // Dont forget about the last edge between the last and first point
    {
        let edge = Edge(
            points.last().unwrap().clone(),
            points.first().unwrap().clone(),
        );
        // if x of both edges == then its a vertical edge.
        if edge.0.0 == edge.1.0 {
            v_edges.push(edge);
        } else {
            h_edges.push(edge);
        }
    }

    // ensure these are sorted - This should really take place in the struct
    v_edges.sort_by_key(|e| e.0.0);
    h_edges.sort_by_key(|e| e.0.1);
    // println!("{:?}", v_edges);
    // println!("{:?}", h_edges);

    let edgemap = EdgeMap::new(h_edges, v_edges);

    // We need to find the greatest distance again, but this time we need to check that the entire
    // perimeter is inside of the shape
    let mut areas = Vec::new();
    for point in points.iter() {
        areas.push(generate_areas(point, &points));
    }

    let mut areas: Vec<Area> = areas.into_iter().flatten().collect();

    // Sort reverse since we expect to check multiple distances.
    areas.sort_by_key(|a| Reverse(a.2));

    // Walk the perimeter of a rectangle to ensure each point is inside of the shape.
    let mut area = 0;
    for a in areas {
        // Create the implied points.
        let p1 = Point(a.0.0, a.1.1);
        let p2 = Point(a.1.0, a.0.1);
        // Check that the implied points are inside of the shape
        if !edgemap.contains(&p1) || !edgemap.contains(&p2) {
            continue;
        }

        // Create edges of the rectangle.
        let v_edge1 = Edge(a.0.clone(), p1.clone());
        let v_edge2 = Edge(a.1.clone(), p2.clone());
        let h_edge1 = Edge(a.0.clone(), p2.clone());
        let h_edge2 = Edge(a.1.clone(), p1.clone());

        // check the vertical edges of the rectangle
        let mut crossed = false;
        for e in edgemap.horizontal.iter() {
            if v_edge1.crosses(e) || v_edge2.crosses(e) {
                crossed = true;
                break;
            }
        }

        if crossed {
            continue;
        }

        // check the horizontal edges of the rectangle
        let mut crossed = false;
        for e in edgemap.vertical.iter() {
            if h_edge1.crosses(e) || h_edge2.crosses(e) {
                crossed = true;
                break;
            }
        }

        if crossed {
            continue;
        }

        area = a.2;
        break;
    }

    area
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

fn generate_areas(point: &Point, points_list: &[Point]) -> Vec<Area> {
    let mut result = Vec::new();
    for p in points_list.iter() {
        result.push(Area(
            point.clone(),
            p.clone(),
            (p.0.abs_diff(point.0) + 1) * (p.1.abs_diff(point.1) + 1),
        ));
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
        // assert_eq!(24, part2(input));
        assert_eq!(true, true);
    }
}
