#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC Active Sample Sequencer"]
    pub actss: ACTSS,
    #[doc = "0x04 - ADC Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x08 - ADC Interrupt Mask"]
    pub im: IM,
    #[doc = "0x0c - ADC Interrupt Status and Clear"]
    pub isc: ISC,
    #[doc = "0x10 - ADC Overflow Status"]
    pub ostat: OSTAT,
    #[doc = "0x14 - ADC Event Multiplexer Select"]
    pub emux: EMUX,
    #[doc = "0x18 - ADC Underflow Status"]
    pub ustat: USTAT,
    _reserved7: [u8; 4usize],
    #[doc = "0x20 - ADC Sample Sequencer Priority"]
    pub sspri: SSPRI,
    _reserved8: [u8; 4usize],
    #[doc = "0x28 - ADC Processor Sample Sequence Initiate"]
    pub pssi: PSSI,
    _reserved9: [u8; 4usize],
    #[doc = "0x30 - ADC Sample Averaging Control"]
    pub sac: SAC,
    _reserved10: [u8; 12usize],
    #[doc = "0x40 - ADC Sample Sequence Input Multiplexer Select 0"]
    pub ssmux0: SSMUX0,
    #[doc = "0x44 - ADC Sample Sequence Control 0"]
    pub ssctl0: SSCTL0,
    #[doc = "0x48 - ADC Sample Sequence Result FIFO 0"]
    pub ssfifo0: SSFIFO0,
    #[doc = "0x4c - ADC Sample Sequence FIFO 0 Status"]
    pub ssfstat0: SSFSTAT0,
    _reserved14: [u8; 16usize],
    #[doc = "0x60 - ADC Sample Sequence Input Multiplexer Select 1"]
    pub ssmux1: SSMUX1,
    #[doc = "0x64 - ADC Sample Sequence Control 1"]
    pub ssctl1: SSCTL1,
    #[doc = "0x68 - ADC Sample Sequence Result FIFO 1"]
    pub ssfifo1: SSFIFO1,
    #[doc = "0x6c - ADC Sample Sequence FIFO 1 Status"]
    pub ssfstat1: SSFSTAT1,
    _reserved18: [u8; 16usize],
    #[doc = "0x80 - ADC Sample Sequence Input Multiplexer Select 2"]
    pub ssmux2: SSMUX2,
    #[doc = "0x84 - ADC Sample Sequence Control 2"]
    pub ssctl2: SSCTL2,
    #[doc = "0x88 - ADC Sample Sequence Result FIFO 2"]
    pub ssfifo2: SSFIFO2,
    #[doc = "0x8c - ADC Sample Sequence FIFO 2 Status"]
    pub ssfstat2: SSFSTAT2,
    _reserved22: [u8; 16usize],
    #[doc = "0xa0 - ADC Sample Sequence Input Multiplexer Select 3"]
    pub ssmux3: SSMUX3,
    #[doc = "0xa4 - ADC Sample Sequence Control 3"]
    pub ssctl3: SSCTL3,
    #[doc = "0xa8 - ADC Sample Sequence Result FIFO 3"]
    pub ssfifo3: SSFIFO3,
    #[doc = "0xac - ADC Sample Sequence FIFO 3 Status"]
    pub ssfstat3: SSFSTAT3,
    _reserved26: [u8; 80usize],
    #[doc = "0x100 - ADC Test Mode Loopback"]
    pub tmlb: TMLB,
}
#[doc = "ADC Active Sample Sequencer\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [actss](actss) module"]
pub type ACTSS = crate::Reg<u32, _ACTSS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACTSS;
#[doc = "`read()` method returns [actss::R](actss::R) reader structure"]
impl crate::Readable for ACTSS {}
#[doc = "`write(|w| ..)` method takes [actss::W](actss::W) writer structure"]
impl crate::Writable for ACTSS {}
#[doc = "ADC Active Sample Sequencer"]
pub mod actss;
#[doc = "ADC Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "ADC Raw Interrupt Status"]
pub mod ris;
#[doc = "ADC Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "ADC Interrupt Mask"]
pub mod im;
#[doc = "ADC Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isc](isc) module"]
pub type ISC = crate::Reg<u32, _ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISC;
#[doc = "`read()` method returns [isc::R](isc::R) reader structure"]
impl crate::Readable for ISC {}
#[doc = "`write(|w| ..)` method takes [isc::W](isc::W) writer structure"]
impl crate::Writable for ISC {}
#[doc = "ADC Interrupt Status and Clear"]
pub mod isc;
#[doc = "ADC Overflow Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ostat](ostat) module"]
pub type OSTAT = crate::Reg<u32, _OSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OSTAT;
#[doc = "`read()` method returns [ostat::R](ostat::R) reader structure"]
impl crate::Readable for OSTAT {}
#[doc = "`write(|w| ..)` method takes [ostat::W](ostat::W) writer structure"]
impl crate::Writable for OSTAT {}
#[doc = "ADC Overflow Status"]
pub mod ostat;
#[doc = "ADC Event Multiplexer Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [emux](emux) module"]
pub type EMUX = crate::Reg<u32, _EMUX>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EMUX;
#[doc = "`read()` method returns [emux::R](emux::R) reader structure"]
impl crate::Readable for EMUX {}
#[doc = "`write(|w| ..)` method takes [emux::W](emux::W) writer structure"]
impl crate::Writable for EMUX {}
#[doc = "ADC Event Multiplexer Select"]
pub mod emux;
#[doc = "ADC Underflow Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ustat](ustat) module"]
pub type USTAT = crate::Reg<u32, _USTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USTAT;
#[doc = "`read()` method returns [ustat::R](ustat::R) reader structure"]
impl crate::Readable for USTAT {}
#[doc = "`write(|w| ..)` method takes [ustat::W](ustat::W) writer structure"]
impl crate::Writable for USTAT {}
#[doc = "ADC Underflow Status"]
pub mod ustat;
#[doc = "ADC Sample Sequencer Priority\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sspri](sspri) module"]
pub type SSPRI = crate::Reg<u32, _SSPRI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSPRI;
#[doc = "`read()` method returns [sspri::R](sspri::R) reader structure"]
impl crate::Readable for SSPRI {}
#[doc = "`write(|w| ..)` method takes [sspri::W](sspri::W) writer structure"]
impl crate::Writable for SSPRI {}
#[doc = "ADC Sample Sequencer Priority"]
pub mod sspri;
#[doc = "ADC Processor Sample Sequence Initiate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssi](pssi) module"]
pub type PSSI = crate::Reg<u32, _PSSI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PSSI;
#[doc = "`read()` method returns [pssi::R](pssi::R) reader structure"]
impl crate::Readable for PSSI {}
#[doc = "`write(|w| ..)` method takes [pssi::W](pssi::W) writer structure"]
impl crate::Writable for PSSI {}
#[doc = "ADC Processor Sample Sequence Initiate"]
pub mod pssi;
#[doc = "ADC Sample Averaging Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sac](sac) module"]
pub type SAC = crate::Reg<u32, _SAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAC;
#[doc = "`read()` method returns [sac::R](sac::R) reader structure"]
impl crate::Readable for SAC {}
#[doc = "`write(|w| ..)` method takes [sac::W](sac::W) writer structure"]
impl crate::Writable for SAC {}
#[doc = "ADC Sample Averaging Control"]
pub mod sac;
#[doc = "ADC Sample Sequence Input Multiplexer Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux0](ssmux0) module"]
pub type SSMUX0 = crate::Reg<u32, _SSMUX0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX0;
#[doc = "`read()` method returns [ssmux0::R](ssmux0::R) reader structure"]
impl crate::Readable for SSMUX0 {}
#[doc = "`write(|w| ..)` method takes [ssmux0::W](ssmux0::W) writer structure"]
impl crate::Writable for SSMUX0 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 0"]
pub mod ssmux0;
#[doc = "ADC Sample Sequence Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl0](ssctl0) module"]
pub type SSCTL0 = crate::Reg<u32, _SSCTL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL0;
#[doc = "`read()` method returns [ssctl0::R](ssctl0::R) reader structure"]
impl crate::Readable for SSCTL0 {}
#[doc = "`write(|w| ..)` method takes [ssctl0::W](ssctl0::W) writer structure"]
impl crate::Writable for SSCTL0 {}
#[doc = "ADC Sample Sequence Control 0"]
pub mod ssctl0;
#[doc = "ADC Sample Sequence Result FIFO 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo0](ssfifo0) module"]
pub type SSFIFO0 = crate::Reg<u32, _SSFIFO0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO0;
#[doc = "`read()` method returns [ssfifo0::R](ssfifo0::R) reader structure"]
impl crate::Readable for SSFIFO0 {}
#[doc = "`write(|w| ..)` method takes [ssfifo0::W](ssfifo0::W) writer structure"]
impl crate::Writable for SSFIFO0 {}
#[doc = "ADC Sample Sequence Result FIFO 0"]
pub mod ssfifo0;
#[doc = "ADC Sample Sequence FIFO 0 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat0](ssfstat0) module"]
pub type SSFSTAT0 = crate::Reg<u32, _SSFSTAT0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT0;
#[doc = "`read()` method returns [ssfstat0::R](ssfstat0::R) reader structure"]
impl crate::Readable for SSFSTAT0 {}
#[doc = "`write(|w| ..)` method takes [ssfstat0::W](ssfstat0::W) writer structure"]
impl crate::Writable for SSFSTAT0 {}
#[doc = "ADC Sample Sequence FIFO 0 Status"]
pub mod ssfstat0;
#[doc = "ADC Sample Sequence Input Multiplexer Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux1](ssmux1) module"]
pub type SSMUX1 = crate::Reg<u32, _SSMUX1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX1;
#[doc = "`read()` method returns [ssmux1::R](ssmux1::R) reader structure"]
impl crate::Readable for SSMUX1 {}
#[doc = "`write(|w| ..)` method takes [ssmux1::W](ssmux1::W) writer structure"]
impl crate::Writable for SSMUX1 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 1"]
pub mod ssmux1;
#[doc = "ADC Sample Sequence Control 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl1](ssctl1) module"]
pub type SSCTL1 = crate::Reg<u32, _SSCTL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL1;
#[doc = "`read()` method returns [ssctl1::R](ssctl1::R) reader structure"]
impl crate::Readable for SSCTL1 {}
#[doc = "`write(|w| ..)` method takes [ssctl1::W](ssctl1::W) writer structure"]
impl crate::Writable for SSCTL1 {}
#[doc = "ADC Sample Sequence Control 1"]
pub mod ssctl1;
#[doc = "ADC Sample Sequence Result FIFO 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo1](ssfifo1) module"]
pub type SSFIFO1 = crate::Reg<u32, _SSFIFO1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO1;
#[doc = "`read()` method returns [ssfifo1::R](ssfifo1::R) reader structure"]
impl crate::Readable for SSFIFO1 {}
#[doc = "`write(|w| ..)` method takes [ssfifo1::W](ssfifo1::W) writer structure"]
impl crate::Writable for SSFIFO1 {}
#[doc = "ADC Sample Sequence Result FIFO 1"]
pub mod ssfifo1;
#[doc = "ADC Sample Sequence FIFO 1 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat1](ssfstat1) module"]
pub type SSFSTAT1 = crate::Reg<u32, _SSFSTAT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT1;
#[doc = "`read()` method returns [ssfstat1::R](ssfstat1::R) reader structure"]
impl crate::Readable for SSFSTAT1 {}
#[doc = "`write(|w| ..)` method takes [ssfstat1::W](ssfstat1::W) writer structure"]
impl crate::Writable for SSFSTAT1 {}
#[doc = "ADC Sample Sequence FIFO 1 Status"]
pub mod ssfstat1;
#[doc = "ADC Sample Sequence Input Multiplexer Select 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux2](ssmux2) module"]
pub type SSMUX2 = crate::Reg<u32, _SSMUX2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX2;
#[doc = "`read()` method returns [ssmux2::R](ssmux2::R) reader structure"]
impl crate::Readable for SSMUX2 {}
#[doc = "`write(|w| ..)` method takes [ssmux2::W](ssmux2::W) writer structure"]
impl crate::Writable for SSMUX2 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 2"]
pub mod ssmux2;
#[doc = "ADC Sample Sequence Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl2](ssctl2) module"]
pub type SSCTL2 = crate::Reg<u32, _SSCTL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL2;
#[doc = "`read()` method returns [ssctl2::R](ssctl2::R) reader structure"]
impl crate::Readable for SSCTL2 {}
#[doc = "`write(|w| ..)` method takes [ssctl2::W](ssctl2::W) writer structure"]
impl crate::Writable for SSCTL2 {}
#[doc = "ADC Sample Sequence Control 2"]
pub mod ssctl2;
#[doc = "ADC Sample Sequence Result FIFO 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo2](ssfifo2) module"]
pub type SSFIFO2 = crate::Reg<u32, _SSFIFO2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO2;
#[doc = "`read()` method returns [ssfifo2::R](ssfifo2::R) reader structure"]
impl crate::Readable for SSFIFO2 {}
#[doc = "`write(|w| ..)` method takes [ssfifo2::W](ssfifo2::W) writer structure"]
impl crate::Writable for SSFIFO2 {}
#[doc = "ADC Sample Sequence Result FIFO 2"]
pub mod ssfifo2;
#[doc = "ADC Sample Sequence FIFO 2 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat2](ssfstat2) module"]
pub type SSFSTAT2 = crate::Reg<u32, _SSFSTAT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT2;
#[doc = "`read()` method returns [ssfstat2::R](ssfstat2::R) reader structure"]
impl crate::Readable for SSFSTAT2 {}
#[doc = "`write(|w| ..)` method takes [ssfstat2::W](ssfstat2::W) writer structure"]
impl crate::Writable for SSFSTAT2 {}
#[doc = "ADC Sample Sequence FIFO 2 Status"]
pub mod ssfstat2;
#[doc = "ADC Sample Sequence Input Multiplexer Select 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssmux3](ssmux3) module"]
pub type SSMUX3 = crate::Reg<u32, _SSMUX3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSMUX3;
#[doc = "`read()` method returns [ssmux3::R](ssmux3::R) reader structure"]
impl crate::Readable for SSMUX3 {}
#[doc = "`write(|w| ..)` method takes [ssmux3::W](ssmux3::W) writer structure"]
impl crate::Writable for SSMUX3 {}
#[doc = "ADC Sample Sequence Input Multiplexer Select 3"]
pub mod ssmux3;
#[doc = "ADC Sample Sequence Control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssctl3](ssctl3) module"]
pub type SSCTL3 = crate::Reg<u32, _SSCTL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSCTL3;
#[doc = "`read()` method returns [ssctl3::R](ssctl3::R) reader structure"]
impl crate::Readable for SSCTL3 {}
#[doc = "`write(|w| ..)` method takes [ssctl3::W](ssctl3::W) writer structure"]
impl crate::Writable for SSCTL3 {}
#[doc = "ADC Sample Sequence Control 3"]
pub mod ssctl3;
#[doc = "ADC Sample Sequence Result FIFO 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfifo3](ssfifo3) module"]
pub type SSFIFO3 = crate::Reg<u32, _SSFIFO3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFIFO3;
#[doc = "`read()` method returns [ssfifo3::R](ssfifo3::R) reader structure"]
impl crate::Readable for SSFIFO3 {}
#[doc = "`write(|w| ..)` method takes [ssfifo3::W](ssfifo3::W) writer structure"]
impl crate::Writable for SSFIFO3 {}
#[doc = "ADC Sample Sequence Result FIFO 3"]
pub mod ssfifo3;
#[doc = "ADC Sample Sequence FIFO 3 Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssfstat3](ssfstat3) module"]
pub type SSFSTAT3 = crate::Reg<u32, _SSFSTAT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SSFSTAT3;
#[doc = "`read()` method returns [ssfstat3::R](ssfstat3::R) reader structure"]
impl crate::Readable for SSFSTAT3 {}
#[doc = "`write(|w| ..)` method takes [ssfstat3::W](ssfstat3::W) writer structure"]
impl crate::Writable for SSFSTAT3 {}
#[doc = "ADC Sample Sequence FIFO 3 Status"]
pub mod ssfstat3;
#[doc = "ADC Test Mode Loopback\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmlb](tmlb) module"]
pub type TMLB = crate::Reg<u32, _TMLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TMLB;
#[doc = "`read()` method returns [tmlb::R](tmlb::R) reader structure"]
impl crate::Readable for TMLB {}
#[doc = "`write(|w| ..)` method takes [tmlb::W](tmlb::W) writer structure"]
impl crate::Writable for TMLB {}
#[doc = "ADC Test Mode Loopback"]
pub mod tmlb;
