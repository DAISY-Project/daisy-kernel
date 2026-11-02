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

    pub unsafe fn init(&mut self) {
        outb(COM1 + 1, 0x00); // 인터럽트 비활성화
        outb(COM1 + 3, 0x80); // DLAB 활성화 (Baud rate divisor 설정)
        outb(COM1 + 0, 0x03); // Divisor 3 (38400 baud) Low byte
        outb(COM1 + 1, 0x00); // High byte
        outb(COM1 + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(COM1 + 2, 0xC7); // FIFO 활성화, 14바이트 임계값, 클리어
        outb(COM1 + 4, 0x0B); // IRQs 활성화, RTS/DSR 세팅
    }

    fn is_transmit_empty(&self) -> bool {
        unsafe { (inb(COM1 + 5) & 0x20) != 0 }
    }

    pub fn write_byte(&mut self, byte: u8) {
        while !self.is_transmit_empty() {
            core::hint::spin_loop();
        }
        unsafe {
            outb(COM1, byte);
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}