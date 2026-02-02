use glam::Vec2;

#[derive(Clone, Debug)]
pub struct Strand {
    pub start: Vec2,
    pub end: Vec2,
    pub age: f32,
}

pub struct Silk {
    pub strands: Vec<Strand>,
    pub last_pos: Option<Vec2>,
}

impl Silk {
    pub fn new() -> Self {
        Self {
            strands: Vec::new(),
            last_pos: None,
        }
    }

    pub fn clear(&mut self) {
        self.strands.clear();
        self.last_pos = None;
    }

    pub fn add_strand(&mut self, current_pos: Vec2) {
        if let Some(last) = self.last_pos {
            let dist = last.distance(current_pos);
            // Only add strand if we moved enough
            if dist > 2.0 {
                self.strands.push(Strand {
                    start: last,
                    end: current_pos,
                    age: 0.0,
                });
                self.last_pos = Some(current_pos);
            }
        } else {
            self.last_pos = Some(current_pos);
        }
    }

    pub fn update(&mut self, dt: f32) {
        for strand in &mut self.strands {
            strand.age += dt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_strand() {
        let mut silk = Silk::new();
        let p1 = Vec2::new(0.0, 0.0);
        let p2 = Vec2::new(1.0, 0.0); // Distance 1.0 < 2.0
        let p3 = Vec2::new(3.0, 0.0); // Distance 3.0 from last (0,0) > 2.0

        silk.add_strand(p1);
        assert_eq!(silk.strands.len(), 0);
        assert_eq!(silk.last_pos, Some(p1));

        silk.add_strand(p2);
        assert_eq!(silk.strands.len(), 0); // Not enough distance from p1
                                           // last_pos should NOT update if we didn't add a strand?
                                           // Actually, if we are moving continuously, we should probably only update last_pos when we emit.
                                           // My implementation does: if dist > 2.0 { push; last = current }
                                           // So p2 is ignored. last_pos is still p1.

        silk.add_strand(p3);
        // dist(p1, p3) = 3.0 > 2.0. Should add strand (p1, p3).
        assert_eq!(silk.strands.len(), 1);
        assert_eq!(silk.last_pos, Some(p3));

        let s = &silk.strands[0];
        assert_eq!(s.start, p1);
        assert_eq!(s.end, p3);
    }

    #[test]
    fn test_clear() {
        let mut silk = Silk::new();
        silk.add_strand(Vec2::new(0.0, 0.0));
        silk.add_strand(Vec2::new(5.0, 0.0));
        assert_eq!(silk.strands.len(), 1);

        silk.clear();
        assert_eq!(silk.strands.len(), 0);
        assert!(silk.last_pos.is_none());
    }
}
