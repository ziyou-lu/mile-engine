use std::fmt::{Debug, Formatter, Display};

pub struct Error {
	error_kind: i32,
	message: String
}

impl Error {
	pub fn engine<T>(message: String) -> Result<T, Self>{
		Err(
			Error {
				error_kind: 0,
				message
			}
		)
	}

	pub fn custom<T>(error_kind: i32, message: String) -> Result<T, Self> {
		Err(
			Error {
				error_kind,
				message
			}
		)
	}
}

impl Debug for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		todo!()
	}
}

impl std::error::Error for Error {

}