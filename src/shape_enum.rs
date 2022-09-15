#![allow(dead_code)]
pub enum Shape {
    Square { side: f64 },
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

impl Shape {
    pub fn name(&self) -> String {
        match self {
            Shape::Square { side } => format!("Square side {side}"),
            Shape::Rectangle { width, height } => {
                format!("Rectangle width {width} height {height}")
            }
            Shape::Circle { radius } => format!("Circle radius {radius}"),
        }
    }

    pub fn area(&self) -> Option<f64> {
        match self {
            Shape::Square { side } => Some(side * side),
            Shape::Rectangle { width, height } => Some(width * height),
            Shape::Circle { radius } => Some(3.14 * radius * radius),
        }
    }
}

pub fn shape_enum() {
    println!();
    println!("**************Shape Enums**********");
    println!();
    let shapes: Vec<Shape> = vec![
        Shape::Square { side: 5.0 },
        Shape::Rectangle {
            width: 6.0,
            height: 8.0,
        },
        Shape::Circle { radius: 3.0 },
    ];

    for shape in &shapes {
        println!("{} area={}", shape.name(), shape.area().unwrap_or(0.0));
    }
}
