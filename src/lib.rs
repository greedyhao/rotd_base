#![no_std]

mod rotd_core;
pub use rotd_core::*;

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    fn read_pw_ctrl(pw: &mut PassWord) {}
    fn write_pw_ctrl(pw: &PassWord) {}
    fn open_door_ctrl() {}
    fn buzzer_ctrl(ring: bool) {}

    #[test]
    fn test_base() {
        let ctrl = ROTDCtrl::new(read_pw_ctrl, write_pw_ctrl, open_door_ctrl, buzzer_ctrl);
        let _rotd_core = ROTDCore::new(ctrl);
    }
}
