use bytes::{BufMut, Bytes};
use crc::{Crc, CRC_16_MODBUS};
use hound;
use hound::{SampleFormat, WavWriter};
use std::f32::consts::PI;
use std::io::Cursor;

const SAMPLE_RATE: u32 = 44100;
const BITS_PER_SAMPLE: u16 = 16;
const CHANNELS: u16 = 1;

const PULSE_LENGTH: u16 = 65;

const PULSE_SAMPLES: u32 = (SAMPLE_RATE / 1000) * PULSE_LENGTH as u32;

const MAX_AMPLITUDE: f32 = i16::MAX as f32;

pub fn credentials_to_wave(ssid: &String, password: &String) -> Bytes {
    log::debug!("Source credentials: [{}], [{}]", ssid, password);

    let encoded = encode_credentials(ssid, password);
    log::debug!("Encoded credentials: {:02X?}", encoded);

    let frequencies = bytes_to_frequencies(encoded);
    log::debug!("Frequencies: {:?}", frequencies);

    let bytes = generate_wav(frequencies);

    log::debug!("Len: {}", bytes.len());

    bytes
}

fn encode_credentials(ssid: &String, password: &String) -> Vec<u8> {
    let mut source: Vec<u8> = Vec::new();
    source.put(ssid.as_bytes());
    source.put_i8(-1);
    source.put(password.as_bytes());

    let mut encoded: Vec<u8> = Vec::new();

    let mut current: u8 = 0;
    for char in source {
        let enc = char.wrapping_add(current);
        encoded.put_u8(enc);
        current = enc;
    }

    let crc = Crc::<u16>::new(&CRC_16_MODBUS);
    let mut digest = crc.digest();
    digest.update(encoded.as_slice());
    encoded.put_u16(digest.finalize());

    encoded
}

fn bytes_to_frequencies(bytes: Vec<u8>) -> Vec<f32> {
    let start_frequency = 2375.0;
    let step = 187.5;
    let start_marker = vec![2000.0, 2000.0];
    let end_marker = vec![2187.5, 2187.5];

    let mut result = Vec::new();
    result.extend(start_marker);
    for b in bytes {
        let higher = start_frequency + step * (b >> 4 & 0xf) as f32;
        let lower = start_frequency + step * (b & 0xf) as f32;
        result.extend([higher, lower]);
    }
    result.extend(end_marker);
    result
}

fn generate_wav(frequencies: Vec<f32>) -> Bytes {
    let spec = hound::WavSpec {
        channels: CHANNELS,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: SampleFormat::Int,
    };

    let mut buf = Vec::new();
    let mut writer = WavWriter::new(Cursor::new(&mut buf), spec).unwrap();

    for &freq in frequencies.iter() {
        for n in 0..PULSE_SAMPLES {
            let t = n as f32 / SAMPLE_RATE as f32;
            let sample = (t * freq * 2.0 * PI).sin();

            let window_value =
                0.3 * (1.0 - ((2.0 * PI * n as f32) / (PULSE_SAMPLES as f32 - 1.0)).cos());
            let amplitude = sample * window_value;

            let amplitude = (amplitude * MAX_AMPLITUDE) as i16;
            writer.write_sample(amplitude).unwrap();
        }
    }

    writer.finalize().unwrap();

    Bytes::from(buf)
}

#[cfg(test)]
mod tests {
    use super::{bytes_to_frequencies, encode_credentials};

    #[test]
    fn test_byte_conversion() {
        let test_data: Vec<(String, String, Vec<u8>)> = vec![
            (
                "moyu-ap".to_string(),
                "moyu9876".to_string(),
                vec![
                    0x6D, 0xDC, 0x55, 0xCA, 0xF7, 0x58, 0xC8, 0xC7, 0x34, 0xA3, 0x1C, 0x91, 0xCA,
                    0x2, 0x39, 0x6F, 0xA, 0xF,
                ],
            ),
            (
                "moyu".to_string(),
                "".to_string(),
                vec![0x6D, 0xDC, 0x55, 0xCA, 0xC9, 0xBF, 0x34],
            ),
        ];

        for (ssid, password, expected) in test_data {
            let actual = encode_credentials(&ssid, &password);
            assert_eq!(expected, actual)
        }
    }

    #[test]
    fn test_frequency_conversion() {
        let expected = vec![
            2000.0, 2000.0, 3500.0, 4812.5, 4812.5, 4625.0, 3312.5, 3312.5, 4625.0, 4250.0, 5187.5,
            3687.5, 3312.5, 3875.0, 4625.0, 3875.0, 4625.0, 3687.5, 2937.5, 3125.0, 4250.0, 2937.5,
            2562.5, 4625.0, 4062.5, 2562.5, 4625.0, 4250.0, 2375.0, 2750.0, 2937.5, 4062.5, 3500.0,
            5187.5, 2375.0, 4250.0, 2375.0, 5187.5, 2187.5, 2187.5,
        ];
        let source = vec![
            0x6D, 0xDC, 0x55, 0xCA, 0xF7, 0x58, 0xC8, 0xC7, 0x34, 0xA3, 0x1C, 0x91, 0xCA, 0x2,
            0x39, 0x6F, 0xA, 0xF,
        ];

        let actual = bytes_to_frequencies(source);
        assert_eq!(expected, actual)
    }
}
