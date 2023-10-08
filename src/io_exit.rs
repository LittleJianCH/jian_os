// iobase=0xf4, iosize=0x04

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed  = 0x11,
}


pub fn exit_qemu(exit_code: QemuExitCode) {
    use core::arch::asm;

    unsafe {
        asm!(
            "out 0xf4, eax",
            in("eax") exit_code as u32,
        );
    }
}

#[macro_export]
macro_rules! exit {
    ($exit_code:ident) => {
        let exit_code = $crate::io_exit::QemuExitCode::$exit_code;
        $crate::io_exit::exit_qemu(exit_code);
    };
}