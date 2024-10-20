//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // For tests7: Set up an environment variable called `TEST_FOO` 
    // with the current Unix timestamp (in seconds).
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Setting the TEST_FOO environment variable for Cargo.
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // For tests8: Enable the "pass" feature to make the testcase return early.
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

