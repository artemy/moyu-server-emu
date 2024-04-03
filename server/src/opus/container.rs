use crate::opus::models::{OggPage, OpusHead, OpusTags, Serializable};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crc::{Algorithm, Crc};
use std::ops::{Div, Rem};

const SERIAL: u32 = 0xDEADBEEF;
const CHUNK_SIZE: usize = 50;
const SAMPLE_SIZE: usize = 960;

pub struct MoyuOpusPacket {
    length: i32,
    final_range: u32,
    data: Vec<u8>,
}

pub fn create_container(input: &mut Bytes) -> Bytes {
    let packets = read_packets(input);
    log::info!("Packets found: {}", packets.len());

    let mut output = BytesMut::new();

    let first_page = prepare_first_page();
    output.put(first_page);

    let with_checksum = prepare_tags_page();
    output.put(with_checksum);

    let mut windowed = packets.chunks(CHUNK_SIZE).enumerate().peekable();

    while let Some((index, packets)) = windowed.next() {
        write_opus_page(index, packets, windowed.peek().is_none(), &mut output);
    }

    output.into()
}

pub fn read_packets(input_file: &mut Bytes) -> Vec<MoyuOpusPacket> {
    let mut packets: Vec<MoyuOpusPacket> = vec![];

    loop {
        if !input_file.has_remaining() {
            break;
        }
        let length = input_file.get_i32();

        let final_range = input_file.get_u32();

        let data = (0..length).map(|_| input_file.get_u8()).collect();

        let packet = MoyuOpusPacket {
            length,
            final_range,
            data,
        };
        log::debug!(
            "Decoded packet len: {}, final range: {:#04X}",
            packet.length,
            packet.final_range
        );
        packets.push(packet);
    }
    packets
}

pub fn prepare_first_page() -> Bytes {
    let head = OpusHead {
        version: 1,
        channel_count: 1, // mono
        pre_skip: 120,    // minimal value per spec
        input_sample_rate: 16000,
        output_gain: 0,
        mapping_family: 0,
    }
    .to_bytes();

    let first_page = OggPage {
        version: 0x00,
        flags: 0x02,
        granule: 0x0,
        serial: SERIAL,
        sequence: 0x0,
        checksum: 0x0000000,
        segment_table: vec![head.len() as u8],
        segments: vec![head.to_vec()],
    };

    let first_page_bytes = first_page.to_bytes();
    insert_checksum(first_page_bytes)
}

pub fn prepare_tags_page() -> Bytes {
    let tags = OpusTags {
        vendor_string: String::from("moyu server emulator 0.1").into_bytes(),
        user_comments: vec![],
    };

    let serialized = tags.to_bytes();

    let tags_page = OggPage {
        version: 0x00,
        flags: 0x00,
        granule: 0x0,
        serial: SERIAL,
        sequence: 0x1,
        checksum: 0x0000000,
        segment_table: vec![serialized.len() as u8],
        segments: vec![serialized.to_vec()],
    };

    let tags_page_bytes = tags_page.to_bytes();

    insert_checksum(tags_page_bytes)
}

fn write_opus_page(
    index: usize,
    packets: &[MoyuOpusPacket],
    last_page: bool,
    output_file: &mut BytesMut,
) {
    let packets_len = packets.len();
    let calculated_granule = {
        let previous_samples = index * CHUNK_SIZE * SAMPLE_SIZE;
        let current_samples = packets_len * SAMPLE_SIZE;
        previous_samples + current_samples
    } as u64;

    let current_page = OggPage {
        version: 0x00,
        flags: if last_page { 0x04 } else { 0x00 },
        granule: calculated_granule,
        serial: SERIAL,
        sequence: (index + 2) as u32,
        checksum: 0x0000000,
        segment_table: packets.iter().flat_map(encode_length).collect(),
        segments: packets.iter().map(|p| p.data.clone()).collect(),
    };

    let last_page_bytes = current_page.to_bytes();

    let with_checksum = insert_checksum(last_page_bytes);
    output_file.put(with_checksum);
}

fn encode_length(x: &MoyuOpusPacket) -> Vec<u8> {
    if x.length > 255 {
        let div = x.length.div(255);
        let rem = x.length.rem(255);
        let mut vec2: Vec<u8> = vec![255; div as usize];
        vec2.push(rem as u8);
        vec2
    } else {
        vec![x.length as u8]
    }
}

fn insert_checksum(first_page_bytes: Bytes) -> Bytes {
    let crc = Crc::<u32>::new(&CRC_32_OPUS);
    let mut digest = crc.digest();
    digest.update(first_page_bytes.as_ref());
    let i = digest.finalize();
    log::debug!("Checksum: 0x{:02X}", i);

    let before_checksum = &first_page_bytes[0..22];
    let after_checksum = &first_page_bytes[26..];
    let mut with_checksum = BytesMut::new();
    with_checksum.put(before_checksum);
    with_checksum.put_u32_le(i);
    with_checksum.put(after_checksum);
    with_checksum.into()
}

pub const CRC_32_OPUS: Algorithm<u32> = Algorithm {
    width: 32,
    poly: 0x04c11db7,
    init: 0x00000000,
    refin: false,
    refout: false,
    xorout: 0x00000000,
    check: 0x765e7680,
    residue: 0xc704dd7b,
};
