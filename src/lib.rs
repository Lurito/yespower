extern crate libc;
use libc::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


pub fn hash(input: &[u8; 80]) -> Result<[u8; 32], &'static str> {
    let mut output = [0u8; 32];
    
    let result = unsafe {
        yespower_hash(input.as_ptr() as *const c_char, output.as_mut_ptr() as *mut c_char)
    };

    if result == 0 {
        Ok(output)
    } else {
        Err("Failed to compute yespower hash")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    #[test]
    fn test_hash() {
        let input_hex = "0000002009f42768de3cfb4e58fc56368c1477f87f60e248d7130df3fb8acd7f6208b83a72f90dd3ad8fe06c7f70d73f256f1e07185dcc217a58b9517c699226ac0297d2ad60ba61b62a021d9b7700f0";
        let expected_output_hex = "9d90c21b5a0bb9566d2999c5d703d7327ee3ac97c020d387aa2dfd0700000000";

        let input_bytes: [u8; 80] = hex::decode(input_hex).expect("Decoding failed").try_into().expect("Incorrect input length");
        let expected_output_bytes: [u8; 32] = hex::decode(expected_output_hex).expect("Decoding failed").try_into().expect("Incorrect output length");
        
        match hash(&input_bytes) {
            Ok(output) => assert_eq!(output, expected_output_bytes),
            Err(e) => panic!("Hashing failed: {}", e),
        }

    }
}