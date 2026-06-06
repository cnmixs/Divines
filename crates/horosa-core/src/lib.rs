// 星阙 Horosa - 核心类型定义
// 参考原项目: Horosa-Web/astrostudyui/src/models/ 和 astrostudysrv 的实体类

pub mod chart;
pub mod astrology;
pub mod bazi;
pub mod ziwei;
pub mod sanshi;
pub mod liuyao;
pub mod qizheng;
pub mod calendar;
pub mod error;
pub mod user;
pub mod ai;

// 重新导出常用类型
pub use chart::*;
pub use astrology::*;
pub use bazi::*;
pub use ziwei::*;
pub use sanshi::*;
// 避免 DiZhi 歧义：bazi 和 sanshi 都定义了 DiZhi，优先使用 bazi 的
pub use bazi::DiZhi;
pub use liuyao::*;
pub use qizheng::*;
pub use calendar::*;
pub use error::*;
pub use user::*;
pub use ai::*;