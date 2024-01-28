use std::path::Path;
use sha256::try_digest;

pub fn check256(sha256token: &str, location: &str) -> bool {


   let input = Path::new(location);
   let val = try_digest(input).unwrap();

   return if val == sha256token {
      true
   } else {
      false
   }

}