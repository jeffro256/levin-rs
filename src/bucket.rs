use crate::error::Result;
use crate::header;

pub struct Bucket<S: serde::Serialize>{
	header: header::Header,
	payload: S
}

impl<S> Bucket<S>
where
	S: serde::Serialize
{
	pub fn new_request(command: u32, payload: S) -> Result<Self> {
		match serde_epee::serialized_size(&payload) {
			Ok(payload_size) => Ok(Self {
				header: header::Header::new_request(command, payload_size as u64),
				payload: payload
			}),
			Err(err) => Err(err.into())
		}
		
	}
}

impl<S> Bucket<S> 
where
	S: serde::Serialize
{
	pub fn send<W>(&self, writer: &mut W) -> Result<()>
	where
		W: std::io::Write
	{
		bincode::serialize_into(writer.by_ref(), &self.header)?;
		serde_epee::to_writer(writer.by_ref(), &self.payload)?;
		writer.flush()?;
		Ok(())
	}
}
