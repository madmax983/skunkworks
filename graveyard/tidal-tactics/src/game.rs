use crate::map::{Map, HEIGHT, WIDTH};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UnitType {
    Tank,
    Hovercraft,
    #[allow(dead_code)]
    Engineer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Team {
    Player,
    Enemy,
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub x: usize,
    pub y: usize,
    pub unit_type: UnitType,
    pub team: Team,
    pub moves_left: i32,
    #[allow(dead_code)]
    pub health: f32,
}

impl Unit {
    pub fn new(x: usize, y: usize, unit_type: UnitType, team: Team) -> Self {
        Unit {
            x,
            y,
            unit_type,
            team,
            moves_left: 2, // Standard movement points
            health: 100.0,
        }
    }
}

pub struct GameState {
    pub map: Map,
    pub units: Vec<Unit>,
    pub turn: usize,
    pub phase: TurnPhase,
    pub selected_unit: Option<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnPhase {
    PlayerInput,
    FlowSimulation,
}

impl GameState {
    pub fn new() -> Self {
        let map = Map::new(WIDTH, HEIGHT);
        let mut game = GameState {
            map,
            units: Vec::new(),
            turn: 1,
            phase: TurnPhase::PlayerInput,
            selected_unit: None,
        };

        // Spawn some initial units
        game.spawn_unit(5, 5, UnitType::Tank, Team::Player);
        game.spawn_unit(6, 6, UnitType::Hovercraft, Team::Player);
        game.spawn_unit(WIDTH - 5, HEIGHT - 5, UnitType::Tank, Team::Enemy);

        game
    }

    pub fn spawn_unit(&mut self, x: usize, y: usize, unit_type: UnitType, team: Team) {
        if x < WIDTH && y < HEIGHT {
            self.units.push(Unit::new(x, y, unit_type, team));
        }
    }

    #[allow(dead_code)]
    pub fn try_select_unit(&mut self, x: usize, y: usize) {
        // Simple selection: find first unit at x,y
        self.selected_unit = self
            .units
            .iter()
            .position(|u| u.x == x && u.y == y && u.team == Team::Player);
    }

    pub fn try_move_selected(&mut self, tx: usize, ty: usize) -> bool {
        if let Some(idx) = self.selected_unit {
            let unit = &self.units[idx];

            // Validate move
            let dx = (tx as i32 - unit.x as i32).abs();
            let dy = (ty as i32 - unit.y as i32).abs();

            // Allow only adjacent moves (including diagonal? Let's say no diagonal for now to be simple, or yes)
            // Let's allow King moves (1 tile any dir)
            if dx > 1 || dy > 1 || (dx == 0 && dy == 0) {
                return false;
            }

            if unit.moves_left <= 0 {
                return false;
            }

            // Check Terrain/Water
            let target_idx = ty * WIDTH + tx;
            let water_depth = self.map.water[target_idx];
            let terrain_height = self.map.terrain[target_idx];

            // Tank limit
            if unit.unit_type == UnitType::Tank {
                if water_depth > 0.5 {
                    return false; // Too deep
                }
                if terrain_height > 8.0 {
                    return false; // Too steep/high
                }
            }

            // Hovercraft limit
            if unit.unit_type == UnitType::Hovercraft {
                // Can cross water but maybe not high walls
                if terrain_height > 10.0 {
                    return false;
                }
            }

            // Move
            self.units[idx].x = tx;
            self.units[idx].y = ty;
            self.units[idx].moves_left -= 1;
            return true;
        }
        false
    }

    pub fn end_turn(&mut self) {
        // Reset movement
        for unit in &mut self.units {
            unit.moves_left = 2;
        }
        self.phase = TurnPhase::FlowSimulation;
        // In real loop, we'll wait for N seconds then switch back
    }
}
