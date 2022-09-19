pub trait Shape {
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;
}

pub struct Square {
    pub side: f64,
}

impl Square {
    fn sizeof(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl Shape for Square {
    fn name(&self) -> &'static str {
        "square"
    }
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

impl Rectangle {
    fn sizeof(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl Shape for Rectangle {
    fn name(&self) -> &'static str {
        "rectangle"
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Circle {
    pub radius: f64,
}

impl Circle {
    fn sizeof(&self) -> usize {
        std::mem::size_of::<Self>()
    }
}

impl Shape for Circle {
    fn name(&self) -> &'static str {
        "Circle"
    }
    fn area(&self) -> f64 {
        self.radius * self.radius * 3.14
    }
}
// consume: a polymorphic function
// that takes any thing that implements 'Shape'
fn consume<T: Shape>(shape: T) {
    println!("Shape is a {}. Its area is {}.", shape.name(), shape.area());
}

pub fn shape_traits() {
    let square1 = Square { side: 5. };
    let rectangle1 = Rectangle {
        width: 5.,
        height: 5.,
    };
    let circle1 = Circle { radius: 5. };
    println!("Size Square: {}", square1.sizeof());
    println!("Size Rectangle: {}", rectangle1.sizeof());
    println!("Size Circle: {}", circle1.sizeof());
    println!("\n************Shape Static(traits) Dispatch*********\n");
    // consume Square
    consume(square1);
    // consume Rectangle
    consume(rectangle1);
    consume(circle1);
    // I can't have a collection of instances that impl Shape

    // let shapes: Vec<impl Shape> = vec![
    //     Square { side: 5. },
    //     Rectangle {
    //         width: 3.,
    //         height: 5.,
    //     },
    //     Rectangle {
    //         width: 3.,
    //         height: 5.,
    //     },
    // ];
    // for shape in &shapes {
    //     println!("{}: {}", shape.name(), shape.area());
    // }

    // Issue: Vec stored homogenous types
    // 1. Hide behind a reference (Heap/Stack)
    // 2. Create an Enum and store it in Vec.
}

pub fn shape_boxed() {
    println!("\n************Shape Dynamic(Box) Dispatch*********");
    // Shape Boxed(Dynamic dispatch)
    // Collection of instances that impl Shape
    // Vec<Box<dyn Shape>> - to disambiguate the behaviour associated with
    // boxed items

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Square { side: 5. }),
        Box::new(Rectangle {
            width: 3.,
            height: 5.,
        }),
        Box::new(Rectangle {
            width: 3.,
            height: 5.,
        }),
    ];

    for shape in &shapes {
        println!("{}: {}", shape.name(), shape.area());
    }
}
