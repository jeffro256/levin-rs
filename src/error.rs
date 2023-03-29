use core::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub enum ErrorKind
{
	IOError,
	EpeeStorageError,
	ShortStream,
	ImmutableDestination,
	BadValue,
	MissingSignature,
	BadVersion,
}

#[derive(Debug)]
pub struct Error
{
	kind: ErrorKind,
	message: String,
	source: Option<Box<dyn std::error::Error>>
}

impl Display for Error
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result
	{
		if self.message.len() != 0
		{
			write!(f, "levin_rs {:?}: {}", self.kind, self.message)
		}
		else
		{
			write!(f, "levin_rs {:?}", self.kind)
		}
    }
}

impl std::error::Error for Error
{
	fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
	{
		match &self.source {
			Some(s) => Some(s.as_ref()),
			None => None
		}
	}
}

pub type Result<T> = std::result::Result<T, Error>;

macro_rules! impl_from_error
{
    ($error_name:path, $ekind:expr) =>
	{
        impl From<$error_name> for Error
		{
			fn from(err: $error_name) -> Self
			{
				Self { kind: $ekind, message: String::new(), source: Some(Box::new(err)) }
			}
		}
    };
}

impl_from_error!(std::io::Error, ErrorKind::IOError);
impl_from_error!(serde_epee::Error, ErrorKind::IOError);
