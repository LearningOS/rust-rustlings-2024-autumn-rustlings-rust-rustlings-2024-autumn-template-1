//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
fn main() {
    // 获取当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 设置环境变量
    println!("cargo:TEST_FOO={}", timestamp);
}


