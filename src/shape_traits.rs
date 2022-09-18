pub trait Shape {
    fn shape(&self) -> &'static str;
    fn area(&self) -> f64;
}

pub struct Square {
    pub side: f64,
}

impl Shape for Square {
    fn shape(&self) -> &'static str {
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

impl Shape for Rectangle {
    fn shape(&self) -> &'static str {
        "rectangle"
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// consume: a polymorphic function
// that takes any thing that implements 'Shape'
fn consume<T: Shape>(shape: T) {
    println!(
        "Shape is a {}. Its area is {}.",
        shape.shape(),
        shape.area()
    );
}

pub fn shape_traits() {
    println!("\n************Shape Static(traits) Dispatch*********\n");
    // consume Square
    consume(Square { side: 5. });
    // consume Rectangle
    consume(Rectangle {
        width: 3.,
        height: 5.,
    });
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
    //     println!("{}: {}", shape.shape(), shape.area());
    // }

    // Issue: Vec needs to know the size of the item thats getting
    // pushed in.
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
        println!("{}: {}", shape.shape(), shape.area());
    }
}
