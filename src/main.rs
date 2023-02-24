#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, pulse_control::ClockSource, 
    utils::{smartLedAdapter, SmartLedsAdapter},
    timer::TimerGroup, Delay, Rtc, PulseControl, IO,};

use smart_leds::{
    brightness,
    gamma,
    RGB8,
    SmartLedsWrite,
};

const NUM_LEDS: usize = 1;
#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    // Configure RMT peripheral globally
    let pulse = PulseControl::new(
        peripherals.RMT,
        &mut system.peripheral_clock_control,
        ClockSource::APB,
        0,
        0,
        0,
    )
    .unwrap();

// We use one of the RMT channels to instantiate a `SmartLedsAdapter` which can
    // be used directly with all `smart_led` implementations
    let mut led = <smartLedAdapter!(1)>::new(pulse.channel0, io.pins.gpio48);

    
    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut delay = Delay::new(&clocks);

    let mut red = 255;
    let mut green = 0;
    let mut blue = 0;
    let mut color = RGB8{r:red,g:green,b:blue};
    let mut data: [RGB8; NUM_LEDS] = [color; NUM_LEDS];
    let color_empty = RGB8{r:0,g:0,b:0}.clone();
    let empty: [RGB8; NUM_LEDS] = [color_empty; NUM_LEDS];

    println!("Hello world!");
    let bright_val = 20;
    let delay_val:u32 = 200;

    loop {
        led.write(brightness(gamma(data.iter().cloned()), bright_val))
                .unwrap();
        delay.delay_ms(delay_val);
        println!("Done 1");
        led.write(brightness(gamma(empty.iter().cloned()), bright_val))
                .unwrap();
        delay.delay_ms(delay_val);
        println!("Done 2");

        
        red = 0;
        green = 255;
        blue = 0;
        color = RGB8{r:red,g:green,b:blue};
        data = [color; NUM_LEDS];
        led.write(brightness(gamma(data.iter().cloned()), bright_val))
                .unwrap();
        delay.delay_ms(delay_val);
        println!("Done 1");
        led.write(brightness(gamma(empty.iter().cloned()), bright_val))
                .unwrap();
        delay.delay_ms(delay_val);

        
        red = 0;  
        green = 0;
        blue = 255;
        color = RGB8{r:red,g:green,b:blue};
        data = [color; NUM_LEDS];
        led.write(brightness(gamma(data.iter().cloned()), bright_val))
                .unwrap();
        delay.delay_ms(delay_val);
        println!("Done 1");
        led.write(brightness(gamma(empty.iter().cloned()), bright_val))
                .unwrap();
        delay.delay_ms(delay_val);

        red = 255; 
        green = 0;
        blue = 0;
        color = RGB8{r:red,g:green,b:blue};
        data = [color; NUM_LEDS];
    }
}
