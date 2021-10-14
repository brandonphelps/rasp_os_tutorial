
use crate::{
    bsp::device_driver::common::MMIODerefWrapper, driver,
    synchronization::NullLock,
};

use tock_registers::{
    interfaces::{ReadWriteable}, //Writeable},
    register_bitfields, register_structs,
    registers::ReadWrite,
};


// Description taken from
// came from bcm2837 files
register_bitfields! {
    u32,
    /// GPIO function select 1
    GPFSEL1 [
        /// PIN 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART RX
        ],

        /// PIN 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART TX
        ]
    ],
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => _reserved1),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0xE8 => @END),
    }
}

/// Abstraction for the associated MMIO registers.
type Registers = MMIODerefWrapper<RegisterBlock>;


// public functions
pub struct GPIOInner {
    registers: Registers,
}


impl GPIOInner {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self { 
            registers: Registers::new(mmio_start_addr),
        }
    }


    #[cfg(feature = "bsp_rpi3")]
    fn disable_pud_14_15_bcm2387(&mut self) {
        use crate::cpu;

        const DELAY: usize = 2000;

        cpu::spin_for_cycles(DELAY);

        todo!()
    }

    /// Map PL011 UART as standard out.
    /// TX to pin 14
    /// RX to pin 15
    pub fn map_pl011_uart(&mut self) {
        // Select the UART on pins 14 and 15.
        self.registers
            .GPFSEL1.modify(GPFSEL1::FSEL15::AltFunc0 + GPFSEL1::FSEL14::AltFunc0);

        // diable pull-up/down on pins 14 and 15.
        #[cfg(feature = "bsp_rpi3")]
        self.disable_pud_14_15_bcm2387();

        // todo rpi4
    }
}

pub struct GPIO {
    inner: NullLock<GPIOInner>,
}

use crate::synchronization::interface::Mutex;

impl GPIO {
    pub const unsafe fn new(mmio_start_addr: usize) -> Self {
        Self {
            inner: NullLock::new(GPIOInner::new(mmio_start_addr)),
        }
    }

    pub fn map_pl011_uart(&self) {
        self.inner.lock(|inner| inner.map_pl011_uart());
    }
}


impl driver::interface::DeviceDriver for GPIO {
    fn compatible(&self) -> &'static str  {
        "BCM GPIO"
    }
}
