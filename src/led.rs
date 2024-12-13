use crate::button::ButtonDirection;
use crate::channel::Receiver;
use crate::time::Timer;
use embedded_hal::digital::{OutputPin, StatefulOutputPin};
use fugit::ExtU64;
use microbit::{
    gpio::NUM_COLS,
    hal::gpio::{Output, Pin, PushPull},
};
use rtt_target::rprintln;
use crate::future::{OurFuture, Poll};

enum LedState {
    Toggle,
    Wait(Timer),
}

pub struct LedTask<'a> {
    col: [Pin<Output<PushPull>>; NUM_COLS],
    active_col: usize,
    receiver: Receiver<'a, ButtonDirection>,
    state: LedState
}

impl<'a> LedTask<'a> {
    pub fn new(
        col: [Pin<Output<PushPull>>; NUM_COLS],
        receiver: Receiver<'a, ButtonDirection>,
    ) -> Self {
        Self {
            col,
            active_col: 2,
            receiver,
            state: LedState::Toggle
        }
    }

    pub fn shift(&mut self, direction: ButtonDirection) {
        rprintln!("Button pressed: {:?}", direction);
        self.col[self.active_col].set_high().ok();

        self.active_col = match direction {
            ButtonDirection::Left => match self.active_col {
                0 => NUM_COLS - 1,
                _ => self.active_col - 1
            }
            ButtonDirection::Right => (self.active_col + 1) % NUM_COLS,
        };

        self.col[self.active_col].set_high().ok();
    }

    fn toggle(&mut self) {
        rprintln!("Toggle LED: {:?}", self.active_col);

        #[cfg(feature = "trigger-overflow")]
        {
            use crate::time::Ticker;
            let time = Ticker::now();
            rprintln!("Time: 0x{:x}, {} ticks", time.ticks(), time.duration_since_epoch().to_millis());
        }

        self.col[self.active_col].toggle().unwrap();
    }
}

impl OurFuture for LedTask<'_> {
    type Output = ();

    fn poll(&mut self, task_id: usize) -> Poll<Self::Output> {
        loop {
            match self.state {
                LedState::Toggle => {
                    rprintln!("Toggle LED {}", self.active_col);
                    self.toggle();
                    self.state = LedState::Wait(Timer::new(500.millis()));
                }
                LedState::Wait(ref timer) => {
                    if timer.is_ready() {
                        self.state = LedState::Toggle;
                        continue;
                    }

                    if let Some(direction) = self.receiver.receive() {
                        self.shift(direction);
                        self.state = LedState::Toggle;
                        continue;
                    }
                    break
                }
            }
        }
        Poll::Pending
    }
}