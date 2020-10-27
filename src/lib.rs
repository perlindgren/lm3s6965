#![doc = "Peripheral access API for LM3S6965 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
// extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
    fn nr(&self) -> u8 {
        match *self {}
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Register map for WATCHDOG0 peripheral"]
pub struct WATCHDOG0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WATCHDOG0 {}
impl WATCHDOG0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const watchdog0::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for WATCHDOG0 {
    type Target = watchdog0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WATCHDOG0::ptr() }
    }
}
#[doc = "Register map for WATCHDOG0 peripheral"]
pub mod watchdog0;
#[doc = "Register map for GPIO_PORTA peripheral"]
pub struct GPIO_PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTA {}
impl GPIO_PORTA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for GPIO_PORTA {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTA::ptr() }
    }
}
#[doc = "Register map for GPIO_PORTA peripheral"]
pub mod gpio_porta;
#[doc = "Register map for GPIO_PORTA peripheral"]
pub struct GPIO_PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTB {}
impl GPIO_PORTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_5000 as *const _
    }
}
impl Deref for GPIO_PORTB {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTB::ptr() }
    }
}
#[doc = "Register map for GPIO_PORTA peripheral"]
pub struct GPIO_PORTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTC {}
impl GPIO_PORTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_6000 as *const _
    }
}
impl Deref for GPIO_PORTC {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTC::ptr() }
    }
}
#[doc = "Register map for GPIO_PORTA peripheral"]
pub struct GPIO_PORTD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTD {}
impl GPIO_PORTD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4000_7000 as *const _
    }
}
impl Deref for GPIO_PORTD {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTD::ptr() }
    }
}
#[doc = "Register map for SSI0 peripheral"]
pub struct SSI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSI0 {}
impl SSI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssi0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SSI0 {
    type Target = ssi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSI0::ptr() }
    }
}
#[doc = "Register map for SSI0 peripheral"]
pub mod ssi0;
#[doc = "Register map for UART0 peripheral"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub mod uart0;
#[doc = "Register map for UART0 peripheral"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_d000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Register map for UART0 peripheral"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_e000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
// #[doc = "Register map for I2C0 peripheral"]
// pub struct I2C0 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for I2C0 {}
// impl I2C0 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const i2c0::RegisterBlock {
//         0x4002_0000 as *const _
//     }
// }
// impl Deref for I2C0 {
//     type Target = i2c0::RegisterBlock;
//     #[inline(always)]
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*I2C0::ptr() }
//     }
// }
// #[doc = "Register map for I2C0 peripheral"]
// pub mod i2c0;
// #[doc = "Register map for I2C0 peripheral"]
// pub struct I2C1 {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for I2C1 {}
// impl I2C1 {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const i2c0::RegisterBlock {
//         0x4002_1000 as *const _
//     }
// }
// impl Deref for I2C1 {
//     type Target = i2c0::RegisterBlock;
//     #[inline(always)]
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*I2C1::ptr() }
//     }
// }
#[doc = "Register map for GPIO_PORTA peripheral"]
pub struct GPIO_PORTE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTE {}
impl GPIO_PORTE {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for GPIO_PORTE {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTE::ptr() }
    }
}
#[doc = "Register map for GPIO_PORTA peripheral"]
pub struct GPIO_PORTF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTF {}
impl GPIO_PORTF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4002_5000 as *const _
    }
}
impl Deref for GPIO_PORTF {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTF::ptr() }
    }
}
#[doc = "Register map for GPIO_PORTA peripheral"]
pub struct GPIO_PORTG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO_PORTG {}
impl GPIO_PORTG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio_porta::RegisterBlock {
        0x4002_6000 as *const _
    }
}
impl Deref for GPIO_PORTG {
    type Target = gpio_porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO_PORTG::ptr() }
    }
}
#[doc = "Register map for PWM0 peripheral"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "Register map for PWM0 peripheral"]
pub mod pwm0;
#[doc = "Register map for QEI0 peripheral"]
pub struct QEI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI0 {}
impl QEI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qei0::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for QEI0 {
    type Target = qei0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QEI0::ptr() }
    }
}
#[doc = "Register map for QEI0 peripheral"]
pub mod qei0;
#[doc = "Register map for QEI0 peripheral"]
pub struct QEI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI1 {}
impl QEI1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qei0::RegisterBlock {
        0x4002_d000 as *const _
    }
}
impl Deref for QEI1 {
    type Target = qei0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QEI1::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub mod timer0;
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_1000 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_2000 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Register map for TIMER0 peripheral"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4003_3000 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "Register map for ADC0 peripheral"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Register map for ADC0 peripheral"]
pub mod adc0;
#[doc = "Register map for COMP peripheral"]
pub struct COMP {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for COMP {}
impl COMP {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const comp::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for COMP {
    type Target = comp::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*COMP::ptr() }
    }
}
#[doc = "Register map for COMP peripheral"]
pub mod comp;
// #[doc = "Register map for MAC peripheral"]
// pub struct MAC {
//     _marker: PhantomData<*const ()>,
// }
// unsafe impl Send for MAC {}
// impl MAC {
//     #[doc = r"Returns a pointer to the register block"]
//     #[inline(always)]
//     pub const fn ptr() -> *const mac::RegisterBlock {
//         0x4004_8000 as *const _
//     }
// }
// impl Deref for MAC {
//     type Target = mac::RegisterBlock;
//     #[inline(always)]
//     fn deref(&self) -> &Self::Target {
//         unsafe { &*MAC::ptr() }
//     }
// }
// #[doc = "Register map for MAC peripheral"]
// pub mod mac;
#[doc = "Register map for HIB peripheral"]
pub struct HIB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HIB {}
impl HIB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hib::RegisterBlock {
        0x400f_c000 as *const _
    }
}
impl Deref for HIB {
    type Target = hib::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*HIB::ptr() }
    }
}
#[doc = "Register map for HIB peripheral"]
pub mod hib;
#[doc = "Register map for FLASH_CTRL peripheral"]
pub struct FLASH_CTRL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FLASH_CTRL {}
impl FLASH_CTRL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const flash_ctrl::RegisterBlock {
        0x400f_d000 as *const _
    }
}
impl Deref for FLASH_CTRL {
    type Target = flash_ctrl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FLASH_CTRL::ptr() }
    }
}
#[doc = "Register map for FLASH_CTRL peripheral"]
pub mod flash_ctrl;
#[doc = "Register map for SYSCTL peripheral"]
pub struct SYSCTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCTL {}
impl SYSCTL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sysctl::RegisterBlock {
        0x400f_e000 as *const _
    }
}
impl Deref for SYSCTL {
    type Target = sysctl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCTL::ptr() }
    }
}
#[doc = "Register map for SYSCTL peripheral"]
pub mod sysctl;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WATCHDOG0"]
    pub WATCHDOG0: WATCHDOG0,
    #[doc = "GPIO_PORTA"]
    pub GPIO_PORTA: GPIO_PORTA,
    #[doc = "GPIO_PORTB"]
    pub GPIO_PORTB: GPIO_PORTB,
    #[doc = "GPIO_PORTC"]
    pub GPIO_PORTC: GPIO_PORTC,
    #[doc = "GPIO_PORTD"]
    pub GPIO_PORTD: GPIO_PORTD,
    #[doc = "SSI0"]
    pub SSI0: SSI0,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    // #[doc = "I2C0"]
    // pub I2C0: I2C0,
    // #[doc = "I2C1"]
    // pub I2C1: I2C1,
    #[doc = "GPIO_PORTE"]
    pub GPIO_PORTE: GPIO_PORTE,
    #[doc = "GPIO_PORTF"]
    pub GPIO_PORTF: GPIO_PORTF,
    #[doc = "GPIO_PORTG"]
    pub GPIO_PORTG: GPIO_PORTG,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "QEI0"]
    pub QEI0: QEI0,
    #[doc = "QEI1"]
    pub QEI1: QEI1,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "COMP"]
    pub COMP: COMP,
    // #[doc = "MAC"]
    // pub MAC: MAC,
    #[doc = "HIB"]
    pub HIB: HIB,
    #[doc = "FLASH_CTRL"]
    pub FLASH_CTRL: FLASH_CTRL,
    #[doc = "SYSCTL"]
    pub SYSCTL: SYSCTL,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            WATCHDOG0: WATCHDOG0 {
                _marker: PhantomData,
            },
            GPIO_PORTA: GPIO_PORTA {
                _marker: PhantomData,
            },
            GPIO_PORTB: GPIO_PORTB {
                _marker: PhantomData,
            },
            GPIO_PORTC: GPIO_PORTC {
                _marker: PhantomData,
            },
            GPIO_PORTD: GPIO_PORTD {
                _marker: PhantomData,
            },
            SSI0: SSI0 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            // I2C0: I2C0 {
            //     _marker: PhantomData,
            // },
            // I2C1: I2C1 {
            //     _marker: PhantomData,
            // },
            GPIO_PORTE: GPIO_PORTE {
                _marker: PhantomData,
            },
            GPIO_PORTF: GPIO_PORTF {
                _marker: PhantomData,
            },
            GPIO_PORTG: GPIO_PORTG {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            QEI0: QEI0 {
                _marker: PhantomData,
            },
            QEI1: QEI1 {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            COMP: COMP {
                _marker: PhantomData,
            },
            // MAC: MAC {
            //     _marker: PhantomData,
            // },
            HIB: HIB {
                _marker: PhantomData,
            },
            FLASH_CTRL: FLASH_CTRL {
                _marker: PhantomData,
            },
            SYSCTL: SYSCTL {
                _marker: PhantomData,
            },
        }
    }
}
