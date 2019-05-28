use rand::Rng;
use tcod::input::{ KeyCode };

use crate::util::{ Contains, Point, Window, XPointRelation, YPointRelation, PointEquality };
use crate::game::Game;

pub trait MovementComponent {
    fn update(&self, point: Point) -> Point;
}

pub struct RandomMovementComponent { window: Window }

impl RandomMovementComponent {
    pub fn new(window: Window) -> RandomMovementComponent {
        RandomMovementComponent { window }
    }
}

impl MovementComponent for RandomMovementComponent {
    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        let offset_x = rand::thread_rng().gen_range(0, 3i32) - 1;
        match self.window.contains(offset.offset_x(offset_x)) {
            Contains::DoesContain => offset = offset.offset_x(offset_x),
            Contains::DoesNotContain => { return point; }
        }

        let offset_y = rand::thread_rng().gen_range(0, 3i32) - 1;
        match self.window.contains(offset.offset_y(offset_y)) {
            Contains::DoesContain => offset = offset.offset_y(offset_y),
            Contains::DoesNotContain => { return point; }
        }
        offset
    }
}

pub struct PlayerMovementComponent { window: Window }

impl PlayerMovementComponent {
    pub fn new(window: Window) -> PlayerMovementComponent {
        PlayerMovementComponent { window }
    }
}

impl MovementComponent for PlayerMovementComponent {
    fn update(&self, point: Point) -> Point {
        let mut offset = Point { x: point.x, y: point.y };
        offset = match Game::get_last_keypress() {
            Some(keypress) => {  
                match keypress.code {
                    KeyCode::Up => { offset.offset_y(-1) },
                    KeyCode::Down => { offset.offset_y(1) },
                    KeyCode::Left => { offset.offset_x(-1) },
                    KeyCode::Right => { offset.offset_x(1) },
                    _ => { offset },
                }
            },
            None => { offset }
        };

        match self.window.contains(offset) {
            Contains::DoesContain => { offset }
            Contains::DoesNotContain => { point }
        }
    }
}

pub struct AggroMovementComponent {
    window: Window
}

impl AggroMovementComponent {
    pub fn new(window: Window) -> AggroMovementComponent {
        AggroMovementComponent { window }
    }
}

impl MovementComponent for AggroMovementComponent {
    fn update(&self, point: Point) -> Point {
        let hero_point = Game::get_hero_point();
        let mut offset = Point { x: 0, y: 0 };
        offset = match point.compare_x(hero_point) {
            XPointRelation::RightOfPoint => offset.offset_x(-1),
            XPointRelation::LeftOfPoint  => offset.offset_x(1),
            XPointRelation::OnPointX => offset
        };

        offset = match point.compare_y(hero_point) {
            YPointRelation::BelowPoint => offset.offset_y(-1),
            YPointRelation::AbovePoint => offset.offset_y(1),
            YPointRelation::OnPointY   => offset
        };

        match point.offset(offset).compare(hero_point) {
            PointEquality::PointsEqual => { return point; },
            PointEquality::PointsNotEqual => {
                match self.window.contains(point.offset(offset)) {
                    Contains::DoesContain    => { return point.offset(offset); }
                    Contains::DoesNotContain => { return point; }
                }
            }
        }
    }
}