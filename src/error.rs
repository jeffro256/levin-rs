#[derive(Debug)]
pub enum Error {
	IOError(std::io::ErrorKind),
	BincodeError(bincode::ErrorKind),
	EpeeStorageError(serde_epee::Error),
	ShortStream,
	ImmutableDestination,
	BadValue,
	MissingSignature,
	BadVersion
}

pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::IOError(err.kind())
	}
}

impl From<bincode::Error> for Error {
	fn from(err: bincode::Error) -> Self {
		Self::BincodeError(*err)
	}
}

/*
impl From<Box<bincode::Error>> for Error {
	fn from(err: Box<bincode::Error>) -> Self {
		Self::BincodeError(*err)
	}
}
*/

impl From<serde_epee::Error> for Error {
	fn from(err: serde_epee::Error) -> Self {
		Self::EpeeStorageError(err.clone())
	}
}