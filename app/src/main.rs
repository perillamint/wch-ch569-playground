#![no_std]
#![no_main]

use ch569_hal as _;
use riscv_rt::entry;

use ch569_pac::Peripherals;
use core::fmt::{Write};

use ch569_hal::sys::GpioPort;

#[inline(never)]
fn delay() {
    for _ in 0..1_000_000 {
        unsafe { riscv::asm::nop() };
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    let sys = ch569_hal::System::new(peripherals.SYS);

    sys.safe_access(true);
    sys.set_pll_div(4);
    sys.set_pll_source(ch569_hal::sys::PllClockSource::Internal480MHz);
    sys.safe_access(false);

    sys.port_set_dir(GpioPort::PA, 8, true);

    let mut uart = ch569_hal::uart::Uart::new(peripherals.UART1, 115200, 120_000_000);
    write!(uart, "Hello Rust\r\n").unwrap();

    /*
    // Set UART1 low slope output and schmitt trigger input
    sys.port_set_smt(GpioPort::PA, 7, true);
    sys.port_set_smt(GpioPort::PA, 8, true);
    */
 
    let mut count: u64 = 0;

    // Set LED pin to output
    sys.port_set_dir(GpioPort::PA, 15, true);

    loop {
        write!(uart, "Hello Rust {}\r\n", count).unwrap();

        count += 1;

        delay();

        sys.port_out_set(GpioPort::PA, 15);

        delay();

        sys.port_out_clear(GpioPort::PA, 15);
    }
}
