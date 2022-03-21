//! To use Bresenham.Zip you need to build a Bresenham object of the axis you want to travel.
//! You must specify the three points of the triangle. This rasterization algorithm only works using
//! triangles were two of its points have the same X, Y or Z.
//!
//! To build the object use the constructor passing first the point out of the horizontal line and
//! then the other two points. If the last two points doesn't belong to the same axis, an
//! error will be returned instead of the iterator.
//!
//! Once the iterator is ready you can use it like any other iterator and it will provide two points
//! each iteration, each one belonging to one of the two lines being calculated. Both of them will
//! have the same X or Y, defining an axis parallel line.
//!
//! # Example
//! ```
//! # use std::error::Error;
//! # use bresenham_zip::bresenham2::BresenhamZipY;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! for (left, right) in BresenhamZipY::new((50, 50), (0, 100), (250, 100))? {
//!   println!("{:?} - {:?}", left, right);
//!   assert_eq!(left.1, right.1);
//!   assert!((0..=50).contains(&left.0));
//!   assert!((50..=250).contains(&right.0));
//! }
//! #   Ok(())
//! # }
//!
//! ```
//!
//! ```
//! # use std::error::Error;
//! # use bresenham_zip::bresenham3::Bresenham3ZipX;
//! # fn main() -> Result<(), Box<dyn Error>> {
//! for (top, bottom) in Bresenham3ZipX::new((50, 50, 50), (100, 0, 10), (100, 250, 200))? {
//!   println!("{:?} - {:?}", top, bottom);
//!   assert_eq!(top.0, bottom.0);
//!   assert!((0..=50).contains(&top.1));
//!   assert!((10..=50).contains(&top.2));
//!   assert!((50..=250).contains(&bottom.1));
//!   assert!((50..=200).contains(&bottom.2));
//! }
//! #   Ok(())
//! # }
//! ```
//!
pub mod bresenham2;
pub mod bresenham3;
mod error;
pub mod zip_3d;

/// Trait to represent any valid number to use with the **BresenhamZip**.
/// Extension of [line_drawing::SignedNum] to allow the use of [std::fmt::Debug].
pub trait SignedNum: line_drawing::SignedNum + std::fmt::Debug {}
impl<T: line_drawing::SignedNum + std::fmt::Debug> SignedNum for T {}

/// A point in 2D space
pub type Point2<T> = (T, T);

/// A point in 3D space
pub type Point3<T> = (T, T, T);
