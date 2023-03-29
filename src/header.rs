use std::io::{Read, Write};

use crate::constants::*;
use crate::error::Result;

#[derive(Debug, Default)]
pub struct Header {
    signature: u64,
    payload_length: u64,
    expect_reponse: bool,
    command: u32,
    return_code: u32,
    flags: u32,
    version: u32,
}

impl Header {
    pub fn new_request(command: u32) -> Self {
        Self::new_unfragmented(true, command, 0, LevinMessage::Request)
    }

    pub fn new_response(command: u32, return_code: u32) -> Self {
        Self::new_unfragmented(false, command, return_code, LevinMessage::Response)
    }

    pub fn new_notification(command: u32) -> Self {
        Self::new_unfragmented(false, command, 0, LevinMessage::Notification)
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<Self> {
        let mut res = Self::default();
        res.signature = read_fixed_u64(&mut reader)?;
        res.payload_length = read_fixed_u64(&mut reader)?;
        res.expect_reponse = read_bool(&mut reader)?;
        res.command = read_fixed_u32(&mut reader)?;
        res.return_code = read_fixed_u32(&mut reader)?;
        res.flags = read_fixed_u32(&mut reader)?;
        res.version = read_fixed_u32(&mut reader)?;
        Ok(res)
    }

    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<()> {
        write_fixed_u64(&mut writer, self.signature)?;
        write_fixed_u64(&mut writer, self.payload_length)?;
        write_bool(&mut writer, self.expect_reponse)?;
        write_fixed_u32(&mut writer, self.command)?;
        write_fixed_u32(&mut writer, self.return_code)?;
        write_fixed_u32(&mut writer, self.flags)?;
        write_fixed_u32(&mut writer, self.version)?;
        Ok(())
    }

    pub fn set_payload_length(&mut self, length: usize) {
        self.payload_length = length as u64;
    }

    fn new_unfragmented(
        expect_reponse: bool,
        command: u32,
        return_code: u32,
        msg_type: LevinMessage,
    ) -> Self {
        Self {
            signature: u64::from_le_bytes(LEVIN_SIGNATURE),
            payload_length: 0,
            expect_reponse,
            command,
            return_code,
            flags: make_flags(msg_type, LevinFragment::Unfragmented),
            version: LEVIN_VERSION,
        }
    }
}

fn make_flags(msg_type: LevinMessage, frag_type: LevinFragment) -> u32 {
    let msg_mask = match msg_type {
        LevinMessage::Request => LEVIN_REQUEST_BIT,
        LevinMessage::Notification => LEVIN_REQUEST_BIT,
        LevinMessage::Response => LEVIN_RESPONSE_BIT,
        LevinMessage::Dummy => 0,
    };

    let frag_mask = match frag_type {
        LevinFragment::Begin => LEVIN_FRAG_BEGIN_BIT,
        LevinFragment::Middle => 0,
        LevinFragment::Unfragmented => 0,
        LevinFragment::End => LEVIN_FRAG_END_BIT,
        LevinFragment::Dummy => LEVIN_FRAG_BEGIN_BIT | LEVIN_FRAG_END_BIT,
    };

    return msg_mask | frag_mask;
}

fn read_fixed_u32<R: Read>(r: &mut R) -> std::io::Result<u32> {
    let mut ibytes = [0u8; 4];
    r.read_exact(&mut ibytes)?;
    Ok(u32::from_le_bytes(ibytes))
}

fn read_fixed_u64<R: Read>(r: &mut R) -> std::io::Result<u64> {
    let mut ibytes = [0u8; 8];
    r.read_exact(&mut ibytes)?;
    Ok(u64::from_le_bytes(ibytes))
}

fn read_bool<R: Read>(r: &mut R) -> std::io::Result<bool> {
    let mut bbyte = 0u8;
    r.read_exact(std::slice::from_mut(&mut bbyte))?;
    Ok(bbyte != 0)
}

fn write_fixed_u32<W: Write>(w: &mut W, val: u32) -> std::io::Result<()> {
    let ibytes = val.to_le_bytes();
    w.write_all(&ibytes)
}

fn write_fixed_u64<W: Write>(w: &mut W, val: u64) -> std::io::Result<()> {
    let ibytes = val.to_le_bytes();
    w.write_all(&ibytes)
}

fn write_bool<W: Write>(w: &mut W, val: bool) -> std::io::Result<()> {
    let bbyte = if val { 1u8 } else { 0 };
    w.write(std::slice::from_ref(&bbyte))?;
    Ok(())
}
