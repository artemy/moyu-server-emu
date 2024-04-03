use bytes::{BufMut, Bytes, BytesMut};

pub struct OggPage {
    pub version: u8,
    pub flags: u8,
    pub granule: u64,
    pub serial: u32,
    pub sequence: u32,
    pub checksum: u32,
    pub segment_table: Vec<u8>,
    pub segments: Vec<Vec<u8>>,
}

pub trait Serializable {
    fn to_bytes(&self) -> Bytes;
}

impl Serializable for OggPage {
    fn to_bytes(&self) -> Bytes {
        let mut out = BytesMut::new();
        for char in "OggS".bytes() {
            out.put_u8(char);
        }
        out.put_u8(self.version);
        out.put_u8(self.flags);
        out.put_u64_le(self.granule);
        out.put_u32_le(self.serial);
        out.put_u32_le(self.sequence);
        out.put_u32_le(self.checksum);
        out.put_u8(self.segments.len() as u8);
        out.put(self.segment_table.as_slice());
        for segment in &self.segments {
            out.put(segment.as_slice())
        }
        out.into()
    }
}

pub struct OpusHead {
    pub version: u8,
    pub channel_count: u8,
    pub pre_skip: u16,
    pub input_sample_rate: u32,
    pub output_gain: u16,
    pub mapping_family: u8,
}
impl Serializable for OpusHead {
    fn to_bytes(&self) -> Bytes {
        let mut out = BytesMut::new();
        for char in "OpusHead".bytes() {
            out.put_u8(char);
        }
        out.put_u8(self.version);
        out.put_u8(self.channel_count);
        out.put_u16_le(self.pre_skip);
        out.put_u32_le(self.input_sample_rate);
        out.put_u16_le(self.output_gain);
        out.put_u8(self.mapping_family);
        out.into()
    }
}

pub struct OpusTags {
    pub vendor_string: Vec<u8>,
    pub user_comments: Vec<Vec<u8>>,
}

impl Serializable for OpusTags {
    fn to_bytes(&self) -> Bytes {
        let mut out = BytesMut::new();
        for char in "OpusTags".bytes() {
            out.put_u8(char);
        }
        out.put_u32_le(self.vendor_string.len() as u32);
        out.extend(&self.vendor_string);
        out.put_u32_le(self.user_comments.len() as u32);
        for user_comment in &self.user_comments {
            out.put_u32_le(user_comment.len() as u32);
            out.extend(user_comment);
        }
        out.into()
    }
}
