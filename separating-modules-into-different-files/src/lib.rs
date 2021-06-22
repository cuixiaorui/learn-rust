// 通过 mod 来拆分代码
// 1. 需要使用 mod 来导入文件
// 文件和这里的名称一致即可
mod front_of_house;

// 这里也必须导入，不然用不了
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
