use std::io::Write;

#[derive(Default)]
pub struct ByteCounter
{
	pub count: usize
}

impl Write for ByteCounter
{
	fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
    {
		self.count += buf.len();
        Ok(buf.len())
	}

	fn flush(&mut self) -> std::io::Result<()>
    {
		Ok(())
	}
}
