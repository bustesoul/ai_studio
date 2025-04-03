use anyhow::Result;
use tokio::time::{sleep, Duration};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(sync)] // 如果是简单计算，可以用 sync
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
// 或者一个异步函数 (默认)
#[flutter_rust_bridge::frb]
pub async fn complex_calculation(input: String) -> Result<String> {
    sleep(Duration::from_millis(10)).await;
    Ok(format!("Processed: {}", input))
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
