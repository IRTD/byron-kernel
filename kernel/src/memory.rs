use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::structures::paging::OffsetPageTable;
use x86_64::{
    structures::paging::{FrameAllocator, Mapper, Page, PageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

pub unsafe fn init(physical_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table_frame = active_level_4_table(physical_offset);
    OffsetPageTable::new(level_4_table_frame, physical_offset)
}

unsafe fn active_level_4_table(physical_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();
    let phys = level_4_table_frame.start_address();
    let virt = physical_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    &mut *page_table_ptr
}

pub struct DummyFrameAlloc;
unsafe impl FrameAllocator<Size4KiB> for DummyFrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub fn create_example_page(
    page: Page,
    mapper: &mut OffsetPageTable,
    frame_alloc: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;
    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_res = unsafe { mapper.map_to(page, frame, flags, frame_alloc) };
    map_to_res.expect("map_to failed").flush();
}
pub struct BootInfoFrameAlloc {
    mem_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAlloc {
    pub unsafe fn init(mem_map: &'static MemoryMap) -> Self {
        BootInfoFrameAlloc { mem_map, next: 0 }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        self.mem_map
            .iter()
            .filter(|r| r.region_type == MemoryRegionType::Usable)
            .map(|r| r.range.start_addr()..r.range.end_addr())
            .flat_map(|r| r.step_by(4096))
            .map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAlloc {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}