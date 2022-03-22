use std::fmt::{Debug, Formatter};
use line_drawing::Bresenham;
use crate::error::Error;
use crate::{Point2, SignedNum};

macro_rules! nth {
    ($x:expr, $axis:tt) => {
	    match $axis {
		    0 => $x.0,
		    1 => $x.1,
		    _ => unreachable!()
	    }
    }
}

pub struct BresenhamZip<T> {
	a: Bresenham<T>,
	b: Bresenham<T>,
	prev_a: Point2<T>,
	prev_b: Point2<T>,
	goal: T,
	axis: u8
}

impl<T: SignedNum> BresenhamZip<T> {

	#[inline]
	pub fn new(start: Point2<T>, end1: Point2<T>, end2: Point2<T>, axis: u8) -> Result<Self, Error<T>> {
		Ok(Self {
			a: Bresenham::new(start, end1),
			b: Bresenham::new(start, end2),
			prev_a: start,
			prev_b: start,
			goal: nth!(end1, axis),
			axis
		})
	}

}

impl<T: SignedNum> Iterator for BresenhamZip<T> {
	type Item = (Point2<T>, Point2<T>);

	#[allow(clippy::while_let_on_iterator)]  // needs to be like that to keep using the iterator
	fn next(&mut self) -> Option<Self::Item> {
		let axis = self.axis;

		let mut a = None;
		while let Some(point) = self.a.next() {
			if (nth!(point, axis) - nth!(self.prev_a, axis)).abs() > T::zero() {
				a = Some(self.prev_a);
				self.prev_a = point;
				break;
			}
			self.prev_a = point;
		}

		let mut b = None;
		while let Some(point) = self.b.next() {
			if (nth!(point, axis) - nth!(self.prev_b, axis)).abs() > T::zero() {
				b = Some(self.prev_b);
				self.prev_b = point;
				break;
			}
			self.prev_b = point;
		}

		if let Some(point) = a {
			Some((point, b.unwrap()))
		} else if nth!(self.prev_a, axis) == self.goal {
			self.goal -= T::one();
			Some((self.prev_a, self.prev_b))
		} else { None }
	}

}

impl<T: SignedNum> Debug for BresenhamZip<T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "BresenhamZip [
			({:?}, {:?}),
			({:?}, {:?})
		]",
		  self.prev_a.0, self.prev_a.1,
		  self.prev_b.0, self.prev_b.1,
		)
	}
}

#[cfg(test)]
mod tests {
	use super::BresenhamZip;

	macro_rules! symmetric {
	    ($a:tt, $b: tt, $c: tt, $axis: tt, $axis1: tt) => {
				let mut for_a = 50;
				let mut for_b = 50;
				let mut matching = 50;

				for (a, b) in BresenhamZip::new($a, $b, $c, $axis).unwrap() {
			    assert_eq!(for_a, a.$axis1);
			    assert_eq!(for_b, b.$axis1);
					assert_eq!(matching, a.$axis);
					assert_eq!(matching, b.$axis);

					for_a -= 1;
					for_b += 1;
					matching += 1;
				}
	    };
	}

	macro_rules! asymmetric {
	    ($a: tt, $b: tt, $c: tt, $axis: tt, $axis1: tt) => {
				let mut for_a = 50;
				let mut for_b = 50;
				let mut matching = 50;

				for (a, b) in BresenhamZip::new($a, $b, $c, $axis).unwrap() {
					assert!(a.$axis1 <= for_a);
					assert!(b.$axis1 >= for_b);
					assert_eq!(matching, a.$axis);
					assert_eq!(a.$axis, b.$axis);

					for_a = a.$axis1;
					for_b = b.$axis1;
					matching += 1;
				}
	    };
	}

	macro_rules! inverted {
	    ($a:tt, $b: tt, $c: tt, $axis: tt, $axis1: tt) => {
		    let mut for_a = 50;
				let mut for_b = 50;
				let mut matching = 50;

				for (a, b) in BresenhamZip::new($a, $b, $c, $axis).unwrap() {
					assert_eq!(for_a, a.$axis1);
					assert_eq!(for_b, b.$axis1);
					assert_eq!(matching, a.$axis);
					assert_eq!(a.$axis, b.$axis);

					for_a -= 1;
					for_b += 1;
					matching -= 1;
				}
	    };
	}

	mod x_axis {
		use super::BresenhamZip;

		/**
		#[test]
		fn invalid_x() {
			let result = Bresenham3dZip::new((0, 0, 0), (1, 1, 1), (2, 2, 2), 0);
			assert_eq!(result.unwrap_err(), Error::InvalidX(1, 2));
		}
		*/

		#[test]
		fn symmetric() {
			symmetric!((50, 50), (100, 0), (100, 100), 0, 1);
		}

		#[test]
		fn asymmetric() {
			asymmetric!((50, 50), (400, 0), (400, 800), 0, 1);
		}

		#[test]
		fn inverted() {
			inverted!((50, 50), (0, 0), (0, 100), 0, 1);
		}

	}

	mod y_axis {
		use super::BresenhamZip;

		#[test]
		fn symmetric() {
			symmetric!((50, 50), (0, 100), (100, 100), 1, 0);
		}

		#[test]
		fn asymmetric() {
			asymmetric!((50, 50), (0, 400), (800, 400), 1, 0);
		}

		#[test]
		fn inverted() {
			inverted!((50, 50), (0, 0), (100, 0), 1, 0);
		}

	}

}
