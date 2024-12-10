#![no_std]
#![no_main]
extern crate alloc;

use core::ptr::write_volatile;
use cortex_m::asm::nop;
use cortex_m_rt::entry;
use nrf52833_hal as hal;
// use nrf52833_pac::Peripherals;
use rtt_target::{rprint, rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // peripheral_blinky_loop();
    // pac_blinky_loop()
    hal_blinky_loop()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}

fn peripheral_blinky_loop() -> ! {
    rprintln!("Hello, world! peripheral loop");

    const GPIO0_PINCNF21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
    const GPIO0_PINCNF28_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;

    const GPIO0_PINCNF19_ROW5_ADDR: *mut u32 = 0x5000_074C as *mut u32;
    const GPIO0_PINCNF30_COL5_ADDR: *mut u32 = 0x5000_0778 as *mut u32;

    const DIR_OUTPUT_POS: u32 = 0;
    const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
    const PINCNF_RESET: u32 = 1 << 1;

    const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
    const GPIO0_OUT_ROW1_POS: u32 = 21;
    const GPIO0_OUT_ROW5_POS: u32 = 19;
    let mut is_on = false;

    loop {
        let flags = ((is_on as u32) << GPIO0_OUT_ROW1_POS) | ((!is_on as u32) << GPIO0_OUT_ROW5_POS);
        rprint!("flags: {:#034b}", flags);

        unsafe {
            write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_RESET);
            write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_RESET);

            write_volatile(GPIO0_PINCNF19_ROW5_ADDR, PINCNF_RESET);
            write_volatile(GPIO0_PINCNF30_COL5_ADDR, PINCNF_RESET);

            if is_on {
                write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_DRIVE_LED);
                write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_DRIVE_LED);
            } else {
                write_volatile(GPIO0_PINCNF19_ROW5_ADDR, PINCNF_DRIVE_LED);
                write_volatile(GPIO0_PINCNF30_COL5_ADDR, PINCNF_DRIVE_LED);
            }

            write_volatile(GPIO0_OUT_ADDR, flags);
        }

        for _ in 0..100_000 {
            nop()
        }

        is_on = !is_on;

    }
}

// fn pac_blinky_loop() -> ! {
//     let pac = Peripherals::take().unwrap();
//
//     let output_pins = [21, 28, 19, 30];
//     let mut is_on = false;
//
//     loop {
//         for pin in output_pins.iter() {
//             pac.P0.pin_cnf[*pin].reset();
//         }
//
//         if is_on {
//             pac.P0.pin_cnf[21].write(|writer| writer.dir().output());
//             pac.P0.pin_cnf[28].write(|writer| writer.dir().output());
//             pac.P0.out.write(|writer| writer.pin21().bit(true));
//         } else {
//             pac.P0.pin_cnf[19].write(|writer| writer.dir().output());
//             pac.P0.pin_cnf[30].write(|writer| writer.dir().output());
//             pac.P0.out.write(|writer| writer.pin19().bit(true));
//         }
//
//         for _ in 0..100_000 {
//             nop()
//         }
//
//         is_on = !is_on;
//     }
// }

fn hal_blinky_loop() -> ! {
    let pac = hal::pac::Peripherals::take().unwrap();
    let port0 = hal::gpio::p0::Parts::new(pac.P0);
    let mut button = port0.p0_13.into_pullup_input();

    loop {

    }
}