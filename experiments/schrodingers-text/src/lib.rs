use rand::prelude::IndexedRandom;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    pub fn all() -> [Direction; 4] {
        [Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }
}

#[derive(Clone, Debug)]
pub struct Superposition {
    pub possibilities: Vec<char>,
}

impl Superposition {
    pub fn new(chars: Vec<char>) -> Self {
        let mut unique_chars = chars;
        unique_chars.sort();
        unique_chars.dedup();
        Self { possibilities: unique_chars }
    }

    pub fn entropy(&self) -> usize {
        self.possibilities.len()
    }

    pub fn is_collapsed(&self) -> bool {
        self.possibilities.len() == 1
    }

    // Observe now takes weights to bias the choice
    pub fn observe(&mut self, weights: &HashMap<char, usize>) -> Option<char> {
        if self.possibilities.is_empty() {
            return None;
        }

        let mut rng = rand::rng();

        // Choose based on weight
        let chosen = self.possibilities.choose_weighted(&mut rng, |c| {
            // Default to weight 1 if unknown, though that shouldn't happen with correct extraction
            *weights.get(c).unwrap_or(&1)
        });

        match chosen {
            Ok(&c) => {
                self.possibilities = vec![c];
                Some(c)
            },
            Err(_) => {
                // Fallback (e.g. if all weights are 0, which is weird)
                if let Some(&c) = self.possibilities.choose(&mut rng) {
                    self.possibilities = vec![c];
                    Some(c)
                } else {
                    None
                }
            }
        }
    }

    // Returns true if possibilities changed
    pub fn constrain(&mut self, allowed: &HashSet<char>) -> bool {
        let original_len = self.possibilities.len();
        self.possibilities.retain(|c| allowed.contains(c));
        self.possibilities.len() != original_len
    }
}

#[derive(Clone, Debug)]
pub struct Rules {
    // A -> Direction -> [Allowed Neighbors]
    pub adjacency: HashMap<char, HashMap<Direction, HashSet<char>>>,
    pub all_chars: HashSet<char>,
    // Global frequency/weight of each character found in input
    pub weights: HashMap<char, usize>,
}

impl Default for Rules {
    fn default() -> Self {
        Self::new()
    }
}

impl Rules {
    pub fn new() -> Self {
        Self {
            adjacency: HashMap::new(),
            all_chars: HashSet::new(),
            weights: HashMap::new(),
        }
    }

    pub fn check_adjacency(&self, from: char, to: char, dir: Direction) -> bool {
        self.adjacency
            .get(&from)
            .and_then(|dir_map| dir_map.get(&dir))
            .is_some_and(|allowed| allowed.contains(&to))
    }

    pub fn add_rule(&mut self, from: char, to: char, dir: Direction) {
        self.all_chars.insert(from);
        self.all_chars.insert(to);

        self.adjacency
            .entry(from)
            .or_default()
            .entry(dir)
            .or_default()
            .insert(to);

        self.adjacency
            .entry(to)
            .or_default()
            .entry(dir.opposite())
            .or_default()
            .insert(from);
    }

    pub fn record_frequency(&mut self, c: char) {
        *self.weights.entry(c).or_insert(0) += 1;
    }

    pub fn get_allowed_neighbors(&self, from: char, dir: Direction) -> HashSet<char> {
        self.adjacency
            .get(&from)
            .and_then(|dir_map| dir_map.get(&dir))
            .cloned()
            .unwrap_or_default()
    }
}

pub struct PatternExtractor;

impl PatternExtractor {
    pub fn from_text(text: &str) -> Rules {
        let mut rules = Rules::new();
        let lines: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
        let height = lines.len();

        for y in 0..height {
            let width = lines[y].len();
            for x in 0..width {
                let current = lines[y][x];

                // Record weight
                rules.record_frequency(current);

                // Check Right (x+1)
                if x + 1 < width {
                    let right = lines[y][x+1];
                    rules.add_rule(current, right, Direction::Right);
                }

                // Check Down (y+1)
                if y + 1 < height {
                    // Ensure the line below has a character at this x
                    if x < lines[y+1].len() {
                        let down = lines[y+1][x];
                        rules.add_rule(current, down, Direction::Down);
                    }
                }
            }
        }
        rules
    }
}

pub struct WaveFunction {
    pub grid: Vec<Vec<Superposition>>,
    pub width: usize,
    pub height: usize,
    pub rules: Rules,
}

impl WaveFunction {
    pub fn new(width: usize, height: usize, rules: Rules) -> Self {
        let all_chars_vec: Vec<char> = rules.all_chars.iter().cloned().collect();
        Self {
             grid: vec![vec![Superposition::new(all_chars_vec); width]; height],
             width,
             height,
             rules
        }
    }

    pub fn get_entropy(&self, x: usize, y: usize) -> usize {
        self.grid[y][x].entropy()
    }

    pub fn collapse(&mut self) -> bool {
        // 1. Find cell with min entropy > 1
        let mut min_entropy = usize::MAX;
        let mut candidates = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let ent = self.grid[y][x].entropy();
                if ent > 1 {
                    if ent < min_entropy {
                        min_entropy = ent;
                        candidates.clear();
                        candidates.push((x, y));
                    } else if ent == min_entropy {
                        candidates.push((x, y));
                    }
                }
            }
        }

        if candidates.is_empty() {
            return false; // Fully collapsed or failed
        }

        // 2. Pick random candidate and observe (using weighted collapse)
        let mut rng = rand::rng();
        let (cx, cy) = *candidates.choose(&mut rng).unwrap();
        self.grid[cy][cx].observe(&self.rules.weights);

        // 3. Propagate
        self.propagate(cx, cy);

        true
    }

    fn propagate(&mut self, start_x: usize, start_y: usize) {
        let mut queue = VecDeque::new();
        queue.push_back((start_x, start_y));

        while let Some((cx, cy)) = queue.pop_front() {
            let current_possibilities = self.grid[cy][cx].possibilities.clone();

            for dir in Direction::all() {
                let (dx, dy) = dir.delta();
                let nx = cx as isize + dx;
                let ny = cy as isize + dy;

                // Bounds check
                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    // Calculate allowed options for neighbor based on current cell's possibilities
                    let mut allowed_for_neighbor = HashSet::new();
                    for &p in &current_possibilities {
                        let allowed = self.rules.get_allowed_neighbors(p, dir);
                        allowed_for_neighbor.extend(allowed);
                    }

                    // Constrain neighbor
                    if self.grid[ny][nx].constrain(&allowed_for_neighbor) {
                        queue.push_back((nx, ny));

                        // If neighbor became empty, we have a contradiction
                        if self.grid[ny][nx].entropy() == 0 {
                            // Glitch out?
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_superposition_entropy() {
        let chars = vec!['a', 'b', 'c'];
        let sup = Superposition::new(chars.clone());
        assert_eq!(sup.entropy(), 3);
        assert!(!sup.is_collapsed());
    }

    #[test]
    fn test_superposition_collapse() {
        let chars = vec!['a', 'b'];
        let mut weights = HashMap::new();
        weights.insert('a', 1);
        weights.insert('b', 1);

        let mut sup = Superposition::new(chars);
        let observed = sup.observe(&weights);
        assert!(observed.is_some());
        assert!(sup.is_collapsed());
        assert_eq!(sup.entropy(), 1);
    }

    #[test]
    fn test_pattern_extraction() {
        let text = "AB\nCD";
        let rules = PatternExtractor::from_text(text);

        assert!(rules.check_adjacency('A', 'B', Direction::Right));
        assert!(rules.check_adjacency('B', 'A', Direction::Left));
        assert!(rules.check_adjacency('A', 'C', Direction::Down));
        assert!(rules.check_adjacency('C', 'A', Direction::Up));
        assert!(!rules.check_adjacency('A', 'A', Direction::Right));

        // Check weights
        assert_eq!(*rules.weights.get(&'A').unwrap(), 1);
    }

    #[test]
    fn test_wave_function_initialization() {
        let text = "ABC";
        let rules = PatternExtractor::from_text(text);
        let wave = WaveFunction::new(10, 10, rules);

        assert_eq!(wave.get_entropy(0, 0), 3);
    }

    #[test]
    fn test_wave_function_propagation() {
        let text = "AB"; // A must be left of B
        let rules = PatternExtractor::from_text(text);
        let mut wave = WaveFunction::new(2, 1, rules);

        // Initial state: Both (0,0) and (1,0) can be A or B
        assert_eq!(wave.get_entropy(0, 0), 2);
        assert_eq!(wave.get_entropy(1, 0), 2);

        // Force collapse (0,0) to 'A'
        // Hack: Manually observe to test propagation deterministically
        wave.grid[0][0].possibilities = vec!['A'];
        wave.propagate(0, 0);

        // (0,0) is A. Rule says Right neighbor of A must be B.
        // So (1,0) should collapse to B.

        assert_eq!(wave.grid[0][1].possibilities, vec!['B']);
    }
}
