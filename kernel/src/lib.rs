#![no_std]
#![allow(dead_code, unused_imports)]
#![feature(abi_x86_interrupt)]
#![feature(ascii_char)]

extern crate alloc;

pub mod allocator;
pub mod byte_sizes;
pub mod cursor;
mod gdt;
pub mod interrupts;
pub mod memory;
pub mod panic_handler;
pub mod serial;
pub mod vga;
pub use x86_64;

use bootloader::bootinfo::BootInfo;
use x86_64::VirtAddr;

pub fn init(boot_info: &'static BootInfo) {
    println!();
    println!("Loading GDT...");
    gdt::init();
    println!("Loading IDT...");
    interrupts::init_idt();
    println!("Initializing PICS");
    unsafe { interrupts::PICS.lock().initialize() };
    println!("Enabling Interrupts");
    x86_64::instructions::interrupts::enable();
    println!("Creating Mapper, FrameAllocator and Heap");
    let offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(offset) };
    let mut frame_alloc = unsafe { memory::BootInfoFrameAlloc::init(&boot_info.memory_map) };
    allocator::init_heap(&mut mapper, &mut frame_alloc).expect("Failed to init heap");
    println!("\n");
    println!("[FRIDAY] :: Good Morning!\n");
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
