#![no_std]
#![no_main]

mod time;
mod led;
mod button;
mod channel;
mod future;
mod executor;
mod gpiote;

// use core::ptr::write_volatile;
// use cortex_m::asm::nop;
use cortex_m_rt::entry;
// use embedded_hal::delay::DelayNs;
// use embedded_hal::digital::InputPin;
use embedded_hal::digital::OutputPin;
use microbit::Board;
// use nrf52833_hal as hal;
// use nrf52833_hal::gpio::Level;
// use nrf52833_hal::Timer;
// use nrf52833_pac::Peripherals;
use rtt_target::{rprintln, rtt_init_print};
use crate::button::{ButtonDirection, ButtonTask};
use crate::channel::Channel;
use crate::executor::run_tasks;
use crate::future::OurFuture;
use crate::led::LedTask;
use crate::time::Ticker;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    led_buttons_loop()
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    rprintln!("panic!");
    loop {
        cortex_m::asm::bkpt();
    }
}

fn led_buttons_loop() ->  ! {
    rprintln!("Hello, world! led buttons loop");
    let mut board = Board::take().unwrap();
    Ticker::init(board.RTC0, &mut board.NVIC);
    let (col, mut row) = board.display_pins.degrade();

    row[0].set_high().ok();

    let button_l = board.buttons.button_a.degrade();
    let button_r = board.buttons.button_b.degrade();

    let channel: Channel<ButtonDirection> = Channel::new();

    let mut led_task = LedTask::new(col, channel.get_receiver());
    let mut button_l_task = ButtonTask::new(button_l, ButtonDirection::Left, channel.get_sender());
    let mut button_r_task = ButtonTask::new(button_r, ButtonDirection::Right, channel.get_sender());

    let mut tasks: [&mut dyn OurFuture<Output = ()>; 3] = [
        &mut led_task,
        &mut button_l_task,
        &mut button_r_task,
    ];

    run_tasks(&mut tasks);
}

// fn peripheral_blinky_loop() -> ! {
//     rprintln!("Hello, world! peripheral loop");
//
//     const GPIO0_PINCNF21_ROW1_ADDR: *mut u32 = 0x5000_0754 as *mut u32;
//     const GPIO0_PINCNF28_COL1_ADDR: *mut u32 = 0x5000_0770 as *mut u32;
//
//     const GPIO0_PINCNF19_ROW5_ADDR: *mut u32 = 0x5000_074C as *mut u32;
//     const GPIO0_PINCNF30_COL5_ADDR: *mut u32 = 0x5000_0778 as *mut u32;
//
//     const DIR_OUTPUT_POS: u32 = 0;
//     const PINCNF_DRIVE_LED: u32 = 1 << DIR_OUTPUT_POS;
//     const PINCNF_RESET: u32 = 1 << 1;
//
//     const GPIO0_OUT_ADDR: *mut u32 = 0x5000_0504 as *mut u32;
//     const GPIO0_OUT_ROW1_POS: u32 = 21;
//     const GPIO0_OUT_ROW5_POS: u32 = 19;
//     let mut is_on = false;
//
//     loop {
//         let flags = ((is_on as u32) << GPIO0_OUT_ROW1_POS) | ((!is_on as u32) << GPIO0_OUT_ROW5_POS);
//         rprint!("flags: {:#034b}", flags);
//
//         unsafe {
//             write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_RESET);
//             write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_RESET);
//
//             write_volatile(GPIO0_PINCNF19_ROW5_ADDR, PINCNF_RESET);
//             write_volatile(GPIO0_PINCNF30_COL5_ADDR, PINCNF_RESET);
//
//             if is_on {
//                 write_volatile(GPIO0_PINCNF21_ROW1_ADDR, PINCNF_DRIVE_LED);
//                 write_volatile(GPIO0_PINCNF28_COL1_ADDR, PINCNF_DRIVE_LED);
//             } else {
//                 write_volatile(GPIO0_PINCNF19_ROW5_ADDR, PINCNF_DRIVE_LED);
//                 write_volatile(GPIO0_PINCNF30_COL5_ADDR, PINCNF_DRIVE_LED);
//             }
//
//             write_volatile(GPIO0_OUT_ADDR, flags);
//         }
//
//         for _ in 0..100_000 {
//             nop()
//         }
//
//         is_on = !is_on;
//
//     }
// }

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

// fn hal_blinky_loop() -> ! {
//     rprintln!("Hello, world! hal loop");
//     let pac = hal::pac::Peripherals::take().unwrap();
//     let port0 = hal::gpio::p0::Parts::new(pac.P0);
//
//     let mut col1 = port0.p0_28.into_push_pull_output(Level::Low);
//     let mut row1 = port0.p0_21.into_push_pull_output(Level::Low);
//     let mut col5 = port0.p0_30.into_push_pull_output(Level::Low);
//     let mut row5 = port0.p0_19.into_push_pull_output(Level::Low);
//
//     let mut is_on = false;
//
//     loop {
//         let _ = row1.set_state(PinState::from(is_on));
//         let _ = col1.set_state(PinState::from(!is_on));
//
//         let _ = row5.set_state(PinState::from(!is_on));
//         let _ = col5.set_state(PinState::from(is_on));
//
//         for _ in 0..100_000 {
//             nop()
//         }
//
//         is_on = !is_on;
//     }
// }

// fn bsp_blinky_loop() -> ! {
//     rprintln!("Hello, world! bsp loop");
//     let board = Board::take().unwrap();
//     let mut timer = Timer::new(board.TIMER0);
//     let (mut col, mut row) = board.display_pins.degrade();
//
//     row[0].set_high().ok();
//     col[0].set_low().ok();
//
//     loop {
//         timer.delay_ms(500);
//
//         col[0].toggle().ok();
//         row[0].toggle().ok();
//         col[4].toggle().ok();
//         row[4].toggle().ok();
//     }
// }


