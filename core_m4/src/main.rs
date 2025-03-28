#![no_std]
#![no_main]

use cyt2b9_c as pac;
use panic_halt as _;

#[rtic::app(device = pac)]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        // 定义共享资源
    }

    #[local]
    struct Local {
        // 定义本地资源
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        // 初始化代码

        let mut peripherals = pac::Peripherals::take().unwrap();

        (Shared {}, Local {},)
    }

    // #[idle]
    // fn idle(ctx: idle::Context) -> ! {
    //     loop {
    //         // 空闲任务
    //     }
    // }
}