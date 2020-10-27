#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2C Master Slave Address"]
    pub msa: MSA,
    _reserved_1_mcs: [u8; 4usize],
    #[doc = "0x08 - I2C Master Data"]
    pub mdr: MDR,
    #[doc = "0x0c - I2C Master Timer Period"]
    pub mtpr: MTPR,
    #[doc = "0x10 - I2C Master Interrupt Mask"]
    pub mimr: MIMR,
    #[doc = "0x14 - I2C Master Raw Interrupt Status"]
    pub mris: MRIS,
    #[doc = "0x18 - I2C Master Masked Interrupt Status"]
    pub mmis: MMIS,
    #[doc = "0x1c - I2C Master Interrupt Clear"]
    pub micr: MICR,
    #[doc = "0x20 - I2C Master Configuration"]
    pub mcr: MCR,
    _reserved9: [u8; 2012usize],
    #[doc = "0x800 - I2C Slave Own Address"]
    pub soar: SOAR,
    _reserved_10_scsr: [u8; 4usize],
    #[doc = "0x808 - I2C Slave Data"]
    pub sdr: SDR,
    #[doc = "0x80c - I2C Slave Interrupt Mask"]
    pub simr: SIMR,
    #[doc = "0x810 - I2C Slave Raw Interrupt Status"]
    pub sris: SRIS,
    #[doc = "0x814 - I2C Slave Masked Interrupt Status"]
    pub smis: SMIS,
    #[doc = "0x818 - I2C Slave Interrupt Clear"]
    pub sicr: SICR,
}
impl RegisterBlock {
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub fn mcs(&self) -> &MCS {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MCS) }
    }
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub fn mcs_mut(&self) -> &mut MCS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MCS) }
    }
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub fn mcs(&self) -> &MCS {
        unsafe { &*(((self as *const Self) as *const u8).add(4usize) as *const MCS) }
    }
    #[doc = "0x04 - I2C Master Control/Status"]
    #[inline(always)]
    pub fn mcs_mut(&self) -> &mut MCS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(4usize) as *mut MCS) }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub fn scsr(&self) -> &SCSR {
        unsafe { &*(((self as *const Self) as *const u8).add(2052usize) as *const SCSR) }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub fn scsr_mut(&self) -> &mut SCSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2052usize) as *mut SCSR) }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub fn scsr(&self) -> &SCSR {
        unsafe { &*(((self as *const Self) as *const u8).add(2052usize) as *const SCSR) }
    }
    #[doc = "0x804 - I2C Slave Control/Status"]
    #[inline(always)]
    pub fn scsr_mut(&self) -> &mut SCSR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(2052usize) as *mut SCSR) }
    }
}
#[doc = "I2C Master Slave Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msa](msa) module"]
pub type MSA = crate::Reg<u32, _MSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MSA;
#[doc = "`read()` method returns [msa::R](msa::R) reader structure"]
impl crate::Readable for MSA {}
#[doc = "`write(|w| ..)` method takes [msa::W](msa::W) writer structure"]
impl crate::Writable for MSA {}
#[doc = "I2C Master Slave Address"]
pub mod msa;
#[doc = "I2C Slave Own Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soar](soar) module"]
pub type SOAR = crate::Reg<u32, _SOAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SOAR;
#[doc = "`read()` method returns [soar::R](soar::R) reader structure"]
impl crate::Readable for SOAR {}
#[doc = "`write(|w| ..)` method takes [soar::W](soar::W) writer structure"]
impl crate::Writable for SOAR {}
#[doc = "I2C Slave Own Address"]
pub mod soar;
#[doc = "I2C Slave Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](scsr) module"]
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
#[doc = "`read()` method returns [scsr::R](scsr::R) reader structure"]
impl crate::Readable for SCSR {}
#[doc = "`write(|w| ..)` method takes [scsr::W](scsr::W) writer structure"]
impl crate::Writable for SCSR {}
#[doc = "I2C Slave Control/Status"]
pub mod scsr;
#[doc = "I2C Slave Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scsr](scsr) module"]
pub type SCSR = crate::Reg<u32, _SCSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SCSR;
#[doc = "`read()` method returns [scsr::R](scsr::R) reader structure"]
impl crate::Readable for SCSR {}
#[doc = "`write(|w| ..)` method takes [scsr::W](scsr::W) writer structure"]
impl crate::Writable for SCSR {}
#[doc = "I2C Slave Control/Status"]
pub mod scsr;
#[doc = "I2C Master Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcs](mcs) module"]
pub type MCS = crate::Reg<u32, _MCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCS;
#[doc = "`read()` method returns [mcs::R](mcs::R) reader structure"]
impl crate::Readable for MCS {}
#[doc = "`write(|w| ..)` method takes [mcs::W](mcs::W) writer structure"]
impl crate::Writable for MCS {}
#[doc = "I2C Master Control/Status"]
pub mod mcs;
#[doc = "I2C Master Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcs](mcs) module"]
pub type MCS = crate::Reg<u32, _MCS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCS;
#[doc = "`read()` method returns [mcs::R](mcs::R) reader structure"]
impl crate::Readable for MCS {}
#[doc = "`write(|w| ..)` method takes [mcs::W](mcs::W) writer structure"]
impl crate::Writable for MCS {}
#[doc = "I2C Master Control/Status"]
pub mod mcs;
#[doc = "I2C Slave Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdr](sdr) module"]
pub type SDR = crate::Reg<u32, _SDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDR;
#[doc = "`read()` method returns [sdr::R](sdr::R) reader structure"]
impl crate::Readable for SDR {}
#[doc = "`write(|w| ..)` method takes [sdr::W](sdr::W) writer structure"]
impl crate::Writable for SDR {}
#[doc = "I2C Slave Data"]
pub mod sdr;
#[doc = "I2C Master Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](mdr) module"]
pub type MDR = crate::Reg<u32, _MDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDR;
#[doc = "`read()` method returns [mdr::R](mdr::R) reader structure"]
impl crate::Readable for MDR {}
#[doc = "`write(|w| ..)` method takes [mdr::W](mdr::W) writer structure"]
impl crate::Writable for MDR {}
#[doc = "I2C Master Data"]
pub mod mdr;
#[doc = "I2C Master Timer Period\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtpr](mtpr) module"]
pub type MTPR = crate::Reg<u32, _MTPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTPR;
#[doc = "`read()` method returns [mtpr::R](mtpr::R) reader structure"]
impl crate::Readable for MTPR {}
#[doc = "`write(|w| ..)` method takes [mtpr::W](mtpr::W) writer structure"]
impl crate::Writable for MTPR {}
#[doc = "I2C Master Timer Period"]
pub mod mtpr;
#[doc = "I2C Slave Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [simr](simr) module"]
pub type SIMR = crate::Reg<u32, _SIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIMR;
#[doc = "`read()` method returns [simr::R](simr::R) reader structure"]
impl crate::Readable for SIMR {}
#[doc = "`write(|w| ..)` method takes [simr::W](simr::W) writer structure"]
impl crate::Writable for SIMR {}
#[doc = "I2C Slave Interrupt Mask"]
pub mod simr;
#[doc = "I2C Slave Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sris](sris) module"]
pub type SRIS = crate::Reg<u32, _SRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SRIS;
#[doc = "`read()` method returns [sris::R](sris::R) reader structure"]
impl crate::Readable for SRIS {}
#[doc = "`write(|w| ..)` method takes [sris::W](sris::W) writer structure"]
impl crate::Writable for SRIS {}
#[doc = "I2C Slave Raw Interrupt Status"]
pub mod sris;
#[doc = "I2C Master Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mimr](mimr) module"]
pub type MIMR = crate::Reg<u32, _MIMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MIMR;
#[doc = "`read()` method returns [mimr::R](mimr::R) reader structure"]
impl crate::Readable for MIMR {}
#[doc = "`write(|w| ..)` method takes [mimr::W](mimr::W) writer structure"]
impl crate::Writable for MIMR {}
#[doc = "I2C Master Interrupt Mask"]
pub mod mimr;
#[doc = "I2C Master Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mris](mris) module"]
pub type MRIS = crate::Reg<u32, _MRIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRIS;
#[doc = "`read()` method returns [mris::R](mris::R) reader structure"]
impl crate::Readable for MRIS {}
#[doc = "`write(|w| ..)` method takes [mris::W](mris::W) writer structure"]
impl crate::Writable for MRIS {}
#[doc = "I2C Master Raw Interrupt Status"]
pub mod mris;
#[doc = "I2C Slave Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smis](smis) module"]
pub type SMIS = crate::Reg<u32, _SMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SMIS;
#[doc = "`read()` method returns [smis::R](smis::R) reader structure"]
impl crate::Readable for SMIS {}
#[doc = "`write(|w| ..)` method takes [smis::W](smis::W) writer structure"]
impl crate::Writable for SMIS {}
#[doc = "I2C Slave Masked Interrupt Status"]
pub mod smis;
#[doc = "I2C Slave Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sicr](sicr) module"]
pub type SICR = crate::Reg<u32, _SICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SICR;
#[doc = "`write(|w| ..)` method takes [sicr::W](sicr::W) writer structure"]
impl crate::Writable for SICR {}
#[doc = "I2C Slave Interrupt Clear"]
pub mod sicr;
#[doc = "I2C Master Masked Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmis](mmis) module"]
pub type MMIS = crate::Reg<u32, _MMIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MMIS;
#[doc = "`read()` method returns [mmis::R](mmis::R) reader structure"]
impl crate::Readable for MMIS {}
#[doc = "`write(|w| ..)` method takes [mmis::W](mmis::W) writer structure"]
impl crate::Writable for MMIS {}
#[doc = "I2C Master Masked Interrupt Status"]
pub mod mmis;
#[doc = "I2C Master Interrupt Clear\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [micr](micr) module"]
pub type MICR = crate::Reg<u32, _MICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MICR;
#[doc = "`write(|w| ..)` method takes [micr::W](micr::W) writer structure"]
impl crate::Writable for MICR {}
#[doc = "I2C Master Interrupt Clear"]
pub mod micr;
#[doc = "I2C Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](mcr) module"]
pub type MCR = crate::Reg<u32, _MCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCR;
#[doc = "`read()` method returns [mcr::R](mcr::R) reader structure"]
impl crate::Readable for MCR {}
#[doc = "`write(|w| ..)` method takes [mcr::W](mcr::W) writer structure"]
impl crate::Writable for MCR {}
#[doc = "I2C Master Configuration"]
pub mod mcr;
