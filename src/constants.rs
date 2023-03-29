pub const LEVIN_SIGNATURE: [u8; 8] = [0x01, 0x21, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01];
pub const LEVIN_VERSION: u32 = 1;

pub const LEVIN_HEADER_SIZE: usize = 33;
pub const LEVIN_MAX_PAYLOAD_SIZE: u64 = 100000000; // 100MB

pub const LEVIN_REQUEST_BIT: u32 = 1 << 0; // Q
pub const LEVIN_RESPONSE_BIT: u32 = 1 << 1; // S
pub const LEVIN_FRAG_BEGIN_BIT: u32 = 1 << 2; // B
pub const LEVIN_FRAG_END_BIT: u32 = 1 << 3; // E

pub const LEVIN_COMMAND_HANDSHAKE: u32 = 1001;
pub const LEVIN_COMMAND_TIMED_SYNC: u32 = 1002;
pub const LEVIN_COMMAND_PING: u32 = 1003;
pub const LEVIN_COMMAND_STAT_INFO: u32 = 1004;
pub const LEVIN_COMMAND_NETWORK_STATE: u32 = 1005;
pub const LEVIN_COMMAND_PEER_ID: u32 = 1006;
pub const LEVIN_COMMAND_SUPPORT_FLAGS: u32 = 1007;

pub const CRYPTONOTE_COMMAND_NEW_BLOCK: u32 = 2001;
pub const CRYPTONOTE_COMMAND_NEW_TX: u32 = 2002;
pub const CRYPTONOTE_COMMAND_REQ_GET_OBJS: u32 = 2003;
pub const CRYPTONOTE_COMMAND_RESP_GET_OBJS: u32 = 2004;
// No 2005
pub const CRYPTONOTE_COMMAND_REQ_CHAIN: u32 = 2006;
pub const CRYPTONOTE_COMMAND_RESP_CHAIN: u32 = 2007;
pub const CRYPTONOTE_COMMAND_NEW_FLUFFY_BLOCK: u32 = 2008;
pub const CRYPTONOTE_COMMAND_REQ_FLUFFY_MISSING_TX: u32 = 2009;

pub enum LevinMessage {
    Request,
    Response,
    Notification,
    Dummy,
}

pub enum LevinFragment {
    Begin,
    Middle,
    End,
    Unfragmented,
    Dummy,
}

// Converts hex string to byte array at compile time
pub const fn from_hex<const N: usize>(hex_str: &str) -> [u8; N] {
    let hex_bytes = hex_str.as_bytes();

    if hex_bytes.len() != N * 2 {
        panic!("input hex string size is incorrect");
    }

    let mut res = [0u8; N];

    let mut i = 0;
    while i < N {
        let left_byte = hex_bytes[2 * i];
        let right_byte = hex_bytes[2 * i + 1];

        let left_val = match left_byte {
            d if ('0' as u8 <= d && d <= '9' as u8) => d - '0' as u8,
            u if ('A' as u8 <= u && u <= 'F' as u8) => u - 'A' as u8 + 10u8,
            l if ('a' as u8 <= l && l <= 'f' as u8) => l - 'a' as u8 + 10u8,
            _ => panic!("found invalid hex character"),
        };

        let right_val = match right_byte {
            d if ('0' as u8 <= d && d <= '9' as u8) => d - '0' as u8,
            u if ('A' as u8 <= u && u <= 'F' as u8) => u - 'A' as u8 + 10u8,
            l if ('a' as u8 <= l && l <= 'f' as u8) => l - 'a' as u8 + 10u8,
            _ => panic!("found invalid hex character"),
        };

        res[i] = left_val << 4 | right_val;

        i += 1;
    }

    res
}

pub const CRYPTONOTE_STAGENET_NETWORK_ID: [u8; 16] = from_hex("1230F171610441611731008216A1A112");

pub const MONERO_GENESIS_HASH: [u8; 32] =
    from_hex("418015bb9ae982a1975da7d79277c2705727a56894ba0fb246adaabb1f4632e3");
