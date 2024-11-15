#![allow(non_snake_case)]
#![feature(test)]

extern crate test;

use rust_yespower::*;

#[cfg(test)]
mod performance {
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

    #[bench]
    // Test for Power2b
    fn power2b_1ki(b: &mut test::Bencher) {
        use std::thread;
        use std::time::Instant;
        use std::sync::{Arc, Barrier};

        let iterations = 1024;
        let thread_count = 16;
        let iterations_per_thread = iterations / thread_count;

        // Unsafe shared output buffer
        let mut output = vec![0u8; 32];
        let output_ptr = output.as_mut_ptr();
        let output_ptr_usize: usize = output_ptr as usize;

        // Recoed the start time
        let start = Instant::now();

        b.iter(|| {
            // Use Barrier to wait for threads
            let barrier = Arc::new(Barrier::new(thread_count + 1));

            for _ in 0..thread_count {
                let barrier_clone = Arc::clone(&barrier);

                thread::spawn(move || {
                    let output_ptr: *mut u8 = unsafe { std::mem::transmute(output_ptr_usize) };
                    for _ in 0..iterations_per_thread {
                        unsafe {
                            power2b_hash(
                                INPUT.as_ptr() as *const c_char,
                                output_ptr as *mut c_char,
                                INPUT.len() as u32,
                            );
                        }
                    }
                    // Wait for all threads to reach Barrier
                    barrier_clone.wait();
                });
            }

            // Wait for all threads to reach Barrier in main thread
            barrier.wait();

            let duration = start.elapsed();
            println!("power2b: {:?}", duration);

            // 验证输出
            assert_eq!(
                hex::encode(test::black_box(&output)),
                "e5516eea8387ad39eeb4fb5cc4d3e858bc7dfa4a9ec941b255c6eb5e8c470e3a"
            );
        });
    }
}
