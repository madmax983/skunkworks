use poincare_disk::Point;
use ratatui::style::Color;

#[derive(Clone, Debug, PartialEq)]
pub enum EntityKind {
    Player,
    Enemy,
    Item,
}

#[derive(Clone, Debug)]
pub struct Entity {
    pub id: usize,
    pub path: Vec<usize>,
    pub offset: Point,
    pub kind: EntityKind,
    pub hp: i32,
    pub max_hp: i32,
    pub symbol: char,
    pub color: Color,
}

impl Entity {
    pub fn new(id: usize, path: Vec<usize>, kind: EntityKind) -> Self {
        match kind {
            EntityKind::Player => Self {
                id,
                path,
                offset: Point::new(0.0, 0.0),
                kind,
                hp: 100,
                max_hp: 100,
                symbol: '@',
                color: Color::Yellow,
            },
            EntityKind::Enemy => Self {
                id,
                path,
                offset: Point::new(0.0, 0.0),
                kind,
                hp: 20,
                max_hp: 20,
                symbol: 'r',
                color: Color::Red,
            },
            EntityKind::Item => Self {
                id,
                path,
                offset: Point::new(0.0, 0.0),
                kind,
                hp: 0,
                max_hp: 0,
                symbol: '!',
                color: Color::Magenta,
            },
        }
    }
}
