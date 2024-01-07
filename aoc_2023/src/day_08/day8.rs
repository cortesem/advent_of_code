use ahash::AHashMap;

struct Network {
    instructions: String,
    network: AHashMap<String, (String, String)>,
    // network_cache: AHashMap<String, String>, -> I tried a caching solution but it was slow
}

impl Network {
    fn new(s: &str) -> Self {
        let (instructions, nodes) = s.split_once("\n\n").unwrap();
        let instructions = instructions.trim();
        let nodes = nodes.trim();
        let mut network = AHashMap::new();

        for line in nodes.split('\n').rev() {
            let (node, next) = line.split_once(" = ").unwrap();
            let next: String = next
                .chars()
                .filter(|&c| c != '(' && c != ')' && c != ' ')
                .collect();
            let (left, right) = next.split_once(',').unwrap();
            network.insert(node.to_string(), (left.to_string(), right.to_string()));
        }

        Self {
            instructions: instructions.to_string(),
            network,
            // network_cache,
        }
    }

    // does one full iteration of the instructions, then checks if it's on the 'to' node
    fn count_steps(&self, from: &str, to: &str) -> u64 {
        let mut steps = 0;
        let mut node = from;
        let mut next = self.network.get(node).unwrap();
        while !node.ends_with(to) {
            for instruction in self.instructions.chars() {
                steps += 1;
                if instruction == 'L' {
                    node = &next.0;
                } else {
                    node = &next.1;
                }
                next = self.network.get(node).unwrap();
            }
        }
        steps
    }

    fn count_all_step(&self, from: &str, to: &str) -> u64 {
        let mut nodes_count = vec![];
        // get initial steps to each first ..Z
        for (node, _) in &self.network {
            if node.ends_with(from) {
                nodes_count.push(self.count_steps(&node, to));
            }
        }

        // find least common multiple of them all! We already know the gcd :)
        let gcd = self.instructions.len() as u64;
        let lcm: u64 = nodes_count.iter().map(|n| n / gcd).product();
        lcm * gcd
    }
}

pub fn solve_q8_p1(s: &str) -> u64 {
    let network = Network::new(s);
    network.count_steps("AAA", "ZZZ")
}

pub fn solve_q8_p2(s: &str) -> u64 {
    let network = Network::new(s);
    network.count_all_step("A", "Z")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_q8p1() {
        let (d8p2_test1, d8p2_test2) = include_str!("./input1_test.txt").split_once("---").unwrap();
        assert_eq!(solve_q8_p1(d8p2_test1), 2);
        assert_eq!(solve_q8_p1(d8p2_test2), 6);
    }

    #[test]
    fn test_solve_q8p2() {
        let d8p2_test = include_str!("./input2_test.txt");
        assert_eq!(solve_q8_p2(d8p2_test), 6);
    }

    #[test]
    fn test_answers() {
        let input = include_str!("./input.txt");
        assert_eq!(solve_q8_p1(input), 21797);
        assert_eq!(solve_q8_p2(input), 23977527174353);
    }
}
