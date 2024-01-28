use sha256::try_digest;
use std::path::Path;

/// Checks the checksum of a file
///
/// # How to use
/// ```no_run
/// check256(sha256token: &str, location: &str) -> bool
/// ```
pub fn check256(sha256token: &str, location: &str) -> bool {
    let input = Path::new(location);
    let val = try_digest(input).unwrap();

    return if val == sha256token { true } else { false };
}
