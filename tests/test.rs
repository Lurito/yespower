#![allow(non_snake_case)]

use yespower::*;

#[cfg(test)]
mod yespower_hash_test {
    use super::*;
    use libc::c_char;

    const INPUT:[u8; 80] = [
        0x00, 0x03, 0x06, 0x09, 0x0c, 0x0f, 0x12, 0x15,
        0x18, 0x1b, 0x1e, 0x21, 0x24, 0x27, 0x2a, 0x2d,
        0x30, 0x33, 0x36, 0x39, 0x3c, 0x3f, 0x42, 0x45,
        0x48, 0x4b, 0x4e, 0x51, 0x54, 0x57, 0x5a, 0x5d,
        0x60, 0x63, 0x66, 0x69, 0x6c, 0x6f, 0x72, 0x75,
        0x78, 0x7b, 0x7e, 0x81, 0x84, 0x87, 0x8a, 0x8d,
        0x90, 0x93, 0x96, 0x99, 0x9c, 0x9f, 0xa2, 0xa5,
        0xa8, 0xab, 0xae, 0xb1, 0xb4, 0xb7, 0xba, 0xbd,
        0xc0, 0xc3, 0xc6, 0xc9, 0xcc, 0xcf, 0xd2, 0xd5,
        0xd8, 0xdb, 0xde, 0xe1, 0xe4, 0xe7, 0xea, 0xed
    ];
    
    #[test]
    // Test for Yespower - N: 2048, r: 32
    fn yespower() {
        let mut output = [0u8; 32];
        unsafe { yespower_hash(INPUT.as_ptr() as *const c_char, output.as_mut_ptr() as *mut c_char) };
        assert_eq!(hex::encode(output), "d5efb813cd263e9b34540130233cbbc6a921fbff3431e5ec1a1abde2aea6ff4d")
    }

    #[test]
    // Test for TIDE - N: 2048, r: 8
    fn yespowerTIDE() {
        let mut output = [0u8; 32];
        unsafe { yespowerTIDE_hash(INPUT.as_ptr() as *const c_char, output.as_mut_ptr() as *mut c_char, INPUT.len() as u32) };
        assert_eq!(hex::encode(output), "69e0e895b3df7aeeb837d71fe199e9d34f7ec46ecbca7a2c4308e51857ae9b46")
    }

    #[test]
    // Test for R16 - N: 4096, r: 16
    fn yespowerR16() {
        let mut output = [0u8; 32];
        unsafe { yespowerR16_hash(INPUT.as_ptr() as *const c_char, output.as_mut_ptr() as *mut c_char) };
        assert_eq!(hex::encode(output), "33fb8f063824a4a020f63dca535f5ca66ab5576468c75d1ccaac7542f76495ac")
    }

    #[test]
    // Test for RES - N: 4096, r: 32
    fn yespowerRES() {
        let mut output = [0u8; 32];
        unsafe { yespowerRES_hash(INPUT.as_ptr() as *const c_char, output.as_mut_ptr() as *mut c_char) };
        assert_eq!(hex::encode(output), "48f4013c05089d90210632567476602a1410b29145bcf049c4ea39f6f05caf80")
    }

    #[test]
    // Test for RES (length of 80) - N: 4096, r: 32, l: 80
    fn yespowerRES_80() {
        let mut output = [0u8; 32];
        let params = yespower_params_t {
            version: YESPOWER_1_0,
            N: 4096,
            r: 32,
            pers: std::ptr::null(),
            perslen: 0,
        };

        unsafe { yespower_tls(
            INPUT.as_ptr(),
            INPUT.len(),
            &params as *const yespower_params_t,
            output.as_mut_ptr() as *mut yespower_binary_t
        ) };
        assert_eq!(hex::encode(output), "771aeefda8fe79a0825bc7f2aee162ab5578574639ffc6ca3723cc18e5e3e285")
    }

    #[test]
    // Test for customize - N: 1024, r: 32
    fn customize_1024_32() {
        let mut output = [0u8; 32];
        let params = yespower_params_t {
            version: YESPOWER_1_0,
            N: 1024,
            r: 32,
            pers: std::ptr::null(),
            perslen: 0,
        };

        unsafe { yespower_tls(
            INPUT.as_ptr(),
            INPUT.len(),
            &params as *const yespower_params_t,
            output.as_mut_ptr() as *mut yespower_binary_t
        ) };
        assert_eq!(hex::encode(output), "501b792db42e388f6e7d453c95d03a12a36016a5154a688390ddc609a40c6799")
    }

    #[test]
    // Test for customize - N: 1024, r: 32, with pers "personality test"
    fn customize_1024_32_with_pers() {
        let mut output = [0u8; 32];
        let pers = "personality test";
        let params = yespower_params_t {
            version: YESPOWER_1_0,
            N: 1024,
            r: 32,
            pers: pers.as_ptr(),
            perslen: pers.len(),
        };

        unsafe { yespower_tls(
            INPUT.as_ptr(),
            INPUT.len(),
            &params as *const yespower_params_t,
            output.as_mut_ptr() as *mut yespower_binary_t
        ) };
        assert_eq!(hex::encode(output), "1f0269acf565c49adc0ef9b8f26ab3808cdc38394a254fddeedcc3aacff6ad9d")
    }

    #[test]
    // Test for Power2b
    fn power2b() {
        let mut output = [0u8; 32];
        unsafe { power2b_hash(INPUT.as_ptr() as *const c_char, output.as_mut_ptr() as *mut c_char, INPUT.len() as u32) };
        assert_eq!(hex::encode(output), "e5516eea8387ad39eeb4fb5cc4d3e858bc7dfa4a9ec941b255c6eb5e8c470e3a")
    }
}
