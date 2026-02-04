use crate::game::World;
use tui_semantic::{Entity, Snapshot};

pub trait ToSnapshot {
    fn to_snapshot(&self) -> Snapshot;
}

impl ToSnapshot for World {
    fn to_snapshot(&self) -> Snapshot {
        let mut snapshot = Snapshot::new("git-ouroboros")
            .with_viewport(self.width as u16, self.height as u16)
            .with_metric("score", self.score)
            .with_metric("snake_length", self.snake.body.len())
            .with_state(if self.game_over {
                "game_over"
            } else {
                "playing"
            });

        // Add Snake Head
        let head = self.snake.head();
        snapshot = snapshot.with_entity(
            Entity::new("snake_head")
                .at(head.x as f64, head.y as f64)
                .display("H"),
        );

        // Add Snake Body
        // We skip the first one because it's the head (already added)
        for (i, segment) in self.snake.body.iter().skip(1).enumerate() {
            snapshot = snapshot.with_entity(
                Entity::new("snake_body")
                    .at(segment.x as f64, segment.y as f64)
                    .with_prop("index", i)
                    .display("B"),
            );
        }

        // Add Food
        if let Some(ref food) = self.food {
            snapshot = snapshot.with_entity(
                Entity::new("food")
                    .at(food.position.x as f64, food.position.y as f64)
                    .display("F")
                    .with_prop("commit_hash", food.commit.hash.clone())
                    .with_prop("commit_author", food.commit.author.clone())
                    .with_prop("commit_message", food.commit.message.clone()),
            );
        }

        snapshot
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::{Point, Snake};
    use crate::git::{Commit, GitHistory};

    #[test]
    fn test_world_to_snapshot() {
        let history = GitHistory {
            commits: vec![Commit {
                hash: "abcdef123".into(),
                author: "Nova".into(),
                message: "Initial commit".into(),
            }],
        };
        let mut world = World::new(20, 10, history);

        // Manually position snake for predictability
        // Snake::new puts head at width/2, height/2. (10, 5)
        // Length 3. Body: (10,5), (9,5), (8,5).
        // Let's force it just in case logic changes.
        world.snake = Snake::new(Point::new(10, 5), 3);

        // Manually place food
        if let Some(ref mut food) = world.food {
            food.position = Point::new(15, 5);
        }

        let snapshot = world.to_snapshot();

        assert_eq!(snapshot.app, "git-ouroboros");
        assert_eq!(snapshot.viewport, Some((20, 10)));

        // Check Metrics
        match snapshot.metrics.get("score") {
            Some(tui_semantic::PropValue::Int(v)) => assert_eq!(*v, 0),
            _ => panic!("Score metric missing or wrong type"),
        }

        // Check Entities
        // 1 Head + 2 Body + 1 Food = 4 entities
        assert_eq!(snapshot.entities.len(), 4);

        // Verify Head
        let head = snapshot
            .entities
            .iter()
            .find(|e| e.kind == "snake_head")
            .expect("Head not found");
        let head_pos = head.position.expect("Head has no position");
        assert_eq!(head_pos.x, 10.0);
        assert_eq!(head_pos.y, 5.0);

        // Verify Food
        let food = snapshot
            .entities
            .iter()
            .find(|e| e.kind == "food")
            .expect("Food not found");
        let food_pos = food.position.expect("Food has no position");
        assert_eq!(food_pos.x, 15.0);
        assert_eq!(food_pos.y, 5.0);

        // Verify Food Props
         match food.props.get("commit_author") {
            Some(tui_semantic::PropValue::Text(s)) => assert_eq!(s, "Nova"),
            _ => panic!("commit_author missing or wrong type"),
        }
    }
}
