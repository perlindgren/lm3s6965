#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Memory Address"]
    pub fma: FMA,
    #[doc = "0x04 - Flash Memory Data"]
    pub fmd: FMD,
    #[doc = "0x08 - Flash Memory Control"]
    pub fmc: FMC,
    #[doc = "0x0c - Flash Controller Raw Interrupt Status"]
    pub fcris: FCRIS,
    #[doc = "0x10 - Flash Controller Interrupt Mask"]
    pub fcim: FCIM,
    #[doc = "0x14 - Flash Controller Masked Interrupt Status and Clear"]
    pub fcmisc: FCMISC,
    _reserved6: [u8; 4392usize],
    #[doc = "0x1140 - USec Reload"]
    pub usecrl: USECRL,
    _reserved7: [u8; 140usize],
    #[doc = "0x11d0 - User Debug"]
    pub userdbg: USERDBG,
    _reserved8: [u8; 12usize],
    #[doc = "0x11e0 - User Register 0"]
    pub userreg0: USERREG0,
    #[doc = "0x11e4 - User Register 1"]
    pub userreg1: USERREG1,
    _reserved10: [u8; 24usize],
    #[doc = "0x1200 - Flash Memory Protection Read Enable 0"]
    pub fmpre0: FMPRE0,
    #[doc = "0x1204 - Flash Memory Protection Read Enable 1"]
    pub fmpre1: FMPRE1,
    #[doc = "0x1208 - Flash Memory Protection Read Enable 2"]
    pub fmpre2: FMPRE2,
    #[doc = "0x120c - Flash Memory Protection Read Enable 3"]
    pub fmpre3: FMPRE3,
    _reserved14: [u8; 496usize],
    #[doc = "0x1400 - Flash Memory Protection Program Enable 0"]
    pub fmppe0: FMPPE0,
    #[doc = "0x1404 - Flash Memory Protection Program Enable 1"]
    pub fmppe1: FMPPE1,
    #[doc = "0x1408 - Flash Memory Protection Program Enable 2"]
    pub fmppe2: FMPPE2,
    #[doc = "0x140c - Flash Memory Protection Program Enable 3"]
    pub fmppe3: FMPPE3,
}
#[doc = "Flash Memory Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fma](fma) module"]
pub type FMA = crate::Reg<u32, _FMA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMA;
#[doc = "`read()` method returns [fma::R](fma::R) reader structure"]
impl crate::Readable for FMA {}
#[doc = "`write(|w| ..)` method takes [fma::W](fma::W) writer structure"]
impl crate::Writable for FMA {}
#[doc = "Flash Memory Address"]
pub mod fma;
#[doc = "Flash Memory Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmd](fmd) module"]
pub type FMD = crate::Reg<u32, _FMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMD;
#[doc = "`read()` method returns [fmd::R](fmd::R) reader structure"]
impl crate::Readable for FMD {}
#[doc = "`write(|w| ..)` method takes [fmd::W](fmd::W) writer structure"]
impl crate::Writable for FMD {}
#[doc = "Flash Memory Data"]
pub mod fmd;
#[doc = "Flash Memory Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc](fmc) module"]
pub type FMC = crate::Reg<u32, _FMC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMC;
#[doc = "`read()` method returns [fmc::R](fmc::R) reader structure"]
impl crate::Readable for FMC {}
#[doc = "`write(|w| ..)` method takes [fmc::W](fmc::W) writer structure"]
impl crate::Writable for FMC {}
#[doc = "Flash Memory Control"]
pub mod fmc;
#[doc = "Flash Controller Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcris](fcris) module"]
pub type FCRIS = crate::Reg<u32, _FCRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCRIS;
#[doc = "`read()` method returns [fcris::R](fcris::R) reader structure"]
impl crate::Readable for FCRIS {}
#[doc = "`write(|w| ..)` method takes [fcris::W](fcris::W) writer structure"]
impl crate::Writable for FCRIS {}
#[doc = "Flash Controller Raw Interrupt Status"]
pub mod fcris;
#[doc = "Flash Controller Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcim](fcim) module"]
pub type FCIM = crate::Reg<u32, _FCIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCIM;
#[doc = "`read()` method returns [fcim::R](fcim::R) reader structure"]
impl crate::Readable for FCIM {}
#[doc = "`write(|w| ..)` method takes [fcim::W](fcim::W) writer structure"]
impl crate::Writable for FCIM {}
#[doc = "Flash Controller Interrupt Mask"]
pub mod fcim;
#[doc = "Flash Controller Masked Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcmisc](fcmisc) module"]
pub type FCMISC = crate::Reg<u32, _FCMISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCMISC;
#[doc = "`read()` method returns [fcmisc::R](fcmisc::R) reader structure"]
impl crate::Readable for FCMISC {}
#[doc = "`write(|w| ..)` method takes [fcmisc::W](fcmisc::W) writer structure"]
impl crate::Writable for FCMISC {}
#[doc = "Flash Controller Masked Interrupt Status and Clear"]
pub mod fcmisc;
#[doc = "USec Reload\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usecrl](usecrl) module"]
pub type USECRL = crate::Reg<u32, _USECRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USECRL;
#[doc = "`read()` method returns [usecrl::R](usecrl::R) reader structure"]
impl crate::Readable for USECRL {}
#[doc = "`write(|w| ..)` method takes [usecrl::W](usecrl::W) writer structure"]
impl crate::Writable for USECRL {}
#[doc = "USec Reload"]
pub mod usecrl;
#[doc = "User Debug\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userdbg](userdbg) module"]
pub type USERDBG = crate::Reg<u32, _USERDBG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERDBG;
#[doc = "`read()` method returns [userdbg::R](userdbg::R) reader structure"]
impl crate::Readable for USERDBG {}
#[doc = "`write(|w| ..)` method takes [userdbg::W](userdbg::W) writer structure"]
impl crate::Writable for USERDBG {}
#[doc = "User Debug"]
pub mod userdbg;
#[doc = "User Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg0](userreg0) module"]
pub type USERREG0 = crate::Reg<u32, _USERREG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERREG0;
#[doc = "`read()` method returns [userreg0::R](userreg0::R) reader structure"]
impl crate::Readable for USERREG0 {}
#[doc = "`write(|w| ..)` method takes [userreg0::W](userreg0::W) writer structure"]
impl crate::Writable for USERREG0 {}
#[doc = "User Register 0"]
pub mod userreg0;
#[doc = "User Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [userreg1](userreg1) module"]
pub type USERREG1 = crate::Reg<u32, _USERREG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USERREG1;
#[doc = "`read()` method returns [userreg1::R](userreg1::R) reader structure"]
impl crate::Readable for USERREG1 {}
#[doc = "`write(|w| ..)` method takes [userreg1::W](userreg1::W) writer structure"]
impl crate::Writable for USERREG1 {}
#[doc = "User Register 1"]
pub mod userreg1;
#[doc = "Flash Memory Protection Read Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre0](fmpre0) module"]
pub type FMPRE0 = crate::Reg<u32, _FMPRE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE0;
#[doc = "`read()` method returns [fmpre0::R](fmpre0::R) reader structure"]
impl crate::Readable for FMPRE0 {}
#[doc = "`write(|w| ..)` method takes [fmpre0::W](fmpre0::W) writer structure"]
impl crate::Writable for FMPRE0 {}
#[doc = "Flash Memory Protection Read Enable 0"]
pub mod fmpre0;
#[doc = "Flash Memory Protection Read Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre1](fmpre1) module"]
pub type FMPRE1 = crate::Reg<u32, _FMPRE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE1;
#[doc = "`read()` method returns [fmpre1::R](fmpre1::R) reader structure"]
impl crate::Readable for FMPRE1 {}
#[doc = "`write(|w| ..)` method takes [fmpre1::W](fmpre1::W) writer structure"]
impl crate::Writable for FMPRE1 {}
#[doc = "Flash Memory Protection Read Enable 1"]
pub mod fmpre1;
#[doc = "Flash Memory Protection Read Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre2](fmpre2) module"]
pub type FMPRE2 = crate::Reg<u32, _FMPRE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE2;
#[doc = "`read()` method returns [fmpre2::R](fmpre2::R) reader structure"]
impl crate::Readable for FMPRE2 {}
#[doc = "`write(|w| ..)` method takes [fmpre2::W](fmpre2::W) writer structure"]
impl crate::Writable for FMPRE2 {}
#[doc = "Flash Memory Protection Read Enable 2"]
pub mod fmpre2;
#[doc = "Flash Memory Protection Read Enable 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmpre3](fmpre3) module"]
pub type FMPRE3 = crate::Reg<u32, _FMPRE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPRE3;
#[doc = "`read()` method returns [fmpre3::R](fmpre3::R) reader structure"]
impl crate::Readable for FMPRE3 {}
#[doc = "`write(|w| ..)` method takes [fmpre3::W](fmpre3::W) writer structure"]
impl crate::Writable for FMPRE3 {}
#[doc = "Flash Memory Protection Read Enable 3"]
pub mod fmpre3;
#[doc = "Flash Memory Protection Program Enable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe0](fmppe0) module"]
pub type FMPPE0 = crate::Reg<u32, _FMPPE0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE0;
#[doc = "`read()` method returns [fmppe0::R](fmppe0::R) reader structure"]
impl crate::Readable for FMPPE0 {}
#[doc = "`write(|w| ..)` method takes [fmppe0::W](fmppe0::W) writer structure"]
impl crate::Writable for FMPPE0 {}
#[doc = "Flash Memory Protection Program Enable 0"]
pub mod fmppe0;
#[doc = "Flash Memory Protection Program Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe1](fmppe1) module"]
pub type FMPPE1 = crate::Reg<u32, _FMPPE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE1;
#[doc = "`read()` method returns [fmppe1::R](fmppe1::R) reader structure"]
impl crate::Readable for FMPPE1 {}
#[doc = "`write(|w| ..)` method takes [fmppe1::W](fmppe1::W) writer structure"]
impl crate::Writable for FMPPE1 {}
#[doc = "Flash Memory Protection Program Enable 1"]
pub mod fmppe1;
#[doc = "Flash Memory Protection Program Enable 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe2](fmppe2) module"]
pub type FMPPE2 = crate::Reg<u32, _FMPPE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE2;
#[doc = "`read()` method returns [fmppe2::R](fmppe2::R) reader structure"]
impl crate::Readable for FMPPE2 {}
#[doc = "`write(|w| ..)` method takes [fmppe2::W](fmppe2::W) writer structure"]
impl crate::Writable for FMPPE2 {}
#[doc = "Flash Memory Protection Program Enable 2"]
pub mod fmppe2;
#[doc = "Flash Memory Protection Program Enable 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmppe3](fmppe3) module"]
pub type FMPPE3 = crate::Reg<u32, _FMPPE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FMPPE3;
#[doc = "`read()` method returns [fmppe3::R](fmppe3::R) reader structure"]
impl crate::Readable for FMPPE3 {}
#[doc = "`write(|w| ..)` method takes [fmppe3::W](fmppe3::W) writer structure"]
impl crate::Writable for FMPPE3 {}
#[doc = "Flash Memory Protection Program Enable 3"]
pub mod fmppe3;
