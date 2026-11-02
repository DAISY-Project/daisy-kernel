use core::mem::size_of;

#[repr(C, packed)]
struct GdtDescriptor {
    size: u16,
    offset: u64,
}

#[repr(C, align(16))]
pub struct Gdt {
    entries: [u64; 3],
}

// 읽기 전용(.rodata)이 아닌 쓰기 가능 섹션(.data)에 강제 배치
#[link_section = ".data"]
static GDT: Gdt = Gdt {
    entries: [
        0,                  // 0x00: Null
        0x00AF9B000000FFFF, // 0x08: 64-bit Kernel Code (Accessed bit = 1 로 사전 설정!)
        0x00CF93000000FFFF, // 0x10: 64-bit Kernel Data (Accessed bit = 1 로 사전 설정!)
    ],
};

pub unsafe fn init() {
    let desc = GdtDescriptor {
        size: (size_of::<Gdt>() - 1) as u16,
        offset: core::ptr::addr_of!(GDT) as u64,
    };

    // 1. GDT 로드
    core::arch::asm!(
        "lgdt [{desc}]",
        desc = in(reg) &desc,
        options(nostack, preserves_flags)
    );

    // 2. 64비트 CS 및 데이터 세그먼트 재장전
    core::arch::asm!(
        "push {sel}",
        "lea {tmp}, [2f + rip]",
        "push {tmp}",
        "retfq",
        "2:",
        "mov ds, {data_sel:x}",
        "mov es, {data_sel:x}",
        "mov fs, {data_sel:x}",
        "mov gs, {data_sel:x}",
        "mov ss, {data_sel:x}",
        sel = in(reg) 0x08_u64,
        tmp = out(reg) _,
        data_sel = in(reg) 0x10_u16,
        options(preserves_flags)
    );
}