pub use bare_metal::{CriticalSection, Mutex, Nr};

#[inline]
pub fn disable() {
    unsafe {
        asm!(
            "cli" :::: "volatile"
        );
    }
}

#[inline]
pub fn enable() {
    unsafe {
        asm!(
            "sei" :::: "volatile"
        );
    }
}

/// Execute closure `f` in an interrupt-free context.
///
/// This as also known as a "critical section".
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce(&CriticalSection) -> R,
{
    let sreg: u8;

    // Store current state
    unsafe {
        asm!(
            "in $0,0x35"
            : "=r"(sreg)
            :
            :
            : "volatile"
        );
    }

    // Disable interrupts
    disable();

    let r = f(unsafe { &CriticalSection::new() });

    // Restore interrupt state
    if sreg & 0x80 != 0x00 {
        enable();
    }

    r
}
