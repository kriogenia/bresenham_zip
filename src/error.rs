//! Library errors

use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter};
use crate::SignedNum;

/// Different errors that can happen using the library
#[derive(PartialEq)]
pub enum Error<'a, T> {
	/// The specified axis is not valid to build the desired BresenhamZip
	InvalidOrMissingAxis(&'a str),
	/// The ending points passed to build the BresenhamZip for X axis doesn't share the same X
	InvalidX(T,T),
	/// The ending points passed to build the BresenhamZip for Y axis doesn't share the same Y
	InvalidY(T,T),
	/// The ending points passed to build the BresenhamZip for Z axis doesn't share the same Z
	InvalidZ(T,T),
	/// Attempted building of `BresenhamZip` without the specification of a required point
	MissingPoint(&'a str),
}

impl<T: SignedNum> Error<'_, T> {

	fn message(&self) -> String {
		use Error::*;
		match self {
			InvalidOrMissingAxis(axis) => format!("Invalid axis. This BresenhamZip only accepts: {axis:?}"),
			InvalidX(left, right) => format!("Invalid X. Both values must have the same X ({left:?} != {right:?})"),
			InvalidY(left, right) => format!("Invalid Y. Both values must have the same Y ({left:?} != {right:?})"),
			InvalidZ(left, right) => format!("Invalid Z. Both values must have the same Z ({left:?} != {right:?})"),
			MissingPoint(point) => format!("Missing point. You must specify the {point:?}"),
		}
	}

}

impl<T: SignedNum> Debug for Error<'_, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message())
	}
}

impl<T: SignedNum> Display for Error<'_, T> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.message())
	}
}

impl<T: SignedNum> StdError for Error<'_, T> {}