//! Base57 affine codec experiment.
//!
//! This module separates ordinary reversible radix conversion from the more
//! experimental affine identifier notation. The baseline codec preserves
//! arbitrary bytes. The affine notation stores payload bytes inside fixed-width
//! base-57 membrane chunks and searches a small nonce space for residue or
//! prime-witness validity.

use crate::validation::{
    bounded_k::DEFAULT_PREFILTER_PRIMES,
    fast_affine::{build_fast_affine_lane, FastAffineLane, FastLaneConfig},
};
use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::{ToPrimitive, Zero};
use serde::Serialize;
use thiserror::Error;

pub const BITCOIN_BASE58_ALPHABET: &str =
    "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
pub const BASE57_ALPHABET: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxy";
pub const BASE57_DROPPED_CHAR: char = 'z';

pub const AFFINE_BASE: u32 = 57;
pub const AFFINE_OUTER: u32 = 1;
pub const AFFINE_INNER: u32 = 56;
pub const AFFINE_MIDDLE_LENGTH: usize = 6;
pub const AFFINE_K: (u32, u32) = (0, 0);
pub const AFFINE_PAYLOAD_BYTES_PER_CHUNK: usize = 3;
pub const AFFINE_NONCE_BITS: u32 = 10;
pub const AFFINE_NONCE_SPACE: u16 = 1 << AFFINE_NONCE_BITS;
pub const AFFINE_CHUNK_DIGITS: usize = 2 + AFFINE_MIDDLE_LENGTH + 2;
pub const AFFINE_RESIDUE_PREFIX: &str = "a57r1";
pub const AFFINE_PRIME_PREFIX: &str = "a57p1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CodecAlphabet {
    Base58,
    Base57,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CanonicalInputFormat {
    Hex,
    Text,
    Base58,
    Base57,
    Affine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum AffineCodecMode {
    Residue,
    Prime,
}

#[derive(Debug, Clone, Error)]
pub enum Base57AffineCodecError {
    #[error("alphabet has duplicate character '{0}'")]
    DuplicateAlphabetCharacter(char),
    #[error("alphabet must contain at least two characters")]
    AlphabetTooShort,
    #[error("character '{ch}' is not valid for {alphabet}")]
    InvalidCharacter { alphabet: &'static str, ch: char },
    #[error("hex input has odd length")]
    OddHexLength,
    #[error("invalid hex byte '{0}'")]
    InvalidHexByte(String),
    #[error("text output is not valid UTF-8")]
    InvalidUtf8,
    #[error("invalid affine frame; expected <prefix>:<payload_len>:<body>")]
    InvalidAffineFrame,
    #[error("unsupported affine prefix '{0}'")]
    UnsupportedAffinePrefix(String),
    #[error("invalid payload length '{0}'")]
    InvalidPayloadLength(String),
    #[error("affine body length must be a multiple of {AFFINE_CHUNK_DIGITS}")]
    InvalidAffineBodyLength,
    #[error(
        "affine chunk count {actual} does not match expected count {expected} for payload length"
    )]
    InvalidAffineChunkCount { expected: usize, actual: usize },
    #[error("affine chunk {chunk_index} has invalid boundary digits")]
    InvalidAffineChunkShape { chunk_index: usize },
    #[error("affine chunk {chunk_index} has seed outside the payload/nonce subspace")]
    InvalidAffineSeed { chunk_index: usize },
    #[error("affine chunk {chunk_index} failed residue validation")]
    ResidueValidationFailed { chunk_index: usize },
    #[error("affine chunk {chunk_index} failed prime validation")]
    PrimeValidationFailed { chunk_index: usize },
    #[error("payload length exceeds decoded affine bytes")]
    PayloadLengthExceedsDecodedBytes,
    #[error("decoded affine padding contains nonzero bytes")]
    NonZeroAffinePadding,
    #[error("no valid nonce found for chunk {chunk_index} in {mode:?} mode")]
    NonceExhausted {
        chunk_index: usize,
        mode: AffineCodecMode,
    },
    #[error("fast affine lane setup failed: {0}")]
    FastLane(#[from] crate::validation::fast_affine::FastPrimeError),
}

#[derive(Debug, Clone, Serialize)]
pub struct BaseCodecRoundTrip {
    pub alphabet: CodecAlphabet,
    pub encoded: String,
    pub encoded_len: usize,
    pub decoded_len: usize,
    pub roundtrip_ok: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineEncoded {
    pub mode: AffineCodecMode,
    pub notation: String,
    pub prefix: String,
    pub payload_len: usize,
    pub body: String,
    pub body_len: usize,
    pub chunk_count: usize,
    pub chunks: Vec<AffineChunkRecord>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineDecoded {
    pub mode: AffineCodecMode,
    pub payload_len: usize,
    pub payload: Vec<u8>,
    pub chunk_count: usize,
    pub chunks: Vec<AffineChunkRecord>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineChunkRecord {
    pub chunk_index: usize,
    pub mode: AffineCodecMode,
    pub payload_hex: String,
    pub payload_value: u32,
    pub nonce: u16,
    pub attempts: u16,
    pub seed: u64,
    pub middle_digits: String,
    pub chunk_text: String,
    pub candidate_value: u64,
    pub residue_admissible: bool,
    pub prime: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BaseRendering {
    pub label: String,
    pub text: String,
    pub char_len: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AffineEnvelopeSummary {
    pub mode: AffineCodecMode,
    pub notation: String,
    pub char_len: usize,
    pub chunk_count: usize,
    pub total_attempts: u64,
    pub average_attempts_per_chunk: f64,
    pub all_chunks_residue_admissible: bool,
    pub all_chunks_prime: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct BaseInvariantMap {
    pub payload_len: usize,
    pub leading_zero_bytes: usize,
    pub hex: String,
    pub decimal_value: String,
    pub renderings: Vec<BaseRendering>,
    pub affine_residue: Option<AffineEnvelopeSummary>,
    pub affine_prime: Option<AffineEnvelopeSummary>,
}

pub fn encode_bytes(
    bytes: &[u8],
    alphabet: CodecAlphabet,
) -> Result<String, Base57AffineCodecError> {
    let alphabet = alphabet_chars(alphabet);
    validate_alphabet(&alphabet)?;
    if bytes.is_empty() {
        return Ok(String::new());
    }

    let leading_zeroes = bytes.iter().take_while(|&&byte| byte == 0).count();
    let mut value = BigUint::from_bytes_be(bytes);
    let radix = BigUint::from(alphabet.len());
    let mut encoded = Vec::new();

    while !value.is_zero() {
        let (quotient, remainder) = value.div_rem(&radix);
        let index = remainder
            .to_usize()
            .expect("remainder should fit usize for small alphabets");
        encoded.push(alphabet[index]);
        value = quotient;
    }

    encoded.extend(std::iter::repeat_n(alphabet[0], leading_zeroes));
    encoded.reverse();
    Ok(encoded.into_iter().collect())
}

pub fn decode_to_bytes(
    encoded: &str,
    alphabet: CodecAlphabet,
) -> Result<Vec<u8>, Base57AffineCodecError> {
    let alphabet_chars = alphabet_chars(alphabet);
    validate_alphabet(&alphabet_chars)?;
    if encoded.is_empty() {
        return Ok(Vec::new());
    }

    let mut value = BigUint::zero();
    let radix = BigUint::from(alphabet_chars.len());
    for ch in encoded.chars() {
        let digit = alphabet_chars
            .iter()
            .position(|&candidate| candidate == ch)
            .ok_or(Base57AffineCodecError::InvalidCharacter {
                alphabet: alphabet_name(alphabet),
                ch,
            })?;
        value = value * &radix + BigUint::from(digit);
    }

    let leading_zeroes = encoded
        .chars()
        .take_while(|&ch| ch == alphabet_chars[0])
        .count();
    let mut bytes = vec![0; leading_zeroes];
    if !value.is_zero() {
        bytes.extend(value.to_bytes_be());
    }
    Ok(bytes)
}

pub fn base_codec_round_trip(
    bytes: &[u8],
    alphabet: CodecAlphabet,
) -> Result<BaseCodecRoundTrip, Base57AffineCodecError> {
    let encoded = encode_bytes(bytes, alphabet)?;
    let decoded = decode_to_bytes(&encoded, alphabet)?;
    Ok(BaseCodecRoundTrip {
        alphabet,
        encoded_len: encoded.len(),
        decoded_len: decoded.len(),
        roundtrip_ok: decoded == bytes,
        encoded,
    })
}

pub fn decode_canonical_payload(
    input: &str,
    input_format: CanonicalInputFormat,
) -> Result<Vec<u8>, Base57AffineCodecError> {
    Ok(match input_format {
        CanonicalInputFormat::Hex => parse_hex(input)?,
        CanonicalInputFormat::Text => input.as_bytes().to_vec(),
        CanonicalInputFormat::Base58 => decode_to_bytes(input, CodecAlphabet::Base58)?,
        CanonicalInputFormat::Base57 => decode_to_bytes(input, CodecAlphabet::Base57)?,
        CanonicalInputFormat::Affine => decode_affine(input)?.payload,
    })
}

pub fn build_base_invariant_map(
    payload: &[u8],
    include_affine_residue: bool,
    include_affine_prime: bool,
) -> Result<BaseInvariantMap, Base57AffineCodecError> {
    let base58 = encode_bytes(payload, CodecAlphabet::Base58)?;
    let base57 = encode_bytes(payload, CodecAlphabet::Base57)?;
    let affine_residue = if include_affine_residue {
        Some(affine_envelope_summary(encode_affine(
            payload,
            AffineCodecMode::Residue,
        )?))
    } else {
        None
    };
    let affine_prime = if include_affine_prime {
        Some(affine_envelope_summary(encode_affine(
            payload,
            AffineCodecMode::Prime,
        )?))
    } else {
        None
    };

    Ok(BaseInvariantMap {
        payload_len: payload.len(),
        leading_zero_bytes: payload.iter().take_while(|&&byte| byte == 0).count(),
        hex: format_hex(payload),
        decimal_value: BigUint::from_bytes_be(payload).to_str_radix(10),
        renderings: vec![
            BaseRendering {
                label: "base16_hex".to_string(),
                text: format_hex(payload),
                char_len: payload.len() * 2,
            },
            BaseRendering {
                label: "base58".to_string(),
                char_len: base58.len(),
                text: base58,
            },
            BaseRendering {
                label: "base57".to_string(),
                char_len: base57.len(),
                text: base57,
            },
        ],
        affine_residue,
        affine_prime,
    })
}

pub fn encode_affine(
    payload: &[u8],
    mode: AffineCodecMode,
) -> Result<AffineEncoded, Base57AffineCodecError> {
    let lane = affine_lane()?;
    let chunks = payload
        .chunks(AFFINE_PAYLOAD_BYTES_PER_CHUNK)
        .enumerate()
        .map(|(chunk_index, payload_chunk)| {
            encode_affine_chunk(&lane, chunk_index, payload_chunk, mode)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let body = chunks
        .iter()
        .map(|chunk| chunk.chunk_text.as_str())
        .collect::<String>();
    let prefix = affine_prefix(mode).to_string();
    let notation = format!("{prefix}:{}:{body}", payload.len());

    Ok(AffineEncoded {
        mode,
        notation,
        prefix,
        payload_len: payload.len(),
        body_len: body.len(),
        chunk_count: chunks.len(),
        body,
        chunks,
    })
}

pub fn decode_affine(notation: &str) -> Result<AffineDecoded, Base57AffineCodecError> {
    let (mode, payload_len, body) = parse_affine_frame(notation)?;
    if body.len() % AFFINE_CHUNK_DIGITS != 0 {
        return Err(Base57AffineCodecError::InvalidAffineBodyLength);
    }
    let actual_chunk_count = body.len() / AFFINE_CHUNK_DIGITS;
    let expected_chunk_count = payload_len.div_ceil(AFFINE_PAYLOAD_BYTES_PER_CHUNK);
    if actual_chunk_count != expected_chunk_count {
        return Err(Base57AffineCodecError::InvalidAffineChunkCount {
            expected: expected_chunk_count,
            actual: actual_chunk_count,
        });
    }
    let lane = affine_lane()?;
    let mut decoded = Vec::new();
    let mut chunks = Vec::new();

    for (chunk_index, start) in (0..body.len()).step_by(AFFINE_CHUNK_DIGITS).enumerate() {
        let chunk_text = &body[start..start + AFFINE_CHUNK_DIGITS];
        let record = decode_affine_chunk(&lane, chunk_index, chunk_text, mode)?;
        decoded.extend(record.payload_value.to_be_bytes()[1..].iter().copied());
        chunks.push(record);
    }

    if payload_len > decoded.len() {
        return Err(Base57AffineCodecError::PayloadLengthExceedsDecodedBytes);
    }
    if decoded[payload_len..].iter().any(|&byte| byte != 0) {
        return Err(Base57AffineCodecError::NonZeroAffinePadding);
    }
    decoded.truncate(payload_len);

    Ok(AffineDecoded {
        mode,
        payload_len,
        payload: decoded,
        chunk_count: chunks.len(),
        chunks,
    })
}

fn affine_envelope_summary(encoded: AffineEncoded) -> AffineEnvelopeSummary {
    let total_attempts = encoded
        .chunks
        .iter()
        .map(|chunk| u64::from(chunk.attempts))
        .sum::<u64>();
    let chunk_count = encoded.chunks.len();
    AffineEnvelopeSummary {
        mode: encoded.mode,
        char_len: encoded.notation.len(),
        chunk_count,
        total_attempts,
        average_attempts_per_chunk: if chunk_count == 0 {
            0.0
        } else {
            total_attempts as f64 / chunk_count as f64
        },
        all_chunks_residue_admissible: encoded.chunks.iter().all(|chunk| chunk.residue_admissible),
        all_chunks_prime: encoded.chunks.iter().all(|chunk| chunk.prime),
        notation: encoded.notation,
    }
}

pub fn parse_hex(input: &str) -> Result<Vec<u8>, Base57AffineCodecError> {
    let input = input
        .strip_prefix("0x")
        .or_else(|| input.strip_prefix("0X"))
        .unwrap_or(input);
    if !input.len().is_multiple_of(2) {
        return Err(Base57AffineCodecError::OddHexLength);
    }
    let mut bytes = Vec::with_capacity(input.len() / 2);
    for idx in (0..input.len()).step_by(2) {
        let part = &input[idx..idx + 2];
        let value = u8::from_str_radix(part, 16)
            .map_err(|_| Base57AffineCodecError::InvalidHexByte(part.to_string()))?;
        bytes.push(value);
    }
    Ok(bytes)
}

pub fn format_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

pub fn bytes_to_text(bytes: &[u8]) -> Result<String, Base57AffineCodecError> {
    String::from_utf8(bytes.to_vec()).map_err(|_| Base57AffineCodecError::InvalidUtf8)
}

pub fn affine_lane() -> Result<FastAffineLane, Base57AffineCodecError> {
    Ok(build_fast_affine_lane(FastLaneConfig::new(
        AFFINE_BASE,
        AFFINE_OUTER,
        AFFINE_INNER,
        AFFINE_MIDDLE_LENGTH,
        AFFINE_K,
    ))?)
}

pub fn residue_moduli_for_affine_base() -> Vec<u32> {
    DEFAULT_PREFILTER_PRIMES
        .iter()
        .copied()
        .filter(|&modulus| gcd_u32(AFFINE_BASE, modulus) == 1)
        .collect()
}

pub fn is_affine_seed_residue_admissible(lane: &FastAffineLane, seed: u64) -> bool {
    residue_moduli_for_affine_base()
        .iter()
        .copied()
        .all(|modulus| {
            !((lane.shift % modulus as u64)
                + (lane.gradient % modulus as u64) * (seed % modulus as u64))
                .is_multiple_of(modulus as u64)
        })
}

fn encode_affine_chunk(
    lane: &FastAffineLane,
    chunk_index: usize,
    payload_chunk: &[u8],
    mode: AffineCodecMode,
) -> Result<AffineChunkRecord, Base57AffineCodecError> {
    let payload_value = payload_chunk_value(payload_chunk);
    for nonce in 0..AFFINE_NONCE_SPACE {
        let seed = affine_seed(payload_value, nonce);
        let candidate_value = lane
            .candidate_value(seed)
            .expect("affine seed should fit lane seed capacity");
        let residue_admissible = is_affine_seed_residue_admissible(lane, seed);
        let prime = primal::is_prime(candidate_value);
        let accepted = match mode {
            AffineCodecMode::Residue => residue_admissible,
            AffineCodecMode::Prime => prime,
        };
        if accepted {
            return Ok(AffineChunkRecord {
                chunk_index,
                mode,
                payload_hex: payload_chunk_hex(payload_chunk),
                payload_value,
                nonce,
                attempts: nonce + 1,
                seed,
                middle_digits: lane.middle_digits(seed),
                chunk_text: affine_chunk_text(lane, seed),
                candidate_value,
                residue_admissible,
                prime,
            });
        }
    }

    Err(Base57AffineCodecError::NonceExhausted { chunk_index, mode })
}

fn decode_affine_chunk(
    lane: &FastAffineLane,
    chunk_index: usize,
    chunk_text: &str,
    mode: AffineCodecMode,
) -> Result<AffineChunkRecord, Base57AffineCodecError> {
    let digits = chunk_text
        .chars()
        .map(base57_char_to_digit)
        .collect::<Result<Vec<_>, _>>()?;
    if digits.len() != AFFINE_CHUNK_DIGITS
        || digits[0] != AFFINE_OUTER
        || digits[1] != AFFINE_INNER
        || digits[AFFINE_CHUNK_DIGITS - 2] != AFFINE_INNER
        || digits[AFFINE_CHUNK_DIGITS - 1] != AFFINE_OUTER
    {
        return Err(Base57AffineCodecError::InvalidAffineChunkShape { chunk_index });
    }

    let seed = digits_to_u64(&digits[2..2 + AFFINE_MIDDLE_LENGTH], AFFINE_BASE);
    let payload_value_u64 = seed >> AFFINE_NONCE_BITS;
    if payload_value_u64 > 0x00ff_ffff {
        return Err(Base57AffineCodecError::InvalidAffineSeed { chunk_index });
    }
    let payload_value = payload_value_u64 as u32;
    let nonce = (seed & u64::from(AFFINE_NONCE_SPACE - 1)) as u16;
    let candidate_value = lane
        .candidate_value(seed)
        .ok_or(Base57AffineCodecError::InvalidAffineSeed { chunk_index })?;
    let residue_admissible = is_affine_seed_residue_admissible(lane, seed);
    if !residue_admissible {
        return Err(Base57AffineCodecError::ResidueValidationFailed { chunk_index });
    }
    let prime = primal::is_prime(candidate_value);
    if mode == AffineCodecMode::Prime && !prime {
        return Err(Base57AffineCodecError::PrimeValidationFailed { chunk_index });
    }

    Ok(AffineChunkRecord {
        chunk_index,
        mode,
        payload_hex: payload_value_hex(payload_value),
        payload_value,
        nonce,
        attempts: nonce + 1,
        seed,
        middle_digits: lane.middle_digits(seed),
        chunk_text: chunk_text.to_string(),
        candidate_value,
        residue_admissible,
        prime,
    })
}

fn parse_affine_frame(
    notation: &str,
) -> Result<(AffineCodecMode, usize, &str), Base57AffineCodecError> {
    let mut parts = notation.splitn(3, ':');
    let prefix = parts
        .next()
        .ok_or(Base57AffineCodecError::InvalidAffineFrame)?;
    let payload_len = parts
        .next()
        .ok_or(Base57AffineCodecError::InvalidAffineFrame)?;
    let body = parts
        .next()
        .ok_or(Base57AffineCodecError::InvalidAffineFrame)?;
    let mode = match prefix {
        AFFINE_RESIDUE_PREFIX => AffineCodecMode::Residue,
        AFFINE_PRIME_PREFIX => AffineCodecMode::Prime,
        _ => {
            return Err(Base57AffineCodecError::UnsupportedAffinePrefix(
                prefix.to_string(),
            ))
        }
    };
    let payload_len = payload_len
        .parse::<usize>()
        .map_err(|_| Base57AffineCodecError::InvalidPayloadLength(payload_len.to_string()))?;
    Ok((mode, payload_len, body))
}

fn affine_prefix(mode: AffineCodecMode) -> &'static str {
    match mode {
        AffineCodecMode::Residue => AFFINE_RESIDUE_PREFIX,
        AffineCodecMode::Prime => AFFINE_PRIME_PREFIX,
    }
}

fn affine_seed(payload_value: u32, nonce: u16) -> u64 {
    (u64::from(payload_value) << AFFINE_NONCE_BITS) | u64::from(nonce)
}

fn payload_chunk_value(payload_chunk: &[u8]) -> u32 {
    let mut padded = [0u8; AFFINE_PAYLOAD_BYTES_PER_CHUNK];
    padded[..payload_chunk.len()].copy_from_slice(payload_chunk);
    u32::from_be_bytes([0, padded[0], padded[1], padded[2]])
}

fn payload_chunk_hex(payload_chunk: &[u8]) -> String {
    let mut padded = [0u8; AFFINE_PAYLOAD_BYTES_PER_CHUNK];
    padded[..payload_chunk.len()].copy_from_slice(payload_chunk);
    format_hex(&padded)
}

fn payload_value_hex(payload_value: u32) -> String {
    let bytes = payload_value.to_be_bytes();
    format_hex(&bytes[1..])
}

fn affine_chunk_text(lane: &FastAffineLane, seed: u64) -> String {
    let mut digits = Vec::with_capacity(AFFINE_CHUNK_DIGITS);
    digits.push(AFFINE_OUTER);
    digits.push(AFFINE_INNER);
    digits.extend(seed_digits(seed, AFFINE_BASE, AFFINE_MIDDLE_LENGTH));
    digits.push(AFFINE_INNER);
    digits.push(AFFINE_OUTER);
    debug_assert_eq!(lane.template_digits(seed).len(), AFFINE_CHUNK_DIGITS);
    digits.into_iter().map(base57_digit_to_char).collect()
}

fn seed_digits(mut seed: u64, base: u32, width: usize) -> Vec<u32> {
    let mut digits = vec![0u32; width];
    for digit in digits.iter_mut().rev() {
        *digit = (seed % u64::from(base)) as u32;
        seed /= u64::from(base);
    }
    digits
}

fn digits_to_u64(digits: &[u32], base: u32) -> u64 {
    digits.iter().fold(0u64, |value, &digit| {
        value * u64::from(base) + u64::from(digit)
    })
}

fn base57_digit_to_char(digit: u32) -> char {
    alphabet_chars(CodecAlphabet::Base57)[digit as usize]
}

fn base57_char_to_digit(ch: char) -> Result<u32, Base57AffineCodecError> {
    alphabet_chars(CodecAlphabet::Base57)
        .iter()
        .position(|&candidate| candidate == ch)
        .map(|idx| idx as u32)
        .ok_or(Base57AffineCodecError::InvalidCharacter {
            alphabet: alphabet_name(CodecAlphabet::Base57),
            ch,
        })
}

fn alphabet_chars(alphabet: CodecAlphabet) -> Vec<char> {
    match alphabet {
        CodecAlphabet::Base58 => BITCOIN_BASE58_ALPHABET.chars().collect(),
        CodecAlphabet::Base57 => BASE57_ALPHABET.chars().collect(),
    }
}

fn alphabet_name(alphabet: CodecAlphabet) -> &'static str {
    match alphabet {
        CodecAlphabet::Base58 => "base58",
        CodecAlphabet::Base57 => "base57",
    }
}

fn validate_alphabet(alphabet: &[char]) -> Result<(), Base57AffineCodecError> {
    if alphabet.len() < 2 {
        return Err(Base57AffineCodecError::AlphabetTooShort);
    }
    for (idx, &ch) in alphabet.iter().enumerate() {
        if alphabet
            .iter()
            .skip(idx + 1)
            .any(|&candidate| candidate == ch)
        {
            return Err(Base57AffineCodecError::DuplicateAlphabetCharacter(ch));
        }
    }
    Ok(())
}

fn gcd_u32(mut left: u32, mut right: u32) -> u32 {
    while right != 0 {
        let tmp = left % right;
        left = right;
        right = tmp;
    }
    left
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alphabets_have_expected_lengths_and_rejections() {
        assert_eq!(BITCOIN_BASE58_ALPHABET.chars().count(), 58);
        assert_eq!(BASE57_ALPHABET.chars().count(), 57);
        assert!(BITCOIN_BASE58_ALPHABET.contains(BASE57_DROPPED_CHAR));
        assert!(!BASE57_ALPHABET.contains(BASE57_DROPPED_CHAR));
        assert!(decode_to_bytes("z", CodecAlphabet::Base57).is_err());
    }

    #[test]
    fn leading_zero_bytes_round_trip() {
        let payload = [0, 0, 1, 2, 3, 0];
        for alphabet in [CodecAlphabet::Base58, CodecAlphabet::Base57] {
            let encoded = encode_bytes(&payload, alphabet).unwrap();
            assert!(encoded.starts_with("11"));
            assert_eq!(decode_to_bytes(&encoded, alphabet).unwrap(), payload);
        }
    }

    #[test]
    fn arbitrary_bytes_round_trip_and_canonical_transcode() {
        let cases = [
            Vec::new(),
            b"hello".to_vec(),
            vec![0, 0, 0],
            (0u8..=31).collect::<Vec<_>>(),
        ];
        for payload in cases {
            let base58 = encode_bytes(&payload, CodecAlphabet::Base58).unwrap();
            let decoded = decode_to_bytes(&base58, CodecAlphabet::Base58).unwrap();
            let base57 = encode_bytes(&decoded, CodecAlphabet::Base57).unwrap();
            let decoded57 = decode_to_bytes(&base57, CodecAlphabet::Base57).unwrap();
            let canonical58 = encode_bytes(&decoded57, CodecAlphabet::Base58).unwrap();
            assert_eq!(decoded, payload);
            assert_eq!(decoded57, payload);
            assert_eq!(canonical58, base58);
        }
    }

    #[test]
    fn affine_residue_and_prime_modes_round_trip_fixed_payloads() {
        for mode in [AffineCodecMode::Residue, AffineCodecMode::Prime] {
            for payload in [b"hello".to_vec(), vec![0, 1, 2, 3, 4], Vec::new()] {
                let encoded = encode_affine(&payload, mode).unwrap();
                let decoded = decode_affine(&encoded.notation).unwrap();
                assert_eq!(decoded.payload, payload);
                assert_eq!(decoded.mode, mode);
            }
        }
    }

    #[test]
    fn affine_residue_mode_emits_residue_admissible_chunks() {
        let encoded = encode_affine(b"residue-check", AffineCodecMode::Residue).unwrap();
        assert!(!encoded.chunks.is_empty());
        assert!(encoded.chunks.iter().all(|chunk| chunk.residue_admissible));
    }

    #[test]
    fn affine_prime_mode_emits_prime_chunks() {
        let encoded = encode_affine(b"prime-check", AffineCodecMode::Prime).unwrap();
        assert!(!encoded.chunks.is_empty());
        assert!(encoded.chunks.iter().all(|chunk| chunk.prime));
    }

    #[test]
    fn base_invariant_map_preserves_payload_across_renderings() {
        let payload = [0, 0, b'h', b'i'];
        let map = build_base_invariant_map(&payload, true, true).unwrap();
        assert_eq!(map.payload_len, payload.len());
        assert_eq!(map.leading_zero_bytes, 2);
        assert_eq!(map.hex, "00006869");
        let base58 = map
            .renderings
            .iter()
            .find(|rendering| rendering.label == "base58")
            .unwrap();
        let base57 = map
            .renderings
            .iter()
            .find(|rendering| rendering.label == "base57")
            .unwrap();
        assert_eq!(
            decode_to_bytes(&base58.text, CodecAlphabet::Base58).unwrap(),
            payload
        );
        assert_eq!(
            decode_to_bytes(&base57.text, CodecAlphabet::Base57).unwrap(),
            payload
        );
        assert!(map.affine_residue.unwrap().all_chunks_residue_admissible);
        assert!(map.affine_prime.unwrap().all_chunks_prime);
    }

    #[test]
    fn canonical_payload_accepts_affine_input() {
        let encoded = encode_affine(b"map-me", AffineCodecMode::Residue).unwrap();
        let decoded =
            decode_canonical_payload(&encoded.notation, CanonicalInputFormat::Affine).unwrap();
        assert_eq!(decoded, b"map-me");
    }

    #[test]
    fn affine_decode_rejects_noncanonical_chunk_count_and_padding() {
        let extra_chunk = encode_affine(&[0, 0, 0], AffineCodecMode::Residue).unwrap();
        let noncanonical_empty = format!("a57r1:0:{}", extra_chunk.body);
        assert!(matches!(
            decode_affine(&noncanonical_empty),
            Err(Base57AffineCodecError::InvalidAffineChunkCount {
                expected: 0,
                actual: 1
            })
        ));

        let nonzero_padding = encode_affine(&[b'h', 1, 2], AffineCodecMode::Residue).unwrap();
        let noncanonical_one_byte = format!("a57r1:1:{}", nonzero_padding.body);
        assert!(matches!(
            decode_affine(&noncanonical_one_byte),
            Err(Base57AffineCodecError::NonZeroAffinePadding)
        ));
    }
}
