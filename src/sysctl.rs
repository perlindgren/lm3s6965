#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Device Identification 0"]
    pub did0: DID0,
    #[doc = "0x04 - Device Identification 1"]
    pub did1: DID1,
    #[doc = "0x08 - Device Capabilities 0"]
    pub dc0: DC0,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Device Capabilities 1"]
    pub dc1: DC1,
    #[doc = "0x14 - Device Capabilities 2"]
    pub dc2: DC2,
    #[doc = "0x18 - Device Capabilities 3"]
    pub dc3: DC3,
    #[doc = "0x1c - Device Capabilities 4"]
    pub dc4: DC4,
    _reserved7: [u8; 16usize],
    #[doc = "0x30 - Brown-Out Reset Control"]
    pub pborctl: PBORCTL,
    #[doc = "0x34 - LDO Power Control"]
    pub ldopctl: LDOPCTL,
    _reserved9: [u8; 8usize],
    #[doc = "0x40 - Software Reset Control 0"]
    pub srcr0: SRCR0,
    #[doc = "0x44 - Software Reset Control 1"]
    pub srcr1: SRCR1,
    #[doc = "0x48 - Software Reset Control 2"]
    pub srcr2: SRCR2,
    _reserved12: [u8; 4usize],
    #[doc = "0x50 - Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x54 - Interrupt Mask Control"]
    pub imc: IMC,
    #[doc = "0x58 - Masked Interrupt Status and Clear"]
    pub misc: MISC,
    #[doc = "0x5c - Reset Cause"]
    pub resc: RESC,
    #[doc = "0x60 - Run-Mode Clock Configuration"]
    pub rcc: RCC,
    #[doc = "0x64 - XTAL to PLL Translation"]
    pub pllcfg: PLLCFG,
    _reserved18: [u8; 8usize],
    #[doc = "0x70 - Run-Mode Clock Configuration 2"]
    pub rcc2: RCC2,
    _reserved19: [u8; 140usize],
    #[doc = "0x100 - Run Mode Clock Gating Control Register 0"]
    pub rcgc0: RCGC0,
    #[doc = "0x104 - Run Mode Clock Gating Control Register 1"]
    pub rcgc1: RCGC1,
    #[doc = "0x108 - Run Mode Clock Gating Control Register 2"]
    pub rcgc2: RCGC2,
    _reserved22: [u8; 4usize],
    #[doc = "0x110 - Sleep Mode Clock Gating Control Register 0"]
    pub scgc0: SCGC0,
    #[doc = "0x114 - Sleep Mode Clock Gating Control Register 1"]
    pub scgc1: SCGC1,
    #[doc = "0x118 - Sleep Mode Clock Gating Control Register 2"]
    pub scgc2: SCGC2,
    _reserved25: [u8; 4usize],
    #[doc = "0x120 - Deep Sleep Mode Clock Gating Control Register 0"]
    pub dcgc0: DCGC0,
    #[doc = "0x124 - Deep-Sleep Mode Clock Gating Control Register 1"]
    pub dcgc1: DCGC1,
    #[doc = "0x128 - Deep Sleep Mode Clock Gating Control Register 2"]
    pub dcgc2: DCGC2,
    _reserved28: [u8; 24usize],
    #[doc = "0x144 - Deep Sleep Clock Configuration"]
    pub dslpclkcfg: DSLPCLKCFG,
}
#[doc = "Device Identification 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did0](did0) module"]
pub type DID0 = crate::Reg<u32, _DID0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DID0;
#[doc = "`read()` method returns [did0::R](did0::R) reader structure"]
impl crate::Readable for DID0 {}
#[doc = "`write(|w| ..)` method takes [did0::W](did0::W) writer structure"]
impl crate::Writable for DID0 {}
#[doc = "Device Identification 0"]
pub mod did0;
#[doc = "Device Identification 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [did1](did1) module"]
pub type DID1 = crate::Reg<u32, _DID1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DID1;
#[doc = "`read()` method returns [did1::R](did1::R) reader structure"]
impl crate::Readable for DID1 {}
#[doc = "`write(|w| ..)` method takes [did1::W](did1::W) writer structure"]
impl crate::Writable for DID1 {}
#[doc = "Device Identification 1"]
pub mod did1;
#[doc = "Device Capabilities 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc0](dc0) module"]
pub type DC0 = crate::Reg<u32, _DC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC0;
#[doc = "`read()` method returns [dc0::R](dc0::R) reader structure"]
impl crate::Readable for DC0 {}
#[doc = "`write(|w| ..)` method takes [dc0::W](dc0::W) writer structure"]
impl crate::Writable for DC0 {}
#[doc = "Device Capabilities 0"]
pub mod dc0;
#[doc = "Device Capabilities 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc1](dc1) module"]
pub type DC1 = crate::Reg<u32, _DC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC1;
#[doc = "`read()` method returns [dc1::R](dc1::R) reader structure"]
impl crate::Readable for DC1 {}
#[doc = "`write(|w| ..)` method takes [dc1::W](dc1::W) writer structure"]
impl crate::Writable for DC1 {}
#[doc = "Device Capabilities 1"]
pub mod dc1;
#[doc = "Device Capabilities 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc2](dc2) module"]
pub type DC2 = crate::Reg<u32, _DC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC2;
#[doc = "`read()` method returns [dc2::R](dc2::R) reader structure"]
impl crate::Readable for DC2 {}
#[doc = "`write(|w| ..)` method takes [dc2::W](dc2::W) writer structure"]
impl crate::Writable for DC2 {}
#[doc = "Device Capabilities 2"]
pub mod dc2;
#[doc = "Device Capabilities 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc3](dc3) module"]
pub type DC3 = crate::Reg<u32, _DC3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC3;
#[doc = "`read()` method returns [dc3::R](dc3::R) reader structure"]
impl crate::Readable for DC3 {}
#[doc = "`write(|w| ..)` method takes [dc3::W](dc3::W) writer structure"]
impl crate::Writable for DC3 {}
#[doc = "Device Capabilities 3"]
pub mod dc3;
#[doc = "Device Capabilities 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dc4](dc4) module"]
pub type DC4 = crate::Reg<u32, _DC4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DC4;
#[doc = "`read()` method returns [dc4::R](dc4::R) reader structure"]
impl crate::Readable for DC4 {}
#[doc = "`write(|w| ..)` method takes [dc4::W](dc4::W) writer structure"]
impl crate::Writable for DC4 {}
#[doc = "Device Capabilities 4"]
pub mod dc4;
#[doc = "Brown-Out Reset Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pborctl](pborctl) module"]
pub type PBORCTL = crate::Reg<u32, _PBORCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBORCTL;
#[doc = "`read()` method returns [pborctl::R](pborctl::R) reader structure"]
impl crate::Readable for PBORCTL {}
#[doc = "`write(|w| ..)` method takes [pborctl::W](pborctl::W) writer structure"]
impl crate::Writable for PBORCTL {}
#[doc = "Brown-Out Reset Control"]
pub mod pborctl;
#[doc = "LDO Power Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldopctl](ldopctl) module"]
pub type LDOPCTL = crate::Reg<u32, _LDOPCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LDOPCTL;
#[doc = "`read()` method returns [ldopctl::R](ldopctl::R) reader structure"]
impl crate::Readable for LDOPCTL {}
#[doc = "`write(|w| ..)` method takes [ldopctl::W](ldopctl::W) writer structure"]
impl crate::Writable for LDOPCTL {}
#[doc = "LDO Power Control"]
pub mod ldopctl;
#[doc = "Software Reset Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcr0](srcr0) module"]
pub type SRCR0 = crate::Reg<u32, _SRCR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR0;
#[doc = "`read()` method returns [srcr0::R](srcr0::R) reader structure"]
impl crate::Readable for SRCR0 {}
#[doc = "`write(|w| ..)` method takes [srcr0::W](srcr0::W) writer structure"]
impl crate::Writable for SRCR0 {}
#[doc = "Software Reset Control 0"]
pub mod srcr0;
#[doc = "Software Reset Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcr1](srcr1) module"]
pub type SRCR1 = crate::Reg<u32, _SRCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR1;
#[doc = "`read()` method returns [srcr1::R](srcr1::R) reader structure"]
impl crate::Readable for SRCR1 {}
#[doc = "`write(|w| ..)` method takes [srcr1::W](srcr1::W) writer structure"]
impl crate::Writable for SRCR1 {}
#[doc = "Software Reset Control 1"]
pub mod srcr1;
#[doc = "Software Reset Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srcr2](srcr2) module"]
pub type SRCR2 = crate::Reg<u32, _SRCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRCR2;
#[doc = "`read()` method returns [srcr2::R](srcr2::R) reader structure"]
impl crate::Readable for SRCR2 {}
#[doc = "`write(|w| ..)` method takes [srcr2::W](srcr2::W) writer structure"]
impl crate::Writable for SRCR2 {}
#[doc = "Software Reset Control 2"]
pub mod srcr2;
#[doc = "Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "Raw Interrupt Status"]
pub mod ris;
#[doc = "Interrupt Mask Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imc](imc) module"]
pub type IMC = crate::Reg<u32, _IMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMC;
#[doc = "`read()` method returns [imc::R](imc::R) reader structure"]
impl crate::Readable for IMC {}
#[doc = "`write(|w| ..)` method takes [imc::W](imc::W) writer structure"]
impl crate::Writable for IMC {}
#[doc = "Interrupt Mask Control"]
pub mod imc;
#[doc = "Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc](misc) module"]
pub type MISC = crate::Reg<u32, _MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MISC;
#[doc = "`read()` method returns [misc::R](misc::R) reader structure"]
impl crate::Readable for MISC {}
#[doc = "`write(|w| ..)` method takes [misc::W](misc::W) writer structure"]
impl crate::Writable for MISC {}
#[doc = "Masked Interrupt Status and Clear"]
pub mod misc;
#[doc = "Reset Cause\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resc](resc) module"]
pub type RESC = crate::Reg<u32, _RESC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESC;
#[doc = "`read()` method returns [resc::R](resc::R) reader structure"]
impl crate::Readable for RESC {}
#[doc = "`write(|w| ..)` method takes [resc::W](resc::W) writer structure"]
impl crate::Writable for RESC {}
#[doc = "Reset Cause"]
pub mod resc;
#[doc = "Run-Mode Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc](rcc) module"]
pub type RCC = crate::Reg<u32, _RCC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC;
#[doc = "`read()` method returns [rcc::R](rcc::R) reader structure"]
impl crate::Readable for RCC {}
#[doc = "`write(|w| ..)` method takes [rcc::W](rcc::W) writer structure"]
impl crate::Writable for RCC {}
#[doc = "Run-Mode Clock Configuration"]
pub mod rcc;
#[doc = "XTAL to PLL Translation\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pllcfg](pllcfg) module"]
pub type PLLCFG = crate::Reg<u32, _PLLCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PLLCFG;
#[doc = "`read()` method returns [pllcfg::R](pllcfg::R) reader structure"]
impl crate::Readable for PLLCFG {}
#[doc = "`write(|w| ..)` method takes [pllcfg::W](pllcfg::W) writer structure"]
impl crate::Writable for PLLCFG {}
#[doc = "XTAL to PLL Translation"]
pub mod pllcfg;
#[doc = "Run-Mode Clock Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc2](rcc2) module"]
pub type RCC2 = crate::Reg<u32, _RCC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCC2;
#[doc = "`read()` method returns [rcc2::R](rcc2::R) reader structure"]
impl crate::Readable for RCC2 {}
#[doc = "`write(|w| ..)` method takes [rcc2::W](rcc2::W) writer structure"]
impl crate::Writable for RCC2 {}
#[doc = "Run-Mode Clock Configuration 2"]
pub mod rcc2;
#[doc = "Run Mode Clock Gating Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgc0](rcgc0) module"]
pub type RCGC0 = crate::Reg<u32, _RCGC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGC0;
#[doc = "`read()` method returns [rcgc0::R](rcgc0::R) reader structure"]
impl crate::Readable for RCGC0 {}
#[doc = "`write(|w| ..)` method takes [rcgc0::W](rcgc0::W) writer structure"]
impl crate::Writable for RCGC0 {}
#[doc = "Run Mode Clock Gating Control Register 0"]
pub mod rcgc0;
#[doc = "Run Mode Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgc1](rcgc1) module"]
pub type RCGC1 = crate::Reg<u32, _RCGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGC1;
#[doc = "`read()` method returns [rcgc1::R](rcgc1::R) reader structure"]
impl crate::Readable for RCGC1 {}
#[doc = "`write(|w| ..)` method takes [rcgc1::W](rcgc1::W) writer structure"]
impl crate::Writable for RCGC1 {}
#[doc = "Run Mode Clock Gating Control Register 1"]
pub mod rcgc1;
#[doc = "Run Mode Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcgc2](rcgc2) module"]
pub type RCGC2 = crate::Reg<u32, _RCGC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCGC2;
#[doc = "`read()` method returns [rcgc2::R](rcgc2::R) reader structure"]
impl crate::Readable for RCGC2 {}
#[doc = "`write(|w| ..)` method takes [rcgc2::W](rcgc2::W) writer structure"]
impl crate::Writable for RCGC2 {}
#[doc = "Run Mode Clock Gating Control Register 2"]
pub mod rcgc2;
#[doc = "Sleep Mode Clock Gating Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc0](scgc0) module"]
pub type SCGC0 = crate::Reg<u32, _SCGC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC0;
#[doc = "`read()` method returns [scgc0::R](scgc0::R) reader structure"]
impl crate::Readable for SCGC0 {}
#[doc = "`write(|w| ..)` method takes [scgc0::W](scgc0::W) writer structure"]
impl crate::Writable for SCGC0 {}
#[doc = "Sleep Mode Clock Gating Control Register 0"]
pub mod scgc0;
#[doc = "Sleep Mode Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc1](scgc1) module"]
pub type SCGC1 = crate::Reg<u32, _SCGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC1;
#[doc = "`read()` method returns [scgc1::R](scgc1::R) reader structure"]
impl crate::Readable for SCGC1 {}
#[doc = "`write(|w| ..)` method takes [scgc1::W](scgc1::W) writer structure"]
impl crate::Writable for SCGC1 {}
#[doc = "Sleep Mode Clock Gating Control Register 1"]
pub mod scgc1;
#[doc = "Sleep Mode Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc2](scgc2) module"]
pub type SCGC2 = crate::Reg<u32, _SCGC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCGC2;
#[doc = "`read()` method returns [scgc2::R](scgc2::R) reader structure"]
impl crate::Readable for SCGC2 {}
#[doc = "`write(|w| ..)` method takes [scgc2::W](scgc2::W) writer structure"]
impl crate::Writable for SCGC2 {}
#[doc = "Sleep Mode Clock Gating Control Register 2"]
pub mod scgc2;
#[doc = "Deep Sleep Mode Clock Gating Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgc0](dcgc0) module"]
pub type DCGC0 = crate::Reg<u32, _DCGC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGC0;
#[doc = "`read()` method returns [dcgc0::R](dcgc0::R) reader structure"]
impl crate::Readable for DCGC0 {}
#[doc = "`write(|w| ..)` method takes [dcgc0::W](dcgc0::W) writer structure"]
impl crate::Writable for DCGC0 {}
#[doc = "Deep Sleep Mode Clock Gating Control Register 0"]
pub mod dcgc0;
#[doc = "Deep-Sleep Mode Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgc1](dcgc1) module"]
pub type DCGC1 = crate::Reg<u32, _DCGC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGC1;
#[doc = "`read()` method returns [dcgc1::R](dcgc1::R) reader structure"]
impl crate::Readable for DCGC1 {}
#[doc = "`write(|w| ..)` method takes [dcgc1::W](dcgc1::W) writer structure"]
impl crate::Writable for DCGC1 {}
#[doc = "Deep-Sleep Mode Clock Gating Control Register 1"]
pub mod dcgc1;
#[doc = "Deep Sleep Mode Clock Gating Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcgc2](dcgc2) module"]
pub type DCGC2 = crate::Reg<u32, _DCGC2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCGC2;
#[doc = "`read()` method returns [dcgc2::R](dcgc2::R) reader structure"]
impl crate::Readable for DCGC2 {}
#[doc = "`write(|w| ..)` method takes [dcgc2::W](dcgc2::W) writer structure"]
impl crate::Writable for DCGC2 {}
#[doc = "Deep Sleep Mode Clock Gating Control Register 2"]
pub mod dcgc2;
#[doc = "Deep Sleep Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dslpclkcfg](dslpclkcfg) module"]
pub type DSLPCLKCFG = crate::Reg<u32, _DSLPCLKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSLPCLKCFG;
#[doc = "`read()` method returns [dslpclkcfg::R](dslpclkcfg::R) reader structure"]
impl crate::Readable for DSLPCLKCFG {}
#[doc = "`write(|w| ..)` method takes [dslpclkcfg::W](dslpclkcfg::W) writer structure"]
impl crate::Writable for DSLPCLKCFG {}
#[doc = "Deep Sleep Clock Configuration"]
pub mod dslpclkcfg;
