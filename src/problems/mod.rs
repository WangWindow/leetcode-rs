//! 题目模块与注册表
pub mod p812; // 示例：812. Largest Triangle Area
pub mod p976; // 示例：976. Largest Perimeter Triangle

pub static REGISTRY: &[(&str, fn())] = &[
    ("812", p812::run),
    ("976", p976::run),
    // 继续在此追加新的题目映射，例如：("1", p1::run),
];

/// 根据题号调度对应题目的运行函数
pub fn dispatch(id: &str) -> Option<fn()> {
    REGISTRY.iter().find(|(k, _)| k == &id).map(|(_, f)| *f)
}
