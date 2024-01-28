use sha256::try_digest;
use std::path::Path;

pub fn check256(sha256token: &str, location: &str) -> bool {
    let input = Path::new(location);
    let val = try_digest(input).unwrap();

    return if val == sha256token { true } else { false };
}
