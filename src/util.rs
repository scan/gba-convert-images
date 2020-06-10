use std::convert::TryFrom;

pub fn consolidate_u8_u32(input: Vec<u8>) -> Vec<u32> {
    input
        .chunks(4)
        .map(|chunk| <[u8; 4]>::try_from(chunk).unwrap())
        .map(|chunk| u32::from_le_bytes(chunk))
        .collect()
}

pub fn consolidate_u8_u16(input: Vec<u8>) -> Vec<u16> {
    input
        .chunks(2)
        .map(|chunk| <[u8; 2]>::try_from(chunk).unwrap())
        .map(|chunk| u16::from_le_bytes(chunk))
        .collect()
}

pub fn consolidate_u16_u32(input: Vec<u16>) -> Vec<u32> {
    input
        .chunks(2)
        .map(|chunk| <[u16; 2]>::try_from(chunk).unwrap())
        .map(|chunk: [u16; 2]| (chunk[0] as u32) | ((chunk[1] as u32) << 16))
        .collect()
}

pub fn consolidate_u4_u32(input: Vec<u8>) -> Vec<u32> {
    input
        .chunks(2)
        .map(|chunk| (chunk[0] & 0x0f) | ((chunk[1] & 0x0f) << 4))
        .collect::<Vec<u8>>()
        .chunks(4)
        .map(|chunk| <[u8; 4]>::try_from(chunk).unwrap())
        .map(|chunk| u32::from_le_bytes(chunk))
        .collect()
}
