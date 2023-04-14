use mylib::factory::{produce_refrigerator, produce_washing_machine};
fn main() {
    // 绝对路径
    mylib::factory::produce_refrigerator::hit_the_screw();
    mylib::factory::produce_washing_machine::packing();

    // 使用use调用
    produce_refrigerator::hit_the_screw();
    produce_washing_machine::packing();
}
