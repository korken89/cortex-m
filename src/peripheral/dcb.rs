//! Debug Control Block

use volatile_register::{RW, WO};

use peripheral::DCB;

const DCB_DEMCR_TRCENA: u32 = 1 << 24;

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// Debug Halting Control and Status
    pub dhcsr: RW<u32>,
    /// Debug Core Register Selector
    pub dcrsr: WO<u32>,
    /// Debug Core Register Data
    pub dcrdr: RW<u32>,
    /// Debug Exception and Monitor Control
    pub demcr: RW<u32>,
}

impl DCB {
    /// Enables TRACE. This is for example required by the
    /// `peripheral::DWT` cycle counter to work properly.
    /// As by STM documentation, this flag is not reset on
    /// soft-reset, only on power reset.
    pub fn enable_trace(&mut self) {
        // set bit 24 / TRCENA
        unsafe { self.demcr.modify(|w| w | DCB_DEMCR_TRCENA); }
    }

    /// Disables TRACE. See `DCB::enable_trace()` for more details
    pub fn disable_trace(&mut self) {
        // unset bit 24 / TRCENA
        unsafe { self.demcr.modify(|w| w & !DCB_DEMCR_TRCENA); }
    }

    /// Is there a debugger attached? (see notes)
    ///
    /// Note 1: This function is [reported not to
    /// work](http://web.archive.org/web/20180821191012/https://community.nxp.com/thread/424925#comment-782843)
    /// on Cortex-M0 devices. Per the ARM v6-M Architecture Reference Manual, "Access to the DHCSR
    /// from software running on the processor is IMPLEMENTATION DEFINED". Indeed, from the
    /// [Cortex-M0+ r0p1 Technical Reference Manual](http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.ddi0484c/BABJHEIG.html), "Note Software cannot access the debug registers."
    ///
    /// Note 2: This function reads the DHCSR register, and therefore clears S_RESET_ST and
    /// S_RETIRE_ST.
    pub fn is_debugger_attached() -> bool {
        unsafe { (*Self::ptr()).dhcsr.read() & 0x1 == 1 }
    }
}
