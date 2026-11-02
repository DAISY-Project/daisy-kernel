use core::mem::size_of;

// CPU가 인터럽트 발생 시 스택에 자동으로 쌓아주는 기본 프레임
#[repr(C)]
#[derive(Debug)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64, // 예외가 터진 당시의 RIP
    pub code_segment: u64,        // CS
    pub cpu_flags: u64,           // RFLAGS
    pub stack_pointer: u64,       // RSP
    pub stack_segment: u64,       // SS
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist_and_types: u16,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub const fn missing() -> Self {
        Self {
            offset_low: 0,
            selector: 0,
            ist_and_types: 0,
            offset_mid: 0,
            offset_high: 0,
            reserved: 0,
        }
    }

    pub fn set_handler(&mut self, handler: usize) {
        self.offset_low = handler as u16;
        self.selector = 0x08; // 64비트 커널 코드 세그먼트
        self.ist_and_types = 0x8E00; // Present=1, DPL=0, Type=Interrupt Gate (0xE)
        self.offset_mid = (handler >> 16) as u16;
        self.offset_high = (handler >> 32) as u32;
        self.reserved = 0;
    }
}

#[repr(C, packed)]
struct IdtDescriptor {
    size: u16,
    offset: u64,
}

#[repr(C, align(16))]
pub struct Idt {
    entries: [IdtEntry; 256],
}

#[link_section = ".data"]
static mut IDT: Idt = Idt {
    entries: [IdtEntry::missing(); 256],
};

pub unsafe fn init() {
    IDT.entries[3].set_handler(breakpoint_handler as usize);
    IDT.entries[8].set_handler(double_fault_handler as usize);
    IDT.entries[13].set_handler(general_protection_fault_handler as usize);
    IDT.entries[14].set_handler(page_fault_handler as usize);

    let desc = IdtDescriptor {
        size: (size_of::<Idt>() - 1) as u16,
        offset: core::ptr::addr_of!(IDT) as u64,
    };

    core::arch::asm!(
        "lidt [{desc}]",
        desc = in(reg) &desc,
        options(nostack, preserves_flags)
    );
}

// --- 예외 핸들러 (올바른 시그니처) ---

// Vector 3: Breakpoint (에러 코드 없음 -> 매개변수 1개)
extern "x86-interrupt" fn breakpoint_handler(frame: InterruptStackFrame) {
    crate::serial_println!("[DAISY CPU EXCEPTION] Breakpoint Trap (#BP) Hit!");
    crate::serial_println!("  RIP: {:#x}", frame.instruction_pointer);
}

// Vector 8: Double Fault (에러 코드 있음 -> 매개변수 2개)
extern "x86-interrupt" fn double_fault_handler(frame: InterruptStackFrame, error_code: u64) -> ! {
    crate::serial_println!("[DAISY CRITICAL FAULT] Double Fault (#DF)!");
    crate::serial_println!("  Error Code: {:#x}", error_code);
    crate::serial_println!("  RIP: {:#x}", frame.instruction_pointer);
    loop {
        core::hint::spin_loop();
    }
}

// Vector 13: General Protection Fault (에러 코드 있음 -> 매개변수 2개)
extern "x86-interrupt" fn general_protection_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    crate::serial_println!("[DAISY CPU EXCEPTION] General Protection Fault (#GP)!");
    crate::serial_println!("  Error Code: {:#x}", error_code);
    crate::serial_println!("  RIP: {:#x}", frame.instruction_pointer);
    loop {
        core::hint::spin_loop();
    }
}

// Vector 14: Page Fault (에러 코드 있음 -> 매개변수 2개)
extern "x86-interrupt" fn page_fault_handler(frame: InterruptStackFrame, error_code: u64) {
    let faulting_address: u64;
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) faulting_address, options(nomem, nostack, preserves_flags));
    }
    crate::serial_println!("[DAISY CPU EXCEPTION] Page Fault (#PF)!");
    crate::serial_println!("  Accessed Address: {:#x}", faulting_address);
    crate::serial_println!("  Error Code: {:#x}", error_code);
    crate::serial_println!("  RIP: {:#x}", frame.instruction_pointer);
    loop {
        core::hint::spin_loop();
    }
}