use embedded_hal::digital::InputPin;
use microbit::hal::gpio::{Floating, Input, Pin};
use crate::time::Timer;
use fugit::ExtU64;
use rtt_target::rprintln;
use crate::channel::Sender;
use crate::future::{OurFuture, Poll};

#[derive(Clone, Copy, Debug)]
pub enum ButtonDirection {
    Left,
    Right,
}

enum ButtonState {
    WaitForPress,
    Debounce(Timer)
}

pub struct ButtonTask<'a> {
    pin: Pin<Input<Floating>>,
    direction: ButtonDirection,
    sender: Sender<'a, ButtonDirection>,
    state: ButtonState,
}

impl<'a> ButtonTask<'a> {
    pub fn new(
        pin: Pin<Input<Floating>>,
        direction: ButtonDirection,
        sender: Sender<'a, ButtonDirection>,
    ) -> Self {
        Self { pin, direction, state: ButtonState::WaitForPress, sender }
    }

    // pub fn poll(&mut self) {
    //     match self.state {
    //         ButtonState::WaitForPress => {
    //             if self.pin.is_low().unwrap() {
    //                 self.sender.send(self.direction);
    //                 self.state = ButtonState::Debounce(Timer::new(100.millis()));
    //             }
    //         }
    //
    //         ButtonState::Debounce(ref timer) => {
    //             rprintln!("Debouncing");
    //             if timer.is_ready() && self.pin.is_high().unwrap() {
    //                 self.state = ButtonState::WaitForPress;
    //             }
    //         }
    //     }
    // }
}

impl OurFuture for ButtonTask<'_> {
    type Output = ();

    fn poll(&mut self, task_id: usize) -> Poll<Self::Output> {
        loop {
            match self.state {
                ButtonState::WaitForPress => {
                    if self.pin.is_low().unwrap() {
                        self.sender.send(self.direction);
                        self.state = ButtonState::Debounce(Timer::new(100.millis()));
                        continue;
                    }
                }

                ButtonState::Debounce(ref timer) => {
                    rprintln!("Debouncing");
                    if timer.is_ready() && self.pin.is_high().unwrap() {
                        self.state = ButtonState::WaitForPress;
                        continue;
                    }
                }
            }
            break
        }
        Poll::Pending
    }
}