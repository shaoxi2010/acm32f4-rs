#![no_std]
#![no_main]

// pick a panicking behavior
use core::panic::PanicInfo;
use rtt_target::{rprintln, rtt_init_print};
use cortex_m_rt::entry;
use cortex_m::{asm::nop, delay::Delay};
use acm32f40x::{CorePeripherals, Peripherals};
use cortex_m::peripheral::syst;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let pd = Peripherals::take().unwrap();
    let cd = CorePeripherals::take().unwrap();
    let mut delay = Delay::with_source(cd.SYST, 16_000_000, syst::SystClkSource::Core);

    pd.SCU.ipckenr2.write(|w| unsafe{ w.bits(1 << 12)});
    rprintln!("ipckenr2 {:x}", pd.SCU.ipckenr2.read().bits());
    delay.delay_ms(1000);
    pd.GPIO3.gpio_dir.write(|w| unsafe { w.bits(1 << 19) });
    rprintln!("dir: {:08x}", pd.GPIO3.gpio_dir.read().bits());
    delay.delay_ms(1000);
    loop {
        rprintln!("blink led!");

        for i in 0..1_000_000 {
            delay.delay_ms(500);
            rprintln!("Led invert: {} ", i);
            pd.GPIO3.gpio_odata.write(|w| unsafe { w.bits(i%2 << 19) });
            rprintln!("odata: {:08x}", pd.GPIO3.gpio_odata.read().bits());
        }

        panic!("blink led stop(intentional panic).");
    }
}

#[inline(never)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rprintln!("{}", info);
    loop {} // You might need a compiler fence in here.
}