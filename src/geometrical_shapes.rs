use rand::Rng;
use raster::{Color, Image};

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line {
            start: Point {
                x: start.x,
                y: start.y,
            },
            end: Point { x: end.x, y: end.y },
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line::new(&Point::random(width, height), &Point::random(width, height))
    }
}

#[derive(Clone, Debug)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle {
            p1: Point::new(p1.x, p1.y),
            p2: Point::new(p2.x, p2.y),
            p3: Point::new(p3.x, p3.y),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

#[derive(Clone, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}



// impl Drawable for Line {
//     fn draw(&self, img: &mut Image) {
//         let start_x = self.p1.x;
//         let start_y = self.p1.y;
//         let end_x = self.p2.x;
//         let end_y = self.p2.y;

//         let dx = end_x - start_x;
//         let dy = end_y - start_y;

//         let steps = dx.abs().max(dy.abs());

//         let x_increment = dx as f64 / steps as f64;
//         let y_increment = dy as f64 / steps as f64;

//         for i in 0..steps as i32 {
//             //println!("{}",i);
//             // let t = i as f64 / steps;
//             let current_x = start_x as f64 + x_increment * i as f64;
//             let current_y = start_y as f64 + y_increment * i as f64;

//             println!("{}---{}", current_x, current_y);

//             let pixel_x = current_x.round() as i32;
//             let pixel_y = current_y.round() as i32;

//             if pixel_x >= 0 && pixel_x < img.width && pixel_y >= 0 && pixel_y < img.height {
//                 img.display(pixel_x as i32, pixel_y as i32, Point::color());
//             }
//         }
//     }
// }