// Hard-coded endpoints for quick testing.
// Replace the example URLs below with your real endpoints (scheme + host [+ :port]).
// The code uses only the URL origin (scheme + host + optional port), so provide a valid
// http(s) origin. If you want to disable hardcoding later, set these to `None`.
pub static mut ENDPOINTS: Endpoints = Endpoints {
    // 示例: Some("https://dispatch.example.com".to_string())
    // 请根据需要替换为你要重定向到的 dispatch 服务地址
    dispatch: Some("http://8.138.225.248:8888".to_string()),

    // 示例: Some("https://sdk.example.com".to_string())
    // 请根据需要替换为你要重定向到的 sdk 服务地址
    sdk: Some("http://8.138.225.248:22101".to_string()),
};

pub struct Endpoints {
    pub dispatch: Option<String>,
    pub sdk: Option<String>,
}