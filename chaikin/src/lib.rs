use macroquad::prelude::*;
use std::time::{Duration, Instant};

#[derive(Clone, Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { x, y }
    }

    pub fn distance_to(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

pub struct ChaikinState {
    pub control_points: Vec<Point>,
    pub current_points: Vec<Point>,
    pub animation_running: bool,
    pub current_step: usize,
    pub last_update: Instant,
    pub step_duration: Duration,
}

impl ChaikinState {
    pub fn new() -> Self {
        ChaikinState {
            control_points: Vec::new(),
            current_points: Vec::new(),
            animation_running: false,
            current_step: 0,
            last_update: Instant::now(),
            step_duration: Duration::from_millis(800),
        }
    }

    pub fn add_point(&mut self, x: f32, y: f32) {
        if !self.animation_running {
            self.control_points.push(Point::new(x, y));
        }
    }

    pub fn clear(&mut self) {
        self.control_points.clear();
        self.current_points.clear();
        self.animation_running = false;
        self.current_step = 0;
    }

    pub fn start_animation(&mut self) {
        if self.control_points.len() <= 2 {
            self.current_points = self.control_points.clone();
            self.animation_running = false;
            return;
        }

        self.current_points = self.control_points.clone();
        self.animation_running = true;
        self.current_step = 0;
        self.last_update = Instant::now();
    }

    fn chaikin_step(points: &[Point]) -> Vec<Point> {
        if points.len() < 2 {
            return points.to_vec();
        }

        let mut new_points = Vec::new();
        new_points.push(points[0].clone());

        for i in 0..points.len() - 1 {
            let p1 = &points[i];
            let p2 = &points[i + 1];

            let q1 = Point::new(p1.x * 0.75 + p2.x * 0.25, p1.y * 0.75 + p2.y * 0.25);
            let q2 = Point::new(p1.x * 0.25 + p2.x * 0.75, p1.y * 0.25 + p2.y * 0.75);

            new_points.push(q1);
            new_points.push(q2);
        }

        new_points.push(points[points.len() - 1].clone());
        new_points
    }

    pub fn update(&mut self) {
        if self.animation_running && self.last_update.elapsed() >= self.step_duration {
            self.current_points = Self::chaikin_step(&self.current_points);
            self.current_step += 1;
            self.last_update = Instant::now();

            if self.current_step >= 7 {
                self.animation_running = false;
            }
        }
        if !self.animation_running && self.current_step >= 7 {
            self.current_step = 0;
            self.current_points = self.control_points.clone();
            self.animation_running = true;
            self.last_update = Instant::now();
        }
    }
}

pub fn draw_points(points: &[Point], color: Color, radius: f32) {
    for point in points {
        draw_circle(point.x, point.y, radius, color);
    }
}

pub fn draw_lines(points: &[Point], color: Color, thickness: f32) {
    for pair in points.windows(2) {
        draw_line(pair[0].x, pair[0].y, pair[1].x, pair[1].y, thickness, color);
    }
}

pub fn draw_control_point_circles(points: &[Point]) {
    for point in points {
        draw_circle(point.x, point.y, 2.0, BLUE);
    }
}
