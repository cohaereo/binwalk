use crate::signatures::common::{SignatureError, SignatureResult, CONFIDENCE_HIGH};
use crate::structures::ttf::{calculate_ttf_file_size, parse_ttf_header};

use super::common::CONFIDENCE_MEDIUM;

/// Human readable description
pub const DESCRIPTION: &str = "OpenType font";

/// OTF file entry magic bytes
pub fn otf_magic() -> Vec<Vec<u8>> {
    vec![b"OTTO".to_vec()]
}

/// Validates an OTF file entry signature
pub fn otf_parser(file_data: &[u8], offset: usize) -> Result<SignatureResult, SignatureError> {
    // Success return value
    let mut result = SignatureResult {
        offset,
        description: DESCRIPTION.to_string(),
        confidence: CONFIDENCE_MEDIUM,
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
