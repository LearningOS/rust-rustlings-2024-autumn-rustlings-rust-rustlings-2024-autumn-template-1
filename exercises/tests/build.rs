//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.


fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
   // let your_command = format!(
       // {rustc-env=KEY=VALUE},
       // timestamp
   // );
   let test_foo_value = timestamp.to_string(); 
   println!("cargo:rustc-env=TEST_FOO={}", test_foo_value);  
    //println!("cargo:{}", your_command);
    println!("cargo:rustc-env=KEY=VALUE");  

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    //let your_command = "Your command here, please checkout exercises/tests/build.rs";
    //println!("cargo:{}", your_command);
    println!("cargo:rustc-cfg=feature=\"pass\""); 
}
