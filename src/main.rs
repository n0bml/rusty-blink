#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut onboard = pins.d13.into_output();
    onboard.set_low();

    let mut led_red = pins.d12.into_output();
    led_red.set_high();

    let mut led_green = pins.d11.into_output();
    led_green.set_high();

    let mut led_blue = pins.d10.into_output();
    led_blue.set_high();

    let mut counter: u64 = 0;

    loop {
        counter += 1;
        if (counter % 2) == 0 {
            led_red.toggle();
        }

        if (counter % 3) == 0 {
            led_green.toggle();
        }

        if (counter % 4) == 0 {
            led_blue.toggle();
        }

        onboard.toggle();
        arduino_hal::delay_ms(500);
    }
}
