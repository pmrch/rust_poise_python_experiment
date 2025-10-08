use audiopus::{
    coder::Decoder,
    Channels, SampleRate
};
use bytes::Bytes;

pub fn decode_opus_packet(
    packet: &Bytes, 
    payload_offset: usize, 
    payload_end_pad: usize
) -> Result<Vec<i16>, Box<dyn std::error::Error + Send + Sync>> {
    let payload = &packet[payload_offset..packet.len() - payload_end_pad];

    // Create decoder (48 KHz, stereo)
    let mut decoder = Decoder::new(SampleRate::Hz48000, Channels::Stereo)?;
    let mut pcm = vec![0i16; 960 * 2];
    let decoded_len = decoder.decode(
        Some(payload), &mut pcm, false
    )?;

    pcm.truncate(decoded_len);
    Ok(pcm)
}