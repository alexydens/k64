#![allow(dead_code)]

use x86_64::structures::gdt::{ GlobalDescriptorTable, Descriptor };
use x86_64::structures::tss::TaskStateSegment;
use x86_64::registers::segmentation::SegmentSelector;
use x86_64::PrivilegeLevel;

pub const KERNEL_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(0, PrivilegeLevel::Ring0);
pub const KERNEL_DATA_SELECTOR: SegmentSelector = SegmentSelector::new(1, PrivilegeLevel::Ring0);
pub const USER_CODE_SELECTOR: SegmentSelector = SegmentSelector::new(2, PrivilegeLevel::Ring3);
pub const USER_DATA_SELECTOR: SegmentSelector = SegmentSelector::new(3, PrivilegeLevel::Ring3);
pub const TSS_SELECTOR: SegmentSelector = SegmentSelector::new(4, PrivilegeLevel::Ring0);

#[derive(Debug)]
pub struct Gdt {
    pub gdt: GlobalDescriptorTable,
    pub tss: TaskStateSegment,
}

impl Gdt {
    pub fn new() -> Self {
        Gdt {
            gdt: GlobalDescriptorTable::new(),
            tss: TaskStateSegment::new(),
        }
    }
    pub fn init(&mut self) {
        self.gdt.append(Descriptor::kernel_code_segment());
        self.gdt.append(Descriptor::kernel_data_segment());
        self.gdt.append(Descriptor::user_code_segment());
        self.gdt.append(Descriptor::user_data_segment());
        unsafe {
            self.gdt
                .append(Descriptor::tss_segment_unchecked(&self.tss));
            self.gdt.load_unsafe();
        }
    }
}
