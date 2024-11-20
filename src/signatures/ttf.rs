use crate::signatures::common::{SignatureError, SignatureResult, CONFIDENCE_HIGH, CONFIDENCE_LOW};
use crate::structures::ttf::{calculate_ttf_file_size, parse_ttf_header};

/// Human readable description
pub const DESCRIPTION: &str = "TrueType font";

/// TTF file magic bytes
pub fn ttf_magic() -> Vec<Vec<u8>> {
    vec![b"\x00\x01\x00\x00".to_vec(), b"true".to_vec()]
}

/// Validates an TTF file entry signature
pub fn ttf_parser(file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // Success return value
    let mut result = SignatureResult {
        offset,
        description: DESCRIPTION.to_string(),
        confidence: CONFIDENCE_LOW,
        ..Default::default()
    };

    if let Ok(header) = parse_ttf_header(&file_data[offset..]) {
        if let Some(size) = calculate_ttf_file_size(&header) {
            result.confidence = CONFIDENCE_HIGH;
            result.size = size;
            return Ok(result);
        }
    }

    Err(SignatureError)
}
