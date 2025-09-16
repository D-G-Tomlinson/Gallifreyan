use std::convert::From;
use std::boxed::Box;
use std::f64::consts::PI;
use std::f64::consts::TAU;

pub const SENTENCE_RADIUS:f64 = 100.0;

const FILL:&str="current";
const STROKE:&str="current";
pub trait Shape {
    fn shove(&mut self, diff:Cart);
    fn to_element(&self) -> String;
}
pub type Shapes = Vec<BShape>;
pub type BShape = Box<dyn Shape>;

use crate::shape::Thickness::*;
pub enum Thickness {
    Thin,
    Normal,
    Thick,
    ExtraThick,
}
impl Thickness {
    pub fn val(&self,mult:f64) -> f64 {
        match self {
            Thin => mult*0.01,
            Normal => mult*0.02,
            Thick => mult*0.04,
            ExtraThick => mult*0.08,
        }
    }
}

#[derive(Debug,Clone,Copy)]
pub struct Polar {
    pub radius:f64,
    pub theta:f64,//anticlockwise from x-axis, as usual
}
impl Polar {
    pub fn new(radius:f64, theta:f64) -> Self {
        let theta = theta.rem_euclid(TAU);
        Self { radius, theta }
    }
    pub fn rotate(&self, dt:f64) -> Self {
        Self::new(self.radius, self.theta + dt)
    }

    pub fn extend(&self, dr:f64) -> Self {
        Self::new(self.radius+dr, self.theta)
    }
    pub fn mult(&self, other:&Self) -> Self {
        let radius = self.radius * other.radius;
        let theta = self.theta + other.theta;
        Self::new(radius, theta)
    }
    pub fn divide(&self, other:&Self) -> Self {
        let radius = self.radius / other.radius;
        let theta = self.theta - other.theta;
        Self::new(radius, theta)
    }
}
#[derive(Debug,Clone,Copy)]
pub struct Cart {
    pub x:f64,
    pub y:f64,
}
impl Cart {
    pub fn new(x:f64, y:f64) -> Self {
        Self { x, y }
    }
    pub fn shove(&mut self,diff:Cart) {
        self.x+=diff.x;
        self.y+=diff.y;
    }
    pub fn distance(&self, &other:&Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance = dx*dx + dy*dy;
        return distance.sqrt();
    }
    pub fn origin() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn to(&self, other:&Self) -> Self {
        let x = other.x-self.x;
        let y = other.y-self.y;
        return Self {x,y}
    }
}
impl From<Polar> for Cart {
    fn from(polar: Polar) -> Self {
        let x = polar.radius * polar.theta.cos();
        let y = -polar.radius * polar.theta.sin();
        return Self { x, y }
    }
}

impl From<Cart> for Polar {
    fn from(cart: Cart) -> Self {
        let x = cart.x;
        let y = -cart.y;
        let radius = (x*x + y*y).sqrt();
        let theta:f64;
            theta = (y/x).atan();
        let new_theta:f64;
        if x < 0.0 {
            if theta <= 0.0 {
                new_theta = theta + PI;
            } else {
                new_theta = theta - PI;
            }
        } else {
            new_theta = theta;
        }
        return Polar::new(radius, new_theta);
    }
}

pub struct ShapeSet {
    shapes: Shapes,
    class: String,
}
const WORD_PULSE:bool = false;
const SENTENCE_PULSE:bool = false;

impl ShapeSet {
    pub fn new_rotating(shapes: Shapes, is_clockwise:bool) -> Self {
        let direction = if is_clockwise {"clockwise"} else {"anti_clockwise"};
        let class = format!("{direction}_number");
        Self { shapes,class }
    }
    pub fn new_rotating_class(shapes: Shapes, is_clockwise:bool,class:&str) -> Self {
        let direction = if is_clockwise {"clockwise"} else {"anti_clockwise"};
        let class = format!("{direction}_number {class}");
        Self { shapes,class }
    }
    pub fn new(shapes:Shapes, class:&str) -> Self {
        let mut class = class.to_string();
        if WORD_PULSE && class.contains("word") && !class.contains("pulsing") {
            class = format!("{class} pulsing");
        }
        if SENTENCE_PULSE && class.contains("sentence") && !class.contains("pulsing") {
            class = format!("{class} pulsing");
        }
        Self { shapes,class }
    }
}

impl Shape for ShapeSet {
    fn shove(&mut self, diff:Cart) {
        let _ = &self.shapes.iter_mut().for_each(|s| s.shove(diff));
    }
    fn to_element(&self) -> String {
        let els = &self.shapes.iter().map(|s| s.to_element()).collect::<Vec<_>>().join("\n");
        let class = &self.class;
        format!("<g class=\"{class}\">{els}</g>")
    }
}

pub struct Circle {
    centre: Cart,
    radius:f64,
    thickness:Option<f64>, // no thickness indicates fill
}

impl Circle {
    pub fn new(centre: Cart, radius:f64, thickness:Option<f64>) -> Self {
        Self { centre, radius, thickness }
    }
}

impl Shape for Circle {
    fn shove(&mut self, diff:Cart) {
        self.centre.shove(diff);
    }
    fn to_element(&self) -> String {
        let (opacity,width) = match &self.thickness {
            Some(t) => (0.0,*t),
            None => (1.0,0.0)
        };
        return format!("<circle  cx=\"{}\" cy=\"{}\" r=\"{}\" stroke-width=\"{}\" fill-opacity=\"{}\" />",
                       self.centre.x,
                       self.centre.y,
                       self.radius,
                       width,
                       opacity
        )
    }
}
pub struct Arc {
    start: Cart,
    end: Cart,
    radius:f64,
    large:bool,//do we take the long way round
    clockwise:bool,
    thickness:f64,
}

impl Arc {
    pub fn new(start:Cart,end:Cart, radius:f64, large:bool,clockwise:bool, thickness:f64) -> Self {
        Self {start, end, radius, large,clockwise, thickness}
    }
}

impl Shape for Arc {
    fn shove(&mut self, diff:Cart) {
        self.start.shove(diff);
        self.end.shove(diff);
    }
    fn to_element(&self) -> String {
        let width = self.thickness;
        let large = match self.large {
            true => 1,
            false => 0
        };
        let clockwise = match self.clockwise {
            true => 1,
            false => 0
        };
        return format!("<path fill-opacity=\"0\" stroke-width=\"{}\" d=\"M {} {} A {} {} 0 {} {} {} {}\" />",
            self.thickness,
            self.start.x,
            self.start.y,
            self.radius,
            self.radius,
            large,
            clockwise,
            self.end.x,
            self.end.y,
        )
    }
}
pub
struct Line {
    start: Cart,
    end: Cart,
    thickness:f64,
}
impl Line {
    pub fn new(start:Cart, end:Cart, thickness:f64) -> Self {
        Self {start, end, thickness}
    }
}
impl Shape for Line {
    fn shove(&mut self, diff:Cart) {
        self.start.shove(diff);
        self.end.shove(diff);
    }
    fn to_element(&self) -> String {
        return format!("<path stroke-width=\"{}\" d=\"M {} {} L {} {}\" />",
                       self.thickness,
                       self.start.x,
                       self.start.y,
                       self.end.x,
                       self.end.y,
        )
    }
}