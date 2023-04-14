// 车间有生产冰箱的模块
pub mod produce_refrigerator {
    // 打螺丝 函数
    pub fn hit_the_screw() {
        println!("螺丝+1");
    }

    // 电线 函数
    pub fn wire() {
        println!("电线+1");
    }
}

// 车间有生产洗衣机的模块
pub mod produce_washing_machine {
    // 生产滚筒 函数
    pub fn roller() {
        println!("滚筒+1");
    }

    // 包装 函数
    pub fn packing() {
        println!("包装+1");
    }
}
