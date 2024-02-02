#![no_std]
#![no_main]

extern crate alloc;
use alloc::boxed::Box;
use kernel::*;

use bootloader::BootInfo;

bootloader::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // <> Kernel Init $ DO NOT REMOVE <> //
    kernel::init(boot_info);
    ///////////////////////////////////////

    let x = Box::new(32);
    println!("Boxed val {x}");

    // <> Kernel End $ Should never occur in actual use <> //
    kernel::hlt_loop()
    /////////////////////////////////////////////////////////
}
