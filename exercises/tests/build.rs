use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    // 获取当前时间的时间戳
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 在 tests7 中，我们需要设置一个名为 `TEST_FOO` 的环境变量
    // 这里我们使用 `println!` 宏来输出一个特殊的格式，Cargo 会解析这个格式并设置环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 在 tests8 中，我们需要启用 "pass" 特征来让测试用例提前返回
    // 我们使用 `println!` 宏来输出一个特殊的格式，告诉 Cargo 启用 "pass" 特征
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
