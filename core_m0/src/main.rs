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
    use cyt2b9_c::cpuss::rom_ctl::SLOW_WS_R;

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

        #[doc = "Setting wait state for ROM"]
        let rom_ctl = &peripherals.CPUSS.rom_ctl;
        rom_ctl.modify(|_, w| {
            w.slow_ws().variant(0x01);
            w.fast_ws().variant(0x00);
            w
        });

        let ram0_ctl0 = &peripherals.CPUSS.ram0_ctl0;
        ram0_ctl0.modify(|_, w| {
            w.slow_ws().variant(0x01);
            w.fast_ws().variant(0x00);
            w
        });

        #[cfg(CPUSS_RAMC1_PRESENT ="1")]
        {
            let ram1_ctl0 = &peripherals.CPUSS.ram1_ctl0;
            ram1_ctl0.modify(|_, w| {
                w.slow_ws().variant(0x01);
                w.fast_ws().variant(0x00);
                w
            });
        }

        #[cfg(CPUSS_RAMC2_PRESENT ="1")]
        {
            let ram_ctl0 = &peripherals.CPUSS.ram2_ctl0;
            ram_ctl0.modify(|_, w| {
                w.slow_ws().variant(0x01);
                w.fast_ws().variant(0x00);
                w
            });
        }
        
        let flast_ctl = &peripherals.FLASHC.flash_ctl;
        flast_ctl.modify(|_, w| {
            w.main_ws().variant(0x01);
            w
        });

        let clk_select = &peripherals.SRSS.clk_select;
        clk_select.modify(|_, w| {
            w.lfclk_sel().ilo0();
            w
        });

        let clk_eco_cfg2 = &peripherals.SRSS.clk_eco_config2;
        clk_eco_cfg2.modify(|_, w| {
            w.wdtrim().variant(4);
            w.atrim().variant(12);
            w.ftrim().variant(3);
            w.rtrim().variant(3);
            w.gtrim().variant(1);
            w
        });
    
        let clk_eco_cfg = &peripherals.SRSS.clk_eco_config;
        clk_eco_cfg.modify(|_, w| {
            w.eco_en().set_bit();
            w
        });
        
        let clk_eco_status = &peripherals.SRSS.clk_eco_status;
        while(clk_eco_status.read().eco_ready() == false){};
        while(clk_eco_status.read().eco_ok() == false){};

        let cm4_clock_ctl = &peripherals.CPUSS.cm4_clock_ctl;
        cm4_clock_ctl.modify(|_, w| {
            w.fast_int_div().variant(0x0);
            w
        });

        let cm0_clock_ctl = &peripherals.CPUSS.cm0_clock_ctl;
        cm0_clock_ctl.modify(|_, w| {
            w.peri_int_div().variant(0x01);
            w.slow_int_div().variant(0x00);
            w
        });

       

        let clk_path_select: &[cyt2b9_c::generic::Reg<cyt2b9_c::srss::clk_path_select::CLK_PATH_SELECT_SPEC>; 16] = &peripherals.SRSS.clk_path_select;
        let pll0_path = clk_path_select[1];
        pll0_path.modify(|_, w| {
            w.path_mux().eco();
            w
        });




        (Shared {test:[128;10]}, Local {},)
    }

    #[idle]
    fn idle(ctx: idle::Context) -> ! {
        loop {
            defmt::info!("Hello, RTIC!");
        }
    }
}