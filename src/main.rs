#![allow(dead_code)]
mod shape_enum;
mod shape_traits;

fn main() {
    shape_traits::shape_traits();
    shape_traits::shape_boxed();
    shape_enum::shape_enum();
}
