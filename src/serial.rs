use core::fmt;

const COM1: u16 = 0x3F8;

#[inline]
unsafe fn outb(port: u16, val: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") val,
        options(nomem, nostack, preserves_flags)
    );
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    let val: u8;
    core::arch::asm!(
        "in al, dx",
        out("al") val,
        in("dx") port,
        options(nomem, nostack, preserves_flags)
    );
    val
}

pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        Self
    }

    pub unsafe fn init(&self) {
        outb(COM1 + 1, 0x00); // 인터럽트 비활성화
        outb(COM1 + 3, 0x80); // DLAB 활성화
        outb(COM1 + 0, 0x03); // Divisor 3 (38400 baud)
        outb(COM1 + 1, 0x00);
        outb(COM1 + 3, 0x03); // 8N1
        outb(COM1 + 2, 0xC7); // FIFO
        outb(COM1 + 4, 0x0B); // IRQs/RTS/DSR
    }

    fn is_transmit_empty(&self) -> bool {
        unsafe { (inb(COM1 + 5) & 0x20) != 0 }
    }

    pub fn write_byte(&self, byte: u8) {
        while !self.is_transmit_empty() {
            core::hint::spin_loop();
        }
        unsafe {
            outb(COM1, byte);
        }
    }

    pub fn write_string(&self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

// SerialPort 자체에 fmt::Write 구현
impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}