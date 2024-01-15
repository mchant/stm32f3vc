#![no_std]
#![no_main]

use embassy_executor::Spawner;
use defmt::*;
use embassy_time::{Timer, Duration}; 
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{AnyPin, Input, Level, Output, Pin, Pull, Speed};
use {defmt_rtt as _, panic_probe as _};

async fn anyblink(pin: AnyPin){
    let mut led = Output::new(pin, Level::Low, Speed::Low);

    loop {
        // Timekeeping is globally available, no need to mess with hardware timers.
        led.set_high();
        Timer::after(Duration::from_millis(150)).await;
        led.set_low();
        Timer::after(Duration::from_millis(150)).await;
    }
}

// Declare async tasks
#[embassy_executor::task]
async fn blink(pin: AnyPin) {
    anyblink(pin).await;
}

#[embassy_executor::task]
async fn blink2(pin: AnyPin) {
    anyblink(pin).await;
}

// Main is itself an async task as well.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    // Spawned tasks run in the background, concurrently.
    spawner.spawn(blink(p.PE9.degrade())).unwrap();
    spawner.spawn(blink2(p.PE15.degrade())).unwrap();

    let button = Input::new(p.PA0, Pull::Down);
    let mut button = ExtiInput::new(button, p.EXTI0);
    loop {
        // Asynchronously wait for GPIO events, allowing other tasks
        // to run, or the core to sleep.
        button.wait_for_low().await;
        info!("Button pressed!");
        button.wait_for_high().await;
        info!("Button released!");
    }
}