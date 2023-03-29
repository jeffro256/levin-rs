use serde::{Serialize, de::DeserializeOwned};
use std::io::{Read, Write};

use crate::byte_counter::ByteCounter;
use crate::constants::LEVIN_HEADER_SIZE;
use crate::error::Result;
use crate::header::Header;

pub struct Bucket<'a, P>
{
	header: &'a mut Header,
	payload: P
}

pub type InputBucket<'a, 'b, P> = Bucket<'a, &'b mut P>;
pub type OutputBucket<'a, 'b, P> = Bucket<'a, &'b P>;

impl<'a, 'b, P> InputBucket<'a, 'b, P>
where
	P: DeserializeOwned
{
	pub fn input<R: Read>(&mut self, mut reader: R) -> Result<()>
	{
		*self.header = Header::from_reader(reader.by_ref())?;
		*self.payload = serde_epee::from_reader(reader)?;
		Ok(())
	}
}

impl<'a, 'b, P> OutputBucket<'a, 'b, P>
where
	P: Serialize
{
	pub fn output<W: Write>(&mut self, mut writer: W) -> Result<()>
	{
		// Do a psudeo-serialize to ByteCounter to be able to set the payload length in Header
		let mut pl_byte_counter = ByteCounter::default();
		serde_epee::to_writer(pl_byte_counter.by_ref(), &self.payload)?;
		self.header.set_payload_length(pl_byte_counter.count);

		self.header.to_writer(writer.by_ref())?;
		serde_epee::to_writer(writer, &self.payload)?;
		Ok(())
	}

	pub fn output_to_vec(&mut self) -> Result<Vec<u8>>
	{
		// Do a psudeo-serialize to ByteCounter to be able to set the payload length in Header
		let mut pl_byte_counter = ByteCounter::default();
		serde_epee::to_writer(pl_byte_counter.by_ref(), &self.payload)?;
		self.header.set_payload_length(pl_byte_counter.count);
		let total_bucket_size = LEVIN_HEADER_SIZE + pl_byte_counter.count;

		let mut res = Vec::<u8>::with_capacity(total_bucket_size);
		self.header.to_writer(res.by_ref())?;
		serde_epee::to_writer(res.by_ref(), &self.payload)?;
		Ok(res)
	}
}
