use std::io::{ Read, Write };

use serde::{Serialize, Deserialize};

use crate::constants::{self, LevinMessage, LevinFragment};
use crate::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Header {
	signature: u64,
	payload_length: u64,
	expect_reponse: bool,
	command: u32,
	return_code: u32,
	flags: u32,
	version: u32
}

impl Header {
	pub fn new_request(command: u32, payload_length: u64) -> Self {
		println!("PAYLOAD LENGTH: {}", payload_length);
		Self {
			signature: u64::from_le_bytes(constants::LEVIN_SIGNATURE),
			payload_length: payload_length,
			expect_reponse: true,
			command: command,
			return_code: 0,
			flags: Self::make_flags(constants::LevinMessage::Request, constants::LevinFragment::Unfragmented),
			version: constants::LEVIN_VERSION,
		}
	}

	pub fn new_response(command: u32, payload_length: u64, return_code: u32) -> Self {
		Self {
			signature: u64::from_le_bytes(constants::LEVIN_SIGNATURE),
			payload_length: payload_length,
			expect_reponse: false,
			command: command,
			return_code: return_code,
			flags: Self::make_flags(constants::LevinMessage::Response, constants::LevinFragment::Unfragmented),
			version: constants::LEVIN_VERSION,
		}
	}

	pub fn new_notification(command: u32, payload_length: u64) -> Self {
		Self {
			signature: u64::from_le_bytes(constants::LEVIN_SIGNATURE),
			payload_length: payload_length,
			expect_reponse: false,
			command: command,
			return_code: 0,
			flags: Self::make_flags(constants::LevinMessage::Notification, constants::LevinFragment::Unfragmented),
			version: constants::LEVIN_VERSION,
		}
	}

	fn make_flags(msgType: constants::LevinMessage, fragType: constants::LevinFragment) -> u32 {
		let typeMask = match msgType {
			LevinMessage::Request => constants::LEVIN_REQUEST_BIT,
			LevinMessage::Notification => constants::LEVIN_REQUEST_BIT,
			LevinMessage::Response => constants::LEVIN_RESPONSE_BIT,
			LevinMessage::Dummy => 0
		};

		let fragMask = match fragType {
			LevinFragment::Begin => constants::LEVIN_FRAG_BEGIN_BIT,
			LevinFragment::Middle => 0,
			LevinFragment::Unfragmented => 0,
			LevinFragment::End => constants::LEVIN_FRAG_END_BIT,
			LevinFragment::Dummy => constants::LEVIN_FRAG_BEGIN_BIT | constants::LEVIN_FRAG_END_BIT
		};

		return typeMask | fragMask;
	}
}
