#![no_std]
#![no_main]
mod accelertate;

use core::ops::Add;
// pick a panicking behavior
use accelertate::enable_icache;
use acm32f40x::{CorePeripherals, Peripherals};
use core::panic::PanicInfo;
use cortex_m::peripheral::syst;
use cortex_m::{asm::nop, delay::Delay};
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

fn calc_eflash_checksum() -> u32 {
    use core::num::Wrapping;
    let mut checksum = Wrapping(0u32);
    for _ in 0..500 {
        let mut data = 0 as *const u32;
        for _ in 0..1024 * 32 {
            use core::ptr::read_volatile;
            checksum = checksum.add(unsafe { Wrapping(read_volatile(data)) });
            unsafe {
                data = data.add(4);
            }
        }
    }
    checksum.0
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let pd = Peripherals::take().unwrap();
    let cd = CorePeripherals::take().unwrap();
    let mut delay = Delay::with_source(cd.SYST, 16_000_000, syst::SystClkSource::Core);

    rprintln!("checksum code without cache :start");
    rprintln!(
        "checksum code without cache:end {:x}",
        calc_eflash_checksum()
    );
    enable_icache();
    rprintln!("checksum code with cache :start");
    rprintln!("checksum code with cache:end {:x}", calc_eflash_checksum());

    pd.SCU.ipckenr2.write(|w| w.gpio3().set_bit());
    delay.delay_ms(1000);
    pd.GPIO3.gpio_dir.write(|w| w.ph3().set_bit());
    delay.delay_ms(1000);
    loop {
        rprintln!("blink led!");
        for i in 0..1_000_000 {
            delay.delay_ms(500);
            if i % 2 == 0 {
                pd.GPIO3.gpio_set.write(|w| w.ph3().set_bit())
            } else {
                pd.GPIO3.gpio_clr.write(|w| w.ph3().set_bit())
            }
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
