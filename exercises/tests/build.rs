//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.
//use std::env;
fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    // 使用 rustc-env 设置环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 打印一条警告信息，显示 TEST_FOO 的值
    println!("cargo:warning=TEST_FOO is set to {}", timestamp);
    // let your_command = format!(
    //     "Your command here with {}, please checkout exercises/tests/build.rs",
    //     timestamp
    // );
    // println!("cargo:{}", your_command);

    

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    //let your_command = "Your command here, please checkout exercises/tests/build.rs";
    //println!("cargo:{}", your_command);
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
