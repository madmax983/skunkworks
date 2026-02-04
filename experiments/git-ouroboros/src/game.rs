use crate::git::{Commit, GitHistory};
use rand::Rng;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_point(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    pub body: VecDeque<Point>,
    pub direction: Direction,
    pub grow_pending: usize,
}

impl Snake {
    pub fn new(start: Point, length: usize) -> Self {
        let mut body = VecDeque::new();
        // Havoc Fix: Ensure minimum length of 1 to prevent head() panic
        let length = length.max(1);
        for i in 0..length {
            body.push_back(Point::new(start.x - i as i32, start.y));
        }
        Self {
            body,
            direction: Direction::Right,
            grow_pending: 0,
        }
    }

    pub fn head(&self) -> Point {
        *self.body.front().unwrap()
    }

    pub fn move_forward(&mut self, width: i32, height: i32) -> bool {
        let head = self.head();
        let delta = self.direction.to_point();
        let mut new_head = Point::new(head.x + delta.x, head.y + delta.y);

        // Wrap around logic (optional) or Wall collision?
        // Let's implement Wall Collision as Game Over first, or Wrap.
        // Wrap is more fun.
        if new_head.x < 0 {
            new_head.x = width - 1;
        }
        if new_head.x >= width {
            new_head.x = 0;
        }
        if new_head.y < 0 {
            new_head.y = height - 1;
        }
        if new_head.y >= height {
            new_head.y = 0;
        }

        // Self collision check
        if self.body.contains(&new_head) {
            // Exception: if we are moving, the tail will move, so if new_head == tail, it's fine UNLESS we are growing.
            // But simpler to just say if it's in the body (excluding tail if we are not growing), it's a hit.
            // If grow_pending > 0, tail doesn't move.
            // If grow_pending == 0, tail moves.
            let tail = self.body.back().unwrap();
            if &new_head != tail || self.grow_pending > 0 {
                return false; // Collision
            }
        }

        self.body.push_front(new_head);

        if self.grow_pending > 0 {
            self.grow_pending -= 1;
        } else {
            self.body.pop_back();
        }

        true
    }

    pub fn grow(&mut self, amount: usize) {
        self.grow_pending += amount;
    }
}

pub struct Food {
    pub position: Point,
    pub commit: Commit,
}

pub struct World {
    pub width: i32,
    pub height: i32,
    pub snake: Snake,
    pub food: Option<Food>,
    pub history: GitHistory,
    pub next_commit_index: usize,
    pub score: usize,
    pub game_over: bool,
    pub last_eaten_commits: VecDeque<Commit>,
}

impl World {
    pub fn new(width: i32, height: i32, history: GitHistory) -> Self {
        let snake = Snake::new(Point::new(width / 2, height / 2), 3);
        let mut world = Self {
            width,
            height,
            snake,
            food: None,
            history,
            next_commit_index: 0,
            score: 0,
            game_over: false,
            last_eaten_commits: VecDeque::with_capacity(5),
        };
        world.spawn_food();
        world
    }

    pub fn update(&mut self) {
        if self.game_over {
            return;
        }

        if !self.snake.move_forward(self.width, self.height) {
            self.game_over = true;
            return;
        }

        if let Some(ref food) = self.food {
            if self.snake.head() == food.position {
                // Eat
                self.snake.grow(1);
                self.score += 1;

                // Log eaten commit
                if self.last_eaten_commits.len() >= 5 {
                    self.last_eaten_commits.pop_back();
                }
                self.last_eaten_commits.push_front(food.commit.clone());

                self.spawn_food();
            }
        }
    }

    fn spawn_food(&mut self) {
        if self.next_commit_index >= self.history.commits.len() {
            // Victory or Loop?
            // Let's loop for infinite gameplay, maybe randomized?
            // Or just clear food.
            self.food = None;
            return;
        }

        let commit = self.history.commits[self.next_commit_index].clone();
        self.next_commit_index += 1;

        let mut rng = rand::thread_rng();
        let mut position;
        loop {
            position = Point::new(rng.gen_range(0..self.width), rng.gen_range(0..self.height));
            if !self.snake.body.contains(&position) {
                break;
            }
        }

        self.food = Some(Food { position, commit });
    }

    pub fn change_direction(&mut self, direction: Direction) {
        if direction != self.snake.direction.opposite() {
            self.snake.direction = direction;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::Commit;

    #[test]
    fn test_snake_move() {
        let mut snake = Snake::new(Point::new(5, 5), 3);
        // Head at 5,5, Tail at 3,5. Direction Right.
        assert_eq!(snake.head(), Point::new(5, 5));

        snake.move_forward(10, 10);
        assert_eq!(snake.head(), Point::new(6, 5));
        assert_eq!(snake.body.len(), 3);
    }

    #[test]
    fn test_snake_grow() {
        let mut snake = Snake::new(Point::new(5, 5), 3);
        snake.grow(1);
        snake.move_forward(10, 10);
        assert_eq!(snake.body.len(), 4);
    }

    #[test]
    fn test_world_update() {
        let history = GitHistory {
            commits: vec![Commit {
                hash: "123".into(),
                author: "A".into(),
                message: "M".into(),
            }],
        };
        let mut world = World::new(10, 10, history);

        // Force food position to be in front of snake
        if let Some(ref mut food) = world.food {
            food.position = Point::new(6, 5); // Snake starts at 5,5 facing Right
        }

        world.update(); // Move to 6,5, should eat

        assert_eq!(world.score, 1);
        assert_eq!(world.snake.grow_pending, 1); // Pending growth
        assert_eq!(world.last_eaten_commits.len(), 1);

        world.update(); // Move again to realize growth
        assert_eq!(world.snake.body.len(), 4);
    }

    #[test]
    fn test_havoc_zero_length_snake_safe() {
        // Havoc: Injecting 0-length snake should now be safe (clamped to 1)
        let snake = Snake::new(Point::new(0, 0), 0);
        let _ = snake.head(); // Should not panic
        assert_eq!(snake.body.len(), 1);
    }
}
