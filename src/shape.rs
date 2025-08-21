use std::convert::From;
use crate::tree::Word;

const FILL:&str="#000000";
const STROKE:&str="#000000";
trait Shape {
    fn shove(&mut self, dx:u32,dy:u32);
    fn to_element(&self) -> String;
}
type Shapes = Vec<Box<dyn Shape>>;

enum Thickness {
    Thin,
    Normal,
    Thick,
}
impl Thickness {
    fn val(&self) -> u32 {
        match self {
            Thickness::Thin => 1,
            Thickness::Normal => 2,
            Thickness::Thick => 4,
        }
    }
}

struct Polar {
    radius:f32,
    theta:f32,//anticlockwise from x-axis, as usual
}
struct Cart {
    x:u32,
    y:u32,
}
impl Cart {
    fn shove(&mut self,dx:u32,dy:u32) {
        self.x+=dx;
        self.y+=dy;
    }
}
impl From<Polar> for Cart {
    fn from(polar: Polar) -> Self {
        let x = polar.radius * polar.theta.cos();
        let y = polar.radius * polar.theta.sin();

        let x = x as u32;
        let y = y as u32;

        return Self {x, y,};
    }
}
struct Circle {
    centre: Cart,
    radius:u32,
    thickness:Option<Thickness>, // no thickness indicates fill
}
impl Shape for Circle {
    fn shove(&mut self, dx:u32,dy:u32) {
        self.centre.shove(dx,dy);
    }
    fn to_element(&self) -> String {
        let (opacity,width) = match &self.thickness {
            Some(t) => (0,t.val()),
            None => (1,0)
        };
        return format!("<circle fill=\"{}\" stroke = \"{}\" cx=\"{}\" cy\"{}\" r=\"{}\" stroke-width=\"{}\" fill-opacity=\"{}\" />",
                       FILL,
                       STROKE,
                       self.centre.x,
                       self.centre.y,
                       self.radius,
                       width,
                       opacity
        )
    }
}
struct Arc {
    start: Cart,
    end: Cart,
    radius:u32,
    large:bool,//do we take the long way round
    clockwise:bool,
    thickness:Thickness,
}

impl Shape for Arc {
    fn shove(&mut self, dx:u32,dy:u32) {
        self.start.shove(dx,dy);
        self.end.shove(dx,dy);
    }
    fn to_element(&self) -> String {
        let width = self.thickness.val();
        let large = match self.large {
            true => 1,
            false => 0
        };
        let clockwise = match self.clockwise {
            true => 1,
            false => 0
        };
        return format!("<path stroke=\"{}\" stroke-width=\"{}\" M {} {} A {} {} 0 {} {} {} {} />",
            STROKE,
            self.thickness.val(),
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

struct Line {
    start: Cart,
    end: Cart,
    thickness:Thickness,
}
impl Shape for Line {
    fn shove(&mut self, dx:u32,dy:u32) {
        self.start.shove(dx,dy);
        self.end.shove(dx,dy);
    }
    fn to_element(&self) -> String {
        let width = self.thickness.val();
        return format!("<path stroke=\"{}\" stroke-width=\"{}\" M {} {} L {} {} />",
                       STROKE,
                       self.thickness.val(),
                       self.start.x,
                       self.start.y,
                       self.end.x,
                       self.end.y,
        )
    }
}