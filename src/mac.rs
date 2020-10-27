#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_ris: [u8; 4usize],
    #[doc = "0x04 - Ethernet MAC Interrupt Mask"]
    pub im: IM,
    #[doc = "0x08 - Ethernet MAC Receive Control"]
    pub rctl: RCTL,
    #[doc = "0x0c - Ethernet MAC Transmit Control"]
    pub tctl: TCTL,
    _reserved_4_data: [u8; 4usize],
    #[doc = "0x14 - Ethernet MAC Individual Address 0"]
    pub ia0: IA0,
    #[doc = "0x18 - Ethernet MAC Individual Address 1"]
    pub ia1: IA1,
    #[doc = "0x1c - Ethernet MAC Threshold"]
    pub thr: THR,
    #[doc = "0x20 - Ethernet MAC Management Control"]
    pub mctl: MCTL,
    #[doc = "0x24 - Ethernet MAC Management Divider"]
    pub mdv: MDV,
    _reserved10: [u8; 4usize],
    #[doc = "0x2c - Ethernet MAC Management Transmit Data"]
    pub mtxd: MTXD,
    #[doc = "0x30 - Ethernet MAC Management Receive Data"]
    pub mrxd: MRXD,
    #[doc = "0x34 - Ethernet MAC Number of Packets"]
    pub np: NP,
    #[doc = "0x38 - Ethernet MAC Transmission Request"]
    pub tr: TR,
}
impl RegisterBlock {
    #[doc = "0x00 - Ethernet MAC Raw Interrupt Status/Acknowledge"]
    #[inline(always)]
    pub fn iack(&self) -> &IACK {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const IACK) }
    }
    #[doc = "0x00 - Ethernet MAC Raw Interrupt Status/Acknowledge"]
    #[inline(always)]
    pub fn iack_mut(&self) -> &mut IACK {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut IACK) }
    }
    #[doc = "0x00 - Ethernet MAC Raw Interrupt Status/Acknowledge"]
    #[inline(always)]
    pub fn ris(&self) -> &RIS {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const RIS) }
    }
    #[doc = "0x00 - Ethernet MAC Raw Interrupt Status/Acknowledge"]
    #[inline(always)]
    pub fn ris_mut(&self) -> &mut RIS {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut RIS) }
    }
    #[doc = "0x10 - Ethernet MAC Data"]
    #[inline(always)]
    pub fn data(&self) -> &DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const DATA) }
    }
    #[doc = "0x10 - Ethernet MAC Data"]
    #[inline(always)]
    pub fn data_mut(&self) -> &mut DATA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut DATA) }
    }
    #[doc = "0x10 - Ethernet MAC Data"]
    #[inline(always)]
    pub fn data(&self) -> &DATA {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const DATA) }
    }
    #[doc = "0x10 - Ethernet MAC Data"]
    #[inline(always)]
    pub fn data_mut(&self) -> &mut DATA {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut DATA) }
    }
}
#[doc = "Ethernet MAC Raw Interrupt Status/Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "Ethernet MAC Raw Interrupt Status/Acknowledge"]
pub mod ris;
#[doc = "Ethernet MAC Raw Interrupt Status/Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iack](iack) module"]
pub type IACK = crate::Reg<u32, _IACK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IACK;
#[doc = "`read()` method returns [iack::R](iack::R) reader structure"]
impl crate::Readable for IACK {}
#[doc = "`write(|w| ..)` method takes [iack::W](iack::W) writer structure"]
impl crate::Writable for IACK {}
#[doc = "Ethernet MAC Raw Interrupt Status/Acknowledge"]
pub mod iack;
#[doc = "Ethernet MAC Interrupt Mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [im](im) module"]
pub type IM = crate::Reg<u32, _IM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IM;
#[doc = "`read()` method returns [im::R](im::R) reader structure"]
impl crate::Readable for IM {}
#[doc = "`write(|w| ..)` method takes [im::W](im::W) writer structure"]
impl crate::Writable for IM {}
#[doc = "Ethernet MAC Interrupt Mask"]
pub mod im;
#[doc = "Ethernet MAC Receive Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rctl](rctl) module"]
pub type RCTL = crate::Reg<u32, _RCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RCTL;
#[doc = "`read()` method returns [rctl::R](rctl::R) reader structure"]
impl crate::Readable for RCTL {}
#[doc = "`write(|w| ..)` method takes [rctl::W](rctl::W) writer structure"]
impl crate::Writable for RCTL {}
#[doc = "Ethernet MAC Receive Control"]
pub mod rctl;
#[doc = "Ethernet MAC Transmit Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tctl](tctl) module"]
pub type TCTL = crate::Reg<u32, _TCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TCTL;
#[doc = "`read()` method returns [tctl::R](tctl::R) reader structure"]
impl crate::Readable for TCTL {}
#[doc = "`write(|w| ..)` method takes [tctl::W](tctl::W) writer structure"]
impl crate::Writable for TCTL {}
#[doc = "Ethernet MAC Transmit Control"]
pub mod tctl;
#[doc = "Ethernet MAC Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Ethernet MAC Data"]
pub mod data;
#[doc = "Ethernet MAC Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data](data) module"]
pub type DATA = crate::Reg<u32, _DATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA;
#[doc = "`read()` method returns [data::R](data::R) reader structure"]
impl crate::Readable for DATA {}
#[doc = "`write(|w| ..)` method takes [data::W](data::W) writer structure"]
impl crate::Writable for DATA {}
#[doc = "Ethernet MAC Data"]
pub mod data;
#[doc = "Ethernet MAC Individual Address 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ia0](ia0) module"]
pub type IA0 = crate::Reg<u32, _IA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IA0;
#[doc = "`read()` method returns [ia0::R](ia0::R) reader structure"]
impl crate::Readable for IA0 {}
#[doc = "`write(|w| ..)` method takes [ia0::W](ia0::W) writer structure"]
impl crate::Writable for IA0 {}
#[doc = "Ethernet MAC Individual Address 0"]
pub mod ia0;
#[doc = "Ethernet MAC Individual Address 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ia1](ia1) module"]
pub type IA1 = crate::Reg<u32, _IA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IA1;
#[doc = "`read()` method returns [ia1::R](ia1::R) reader structure"]
impl crate::Readable for IA1 {}
#[doc = "`write(|w| ..)` method takes [ia1::W](ia1::W) writer structure"]
impl crate::Writable for IA1 {}
#[doc = "Ethernet MAC Individual Address 1"]
pub mod ia1;
#[doc = "Ethernet MAC Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thr](thr) module"]
pub type THR = crate::Reg<u32, _THR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _THR;
#[doc = "`read()` method returns [thr::R](thr::R) reader structure"]
impl crate::Readable for THR {}
#[doc = "`write(|w| ..)` method takes [thr::W](thr::W) writer structure"]
impl crate::Writable for THR {}
#[doc = "Ethernet MAC Threshold"]
pub mod thr;
#[doc = "Ethernet MAC Management Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctl](mctl) module"]
pub type MCTL = crate::Reg<u32, _MCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MCTL;
#[doc = "`read()` method returns [mctl::R](mctl::R) reader structure"]
impl crate::Readable for MCTL {}
#[doc = "`write(|w| ..)` method takes [mctl::W](mctl::W) writer structure"]
impl crate::Writable for MCTL {}
#[doc = "Ethernet MAC Management Control"]
pub mod mctl;
#[doc = "Ethernet MAC Management Divider\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdv](mdv) module"]
pub type MDV = crate::Reg<u32, _MDV>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MDV;
#[doc = "`read()` method returns [mdv::R](mdv::R) reader structure"]
impl crate::Readable for MDV {}
#[doc = "`write(|w| ..)` method takes [mdv::W](mdv::W) writer structure"]
impl crate::Writable for MDV {}
#[doc = "Ethernet MAC Management Divider"]
pub mod mdv;
#[doc = "Ethernet MAC Management Transmit Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtxd](mtxd) module"]
pub type MTXD = crate::Reg<u32, _MTXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTXD;
#[doc = "`read()` method returns [mtxd::R](mtxd::R) reader structure"]
impl crate::Readable for MTXD {}
#[doc = "`write(|w| ..)` method takes [mtxd::W](mtxd::W) writer structure"]
impl crate::Writable for MTXD {}
#[doc = "Ethernet MAC Management Transmit Data"]
pub mod mtxd;
#[doc = "Ethernet MAC Management Receive Data\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrxd](mrxd) module"]
pub type MRXD = crate::Reg<u32, _MRXD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MRXD;
#[doc = "`read()` method returns [mrxd::R](mrxd::R) reader structure"]
impl crate::Readable for MRXD {}
#[doc = "`write(|w| ..)` method takes [mrxd::W](mrxd::W) writer structure"]
impl crate::Writable for MRXD {}
#[doc = "Ethernet MAC Management Receive Data"]
pub mod mrxd;
#[doc = "Ethernet MAC Number of Packets\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [np](np) module"]
pub type NP = crate::Reg<u32, _NP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NP;
#[doc = "`read()` method returns [np::R](np::R) reader structure"]
impl crate::Readable for NP {}
#[doc = "`write(|w| ..)` method takes [np::W](np::W) writer structure"]
impl crate::Writable for NP {}
#[doc = "Ethernet MAC Number of Packets"]
pub mod np;
#[doc = "Ethernet MAC Transmission Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tr](tr) module"]
pub type TR = crate::Reg<u32, _TR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TR;
#[doc = "`read()` method returns [tr::R](tr::R) reader structure"]
impl crate::Readable for TR {}
#[doc = "`write(|w| ..)` method takes [tr::W](tr::W) writer structure"]
impl crate::Writable for TR {}
#[doc = "Ethernet MAC Transmission Request"]
pub mod tr;
