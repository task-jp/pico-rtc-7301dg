#![no_std]
#![no_main]

use bsp::entry;
use bsp::hal;
use defmt::*;
use defmt_rtt as _;
use embedded_hal::digital::v2::OutputPin;
use hal::{
    clocks::{init_clocks_and_plls, Clock},
    // gpio::Pin,
    pac,
    sio::Sio,
    watchdog::Watchdog,
};
use panic_probe as _;
use rp_pico as bsp;

mod rtc7301dg;

#[entry]
fn main() -> ! {
    info!("Program start");
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let sio = Sio::new(pac.SIO);

    // External high-speed crystal on the pico board is 12Mhz
    let external_xtal_freq_hz = 12_000_000u32;
    let clocks = init_clocks_and_plls(
        external_xtal_freq_hz,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let pins = bsp::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.led.into_push_pull_output();

    // RTC-7301 pins
    let rtc_cs0 = pins.gpio15.into_push_pull_output();
    let rtc_cs1 = pins.gpio17.into_push_pull_output();
    let rtc_a0 = pins.gpio10.into_push_pull_output();
    let rtc_a1 = pins.gpio11.into_push_pull_output();
    let rtc_a2 = pins.gpio12.into_push_pull_output();
    let rtc_a3 = pins.gpio13.into_push_pull_output();
    let rtc_rd = pins.gpio6.into_push_pull_output();
    let rtc_wr = pins.gpio27.into_push_pull_output();
    let rtc_d0 = pins.gpio18.into_push_pull_output();
    let rtc_d1 = pins.gpio19.into_push_pull_output();
    let rtc_d2 = pins.gpio20.into_push_pull_output();
    let rtc_d3 = pins.gpio21.into_push_pull_output();

    let mut rtc7301 = rtc7301dg::MyDevice::new(
        rtc_cs0, rtc_cs1, rtc_rd, rtc_wr, rtc_a0, rtc_a1, rtc_a2, rtc_a3, rtc_d0, rtc_d1, rtc_d2,
        rtc_d3,
    );

    // set bank 0
    rtc7301.init(&mut delay, rtc7301dg::Bank::Bank0);

    loop {
        let mut data_values: [Option<bool>; 4] =
            [Some(false), Some(false), Some(false), Some(false)];
        rtc7301.read(
            &mut delay,
            0x00, // Seconds
            &mut data_values,
        );
        for d in 0..4 {
            if let Some(d) = data_values[d] {
                led.set_state(d.into()).unwrap();
            } else {
                led.set_low().unwrap();
            }
            delay.delay_ms(250);
        }
    }
}

// End of file
