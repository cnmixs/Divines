// Divines - 工具模块
// 参考原项目: astrostudyui/src/utils/

/// 本地存储工具
pub mod storage {
    /// 保存到本地存储
    pub fn save_item(key: &str, value: &str) {
        // 在 wasm 环境中使用 web_sys::window().local_storage()
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.set_item(key, value);
                }
            }
        }
        let _ = (key, value);
    }

    /// 从本地存储读取
    pub fn get_item(key: &str) -> Option<String> {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(value)) = storage.get_item(key) {
                        return Some(value);
                    }
                }
            }
        }
        None
    }

    /// 删除本地存储项
    pub fn remove_item(key: &str) {
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    let _ = storage.remove_item(key);
                }
            }
        }
        let _ = key;
    }
}

/// 日期时间工具
pub mod datetime {
    /// 格式化日期时间
    pub fn format_datetime(year: i32, month: u32, day: u32, hour: u32, minute: u32) -> String {
        format!("{}-{:02}-{:02}T{:02}:{:02}", year, month, day, hour, minute)
    }

    /// 获取当前日期时间字符串
    pub fn now_string() -> String {
        let now = chrono::Local::now();
        now.format("%Y-%m-%dT%H:%M").to_string()
    }
}

/// 格式化工具
pub mod format {
    /// 格式化角度
    pub fn format_angle(degrees: f64) -> String {
        let d = degrees.floor() as i32;
        let m = ((degrees - d as f64) * 60.0).floor() as i32;
        let s = ((degrees - d as f64 - m as f64 / 60.0) * 3600.0).round() as i32;
        format!("{}°{}'{}\"", d, m, s)
    }
}