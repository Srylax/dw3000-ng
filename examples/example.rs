#![no_main]
#![no_std]

// crates de gestion des messages de debug
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

//use stm32f1::stm32f103;
//use stm32f1::stm32f103::interrupt;

use cortex_m_rt::entry;
use stm32f1xx_hal::{
    pac,
    prelude::*,
    spi::{Spi, Mode, Phase, Polarity},
    //delay::Delay,
};

use dw3000::{hl, mac};

#[entry]
fn main() -> ! {

    rtt_init_print!();
    rprintln!("Coucou copain !");

    /****************************************************************************************/
    /*****************              CONFIGURATION DE BASE               *********************/
    /****************************************************************************************/

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();
    //let cp = cortex_m::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let clocks = rcc.cfgr.use_hse(8.mhz()).sysclk(36.mhz()).freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);

    /****************************************************************************************/
    /*****************              CONFIGURATION DU SPI                *********************/
    /****************************************************************************************/

    let pins = (
        gpioa.pa5.into_alternate_push_pull(&mut gpioa.crl),
        gpioa.pa6.into_floating_input(&mut gpioa.crl),
        gpioa.pa7.into_alternate_push_pull(&mut gpioa.crl),
    );

    let cs = gpiob.pb6.into_push_pull_output(&mut gpiob.crl);

    let spi_mode = Mode {
        polarity: Polarity::IdleLow,
        phase: Phase::CaptureOnFirstTransition,
    };
    let spi = Spi::spi1(dp.SPI1, pins, &mut afio.mapr, spi_mode, 100.khz(), clocks, &mut rcc.apb2);

    //let mut delay = Delay::new(cp.SYST, clocks);

    /****************************************************************************************/
    /*****************              CONFIGURATION du DW3000               *******************/
    /****************************************************************************************/

    let mut dw3000 = hl::DW1000::new(spi, cs).init()
        .expect("Failed to initialize DW1000");
    rprintln!("dm3000 = {:?}", dw3000);
    

/*
    // test du registre dev_id
    let dev_id = dw3000.ll().dev_id().read()
        .expect("Failed to read DEV_ID register");
    rprintln!("dev-id = {:?}", dev_id);
*/


/*
    dw3000initiated.set_address( mac::PanId(0x0d57),
                                 mac::ShortAddress(50))
                    .expect("Failed to set address.");
*/
    
    let sys_clk = dw3000.ll().clk_ctrl().read().unwrap().sys_clk();
    rprintln!("sys_clk = {:#x?}", sys_clk);

    let ainit2idle = dw3000.ll().seq_ctrl().read().unwrap().ainit2idle();
    rprintln!("ainit2idle = {:#x?}", ainit2idle);

    loop {
        
    }    

}
