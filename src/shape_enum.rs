#![allow(dead_code)]
// Rust enum are SUM types, the variants can have data associated with it
pub enum Shape {
    Square { side: f64 },
    Rectangle { width: f64, height: f64 },
    Circle { radius: f64 },
}

impl Shape {
    pub fn name(&self) -> String {
        // Pattern matching
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
    println!("\n**************Shape Enums**********\n");
    let shapes: Vec<Shape> = vec![
        Shape::Square { side: 5.0 },
        Shape::Rectangle {
            width: 6.0,
            height: 8.0,
        },
        Shape::Circle { radius: 3.0 },
    ];

    for shape in &shapes {
        println!(
            "Size Shape: {} | Type: {} | area={}",
            std::mem::size_of::<Shape>(),
            shape.name(),
            shape.area().unwrap_or(0.0)
        );
    }
}
