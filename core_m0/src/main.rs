#![no_std]
#![no_main]

// use cyt2b9_c as pac;
use cyt2b9_c as pac;
use defmt::info;
use defmt_rtt as _; 
use panic_halt as _;

// fn test() {
//     let mut peripherals = pac::Peripherals::take().unwrap();
// }

#[rtic::app(device = pac, peripherals = true)]
mod app {
    use super::*;
    // use super::device;

    #[shared]
    struct Shared {
        // 定义共享资源
        test:[u16; 10],
    }

    #[local]
    struct Local {
        // 定义本地资源
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        // 初始化代码
        let peripherals = ctx.device;
        let mut cpuss = peripherals.CPUSS;


     


        (Shared {test:[128;10]}, Local {},)
    }

    #[idle]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            defmt::info!("Hello, RTIC!");
        }
    }
}