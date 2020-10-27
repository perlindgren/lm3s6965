#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - PWM Master Control"]
    pub ctl: CTL,
    #[doc = "0x04 - PWM Time Base Sync"]
    pub sync: SYNC,
    #[doc = "0x08 - PWM Output Enable"]
    pub enable: ENABLE,
    #[doc = "0x0c - PWM Output Inversion"]
    pub invert: INVERT,
    #[doc = "0x10 - PWM Output Fault"]
    pub fault: FAULT,
    #[doc = "0x14 - PWM Interrupt Enable"]
    pub inten: INTEN,
    #[doc = "0x18 - PWM Raw Interrupt Status"]
    pub ris: RIS,
    #[doc = "0x1c - PWM Interrupt Status and Clear"]
    pub isc: ISC,
    #[doc = "0x20 - PWM Status"]
    pub status: STATUS,
    _reserved9: [u8; 28usize],
    #[doc = "0x40 - PWM0 Control"]
    pub _0_ctl: _0_CTL,
    #[doc = "0x44 - PWM0 Interrupt and Trigger Enable"]
    pub _0_inten: _0_INTEN,
    #[doc = "0x48 - PWM0 Raw Interrupt Status"]
    pub _0_ris: _0_RIS,
    #[doc = "0x4c - PWM0 Interrupt Status and Clear"]
    pub _0_isc: _0_ISC,
    #[doc = "0x50 - PWM0 Load"]
    pub _0_load: _0_LOAD,
    #[doc = "0x54 - PWM0 Counter"]
    pub _0_count: _0_COUNT,
    #[doc = "0x58 - PWM0 Compare A"]
    pub _0_cmpa: _0_CMPA,
    #[doc = "0x5c - PWM0 Compare B"]
    pub _0_cmpb: _0_CMPB,
    #[doc = "0x60 - PWM0 Generator A Control"]
    pub _0_gena: _0_GENA,
    #[doc = "0x64 - PWM0 Generator B Control"]
    pub _0_genb: _0_GENB,
    #[doc = "0x68 - PWM0 Dead-Band Control"]
    pub _0_dbctl: _0_DBCTL,
    #[doc = "0x6c - PWM0 Dead-Band Rising-Edge Delay"]
    pub _0_dbrise: _0_DBRISE,
    #[doc = "0x70 - PWM0 Dead-Band Falling-Edge-Delay"]
    pub _0_dbfall: _0_DBFALL,
    _reserved22: [u8; 12usize],
    #[doc = "0x80 - PWM1 Control"]
    pub _1_ctl: _1_CTL,
    #[doc = "0x84 - PWM1 Interrupt and Trigger Enable"]
    pub _1_inten: _1_INTEN,
    #[doc = "0x88 - PWM1 Raw Interrupt Status"]
    pub _1_ris: _1_RIS,
    #[doc = "0x8c - PWM1 Interrupt Status and Clear"]
    pub _1_isc: _1_ISC,
    #[doc = "0x90 - PWM1 Load"]
    pub _1_load: _1_LOAD,
    #[doc = "0x94 - PWM1 Counter"]
    pub _1_count: _1_COUNT,
    #[doc = "0x98 - PWM1 Compare A"]
    pub _1_cmpa: _1_CMPA,
    #[doc = "0x9c - PWM1 Compare B"]
    pub _1_cmpb: _1_CMPB,
    #[doc = "0xa0 - PWM1 Generator A Control"]
    pub _1_gena: _1_GENA,
    #[doc = "0xa4 - PWM1 Generator B Control"]
    pub _1_genb: _1_GENB,
    #[doc = "0xa8 - PWM1 Dead-Band Control"]
    pub _1_dbctl: _1_DBCTL,
    #[doc = "0xac - PWM1 Dead-Band Rising-Edge Delay"]
    pub _1_dbrise: _1_DBRISE,
    #[doc = "0xb0 - PWM1 Dead-Band Falling-Edge-Delay"]
    pub _1_dbfall: _1_DBFALL,
    _reserved35: [u8; 12usize],
    #[doc = "0xc0 - PWM2 Control"]
    pub _2_ctl: _2_CTL,
    #[doc = "0xc4 - PWM2 Interrupt and Trigger Enable"]
    pub _2_inten: _2_INTEN,
    #[doc = "0xc8 - PWM2 Raw Interrupt Status"]
    pub _2_ris: _2_RIS,
    #[doc = "0xcc - PWM2 Interrupt Status and Clear"]
    pub _2_isc: _2_ISC,
    #[doc = "0xd0 - PWM2 Load"]
    pub _2_load: _2_LOAD,
    #[doc = "0xd4 - PWM2 Counter"]
    pub _2_count: _2_COUNT,
    #[doc = "0xd8 - PWM2 Compare A"]
    pub _2_cmpa: _2_CMPA,
    #[doc = "0xdc - PWM2 Compare B"]
    pub _2_cmpb: _2_CMPB,
    #[doc = "0xe0 - PWM2 Generator A Control"]
    pub _2_gena: _2_GENA,
    #[doc = "0xe4 - PWM2 Generator B Control"]
    pub _2_genb: _2_GENB,
    #[doc = "0xe8 - PWM2 Dead-Band Control"]
    pub _2_dbctl: _2_DBCTL,
    #[doc = "0xec - PWM2 Dead-Band Rising-Edge Delay"]
    pub _2_dbrise: _2_DBRISE,
    #[doc = "0xf0 - PWM2 Dead-Band Falling-Edge-Delay"]
    pub _2_dbfall: _2_DBFALL,
}
#[doc = "PWM Master Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](ctl) module"]
pub type CTL = crate::Reg<u32, _CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTL;
#[doc = "`read()` method returns [ctl::R](ctl::R) reader structure"]
impl crate::Readable for CTL {}
#[doc = "`write(|w| ..)` method takes [ctl::W](ctl::W) writer structure"]
impl crate::Writable for CTL {}
#[doc = "PWM Master Control"]
pub mod ctl;
#[doc = "PWM Time Base Sync\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "PWM Time Base Sync"]
pub mod sync;
#[doc = "PWM Output Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](enable) module"]
pub type ENABLE = crate::Reg<u32, _ENABLE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENABLE;
#[doc = "`read()` method returns [enable::R](enable::R) reader structure"]
impl crate::Readable for ENABLE {}
#[doc = "`write(|w| ..)` method takes [enable::W](enable::W) writer structure"]
impl crate::Writable for ENABLE {}
#[doc = "PWM Output Enable"]
pub mod enable;
#[doc = "PWM Output Inversion\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [invert](invert) module"]
pub type INVERT = crate::Reg<u32, _INVERT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INVERT;
#[doc = "`read()` method returns [invert::R](invert::R) reader structure"]
impl crate::Readable for INVERT {}
#[doc = "`write(|w| ..)` method takes [invert::W](invert::W) writer structure"]
impl crate::Writable for INVERT {}
#[doc = "PWM Output Inversion"]
pub mod invert;
#[doc = "PWM Output Fault\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fault](fault) module"]
pub type FAULT = crate::Reg<u32, _FAULT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FAULT;
#[doc = "`read()` method returns [fault::R](fault::R) reader structure"]
impl crate::Readable for FAULT {}
#[doc = "`write(|w| ..)` method takes [fault::W](fault::W) writer structure"]
impl crate::Writable for FAULT {}
#[doc = "PWM Output Fault"]
pub mod fault;
#[doc = "PWM Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](inten) module"]
pub type INTEN = crate::Reg<u32, _INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INTEN;
#[doc = "`read()` method returns [inten::R](inten::R) reader structure"]
impl crate::Readable for INTEN {}
#[doc = "`write(|w| ..)` method takes [inten::W](inten::W) writer structure"]
impl crate::Writable for INTEN {}
#[doc = "PWM Interrupt Enable"]
pub mod inten;
#[doc = "PWM Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ris](ris) module"]
pub type RIS = crate::Reg<u32, _RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RIS;
#[doc = "`read()` method returns [ris::R](ris::R) reader structure"]
impl crate::Readable for RIS {}
#[doc = "`write(|w| ..)` method takes [ris::W](ris::W) writer structure"]
impl crate::Writable for RIS {}
#[doc = "PWM Raw Interrupt Status"]
pub mod ris;
#[doc = "PWM Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isc](isc) module"]
pub type ISC = crate::Reg<u32, _ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISC;
#[doc = "`read()` method returns [isc::R](isc::R) reader structure"]
impl crate::Readable for ISC {}
#[doc = "`write(|w| ..)` method takes [isc::W](isc::W) writer structure"]
impl crate::Writable for ISC {}
#[doc = "PWM Interrupt Status and Clear"]
pub mod isc;
#[doc = "PWM Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "PWM Status"]
pub mod status;
#[doc = "PWM0 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_ctl](_0_ctl) module"]
pub type _0_CTL = crate::Reg<u32, __0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_CTL;
#[doc = "`read()` method returns [_0_ctl::R](_0_ctl::R) reader structure"]
impl crate::Readable for _0_CTL {}
#[doc = "`write(|w| ..)` method takes [_0_ctl::W](_0_ctl::W) writer structure"]
impl crate::Writable for _0_CTL {}
#[doc = "PWM0 Control"]
pub mod _0_ctl;
#[doc = "PWM0 Interrupt and Trigger Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_inten](_0_inten) module"]
pub type _0_INTEN = crate::Reg<u32, __0_INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_INTEN;
#[doc = "`read()` method returns [_0_inten::R](_0_inten::R) reader structure"]
impl crate::Readable for _0_INTEN {}
#[doc = "`write(|w| ..)` method takes [_0_inten::W](_0_inten::W) writer structure"]
impl crate::Writable for _0_INTEN {}
#[doc = "PWM0 Interrupt and Trigger Enable"]
pub mod _0_inten;
#[doc = "PWM0 Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_ris](_0_ris) module"]
pub type _0_RIS = crate::Reg<u32, __0_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_RIS;
#[doc = "`read()` method returns [_0_ris::R](_0_ris::R) reader structure"]
impl crate::Readable for _0_RIS {}
#[doc = "`write(|w| ..)` method takes [_0_ris::W](_0_ris::W) writer structure"]
impl crate::Writable for _0_RIS {}
#[doc = "PWM0 Raw Interrupt Status"]
pub mod _0_ris;
#[doc = "PWM0 Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_isc](_0_isc) module"]
pub type _0_ISC = crate::Reg<u32, __0_ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_ISC;
#[doc = "`read()` method returns [_0_isc::R](_0_isc::R) reader structure"]
impl crate::Readable for _0_ISC {}
#[doc = "`write(|w| ..)` method takes [_0_isc::W](_0_isc::W) writer structure"]
impl crate::Writable for _0_ISC {}
#[doc = "PWM0 Interrupt Status and Clear"]
pub mod _0_isc;
#[doc = "PWM0 Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_load](_0_load) module"]
pub type _0_LOAD = crate::Reg<u32, __0_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_LOAD;
#[doc = "`read()` method returns [_0_load::R](_0_load::R) reader structure"]
impl crate::Readable for _0_LOAD {}
#[doc = "`write(|w| ..)` method takes [_0_load::W](_0_load::W) writer structure"]
impl crate::Writable for _0_LOAD {}
#[doc = "PWM0 Load"]
pub mod _0_load;
#[doc = "PWM0 Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_count](_0_count) module"]
pub type _0_COUNT = crate::Reg<u32, __0_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_COUNT;
#[doc = "`read()` method returns [_0_count::R](_0_count::R) reader structure"]
impl crate::Readable for _0_COUNT {}
#[doc = "`write(|w| ..)` method takes [_0_count::W](_0_count::W) writer structure"]
impl crate::Writable for _0_COUNT {}
#[doc = "PWM0 Counter"]
pub mod _0_count;
#[doc = "PWM0 Compare A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_cmpa](_0_cmpa) module"]
pub type _0_CMPA = crate::Reg<u32, __0_CMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_CMPA;
#[doc = "`read()` method returns [_0_cmpa::R](_0_cmpa::R) reader structure"]
impl crate::Readable for _0_CMPA {}
#[doc = "`write(|w| ..)` method takes [_0_cmpa::W](_0_cmpa::W) writer structure"]
impl crate::Writable for _0_CMPA {}
#[doc = "PWM0 Compare A"]
pub mod _0_cmpa;
#[doc = "PWM0 Compare B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_cmpb](_0_cmpb) module"]
pub type _0_CMPB = crate::Reg<u32, __0_CMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_CMPB;
#[doc = "`read()` method returns [_0_cmpb::R](_0_cmpb::R) reader structure"]
impl crate::Readable for _0_CMPB {}
#[doc = "`write(|w| ..)` method takes [_0_cmpb::W](_0_cmpb::W) writer structure"]
impl crate::Writable for _0_CMPB {}
#[doc = "PWM0 Compare B"]
pub mod _0_cmpb;
#[doc = "PWM0 Generator A Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_gena](_0_gena) module"]
pub type _0_GENA = crate::Reg<u32, __0_GENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_GENA;
#[doc = "`read()` method returns [_0_gena::R](_0_gena::R) reader structure"]
impl crate::Readable for _0_GENA {}
#[doc = "`write(|w| ..)` method takes [_0_gena::W](_0_gena::W) writer structure"]
impl crate::Writable for _0_GENA {}
#[doc = "PWM0 Generator A Control"]
pub mod _0_gena;
#[doc = "PWM0 Generator B Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_genb](_0_genb) module"]
pub type _0_GENB = crate::Reg<u32, __0_GENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_GENB;
#[doc = "`read()` method returns [_0_genb::R](_0_genb::R) reader structure"]
impl crate::Readable for _0_GENB {}
#[doc = "`write(|w| ..)` method takes [_0_genb::W](_0_genb::W) writer structure"]
impl crate::Writable for _0_GENB {}
#[doc = "PWM0 Generator B Control"]
pub mod _0_genb;
#[doc = "PWM0 Dead-Band Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_dbctl](_0_dbctl) module"]
pub type _0_DBCTL = crate::Reg<u32, __0_DBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_DBCTL;
#[doc = "`read()` method returns [_0_dbctl::R](_0_dbctl::R) reader structure"]
impl crate::Readable for _0_DBCTL {}
#[doc = "`write(|w| ..)` method takes [_0_dbctl::W](_0_dbctl::W) writer structure"]
impl crate::Writable for _0_DBCTL {}
#[doc = "PWM0 Dead-Band Control"]
pub mod _0_dbctl;
#[doc = "PWM0 Dead-Band Rising-Edge Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_dbrise](_0_dbrise) module"]
pub type _0_DBRISE = crate::Reg<u32, __0_DBRISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_DBRISE;
#[doc = "`read()` method returns [_0_dbrise::R](_0_dbrise::R) reader structure"]
impl crate::Readable for _0_DBRISE {}
#[doc = "`write(|w| ..)` method takes [_0_dbrise::W](_0_dbrise::W) writer structure"]
impl crate::Writable for _0_DBRISE {}
#[doc = "PWM0 Dead-Band Rising-Edge Delay"]
pub mod _0_dbrise;
#[doc = "PWM0 Dead-Band Falling-Edge-Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_0_dbfall](_0_dbfall) module"]
pub type _0_DBFALL = crate::Reg<u32, __0_DBFALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __0_DBFALL;
#[doc = "`read()` method returns [_0_dbfall::R](_0_dbfall::R) reader structure"]
impl crate::Readable for _0_DBFALL {}
#[doc = "`write(|w| ..)` method takes [_0_dbfall::W](_0_dbfall::W) writer structure"]
impl crate::Writable for _0_DBFALL {}
#[doc = "PWM0 Dead-Band Falling-Edge-Delay"]
pub mod _0_dbfall;
#[doc = "PWM1 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_ctl](_1_ctl) module"]
pub type _1_CTL = crate::Reg<u32, __1_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_CTL;
#[doc = "`read()` method returns [_1_ctl::R](_1_ctl::R) reader structure"]
impl crate::Readable for _1_CTL {}
#[doc = "`write(|w| ..)` method takes [_1_ctl::W](_1_ctl::W) writer structure"]
impl crate::Writable for _1_CTL {}
#[doc = "PWM1 Control"]
pub mod _1_ctl;
#[doc = "PWM1 Interrupt and Trigger Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_inten](_1_inten) module"]
pub type _1_INTEN = crate::Reg<u32, __1_INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_INTEN;
#[doc = "`read()` method returns [_1_inten::R](_1_inten::R) reader structure"]
impl crate::Readable for _1_INTEN {}
#[doc = "`write(|w| ..)` method takes [_1_inten::W](_1_inten::W) writer structure"]
impl crate::Writable for _1_INTEN {}
#[doc = "PWM1 Interrupt and Trigger Enable"]
pub mod _1_inten;
#[doc = "PWM1 Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_ris](_1_ris) module"]
pub type _1_RIS = crate::Reg<u32, __1_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_RIS;
#[doc = "`read()` method returns [_1_ris::R](_1_ris::R) reader structure"]
impl crate::Readable for _1_RIS {}
#[doc = "`write(|w| ..)` method takes [_1_ris::W](_1_ris::W) writer structure"]
impl crate::Writable for _1_RIS {}
#[doc = "PWM1 Raw Interrupt Status"]
pub mod _1_ris;
#[doc = "PWM1 Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_isc](_1_isc) module"]
pub type _1_ISC = crate::Reg<u32, __1_ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_ISC;
#[doc = "`read()` method returns [_1_isc::R](_1_isc::R) reader structure"]
impl crate::Readable for _1_ISC {}
#[doc = "`write(|w| ..)` method takes [_1_isc::W](_1_isc::W) writer structure"]
impl crate::Writable for _1_ISC {}
#[doc = "PWM1 Interrupt Status and Clear"]
pub mod _1_isc;
#[doc = "PWM1 Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_load](_1_load) module"]
pub type _1_LOAD = crate::Reg<u32, __1_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_LOAD;
#[doc = "`read()` method returns [_1_load::R](_1_load::R) reader structure"]
impl crate::Readable for _1_LOAD {}
#[doc = "`write(|w| ..)` method takes [_1_load::W](_1_load::W) writer structure"]
impl crate::Writable for _1_LOAD {}
#[doc = "PWM1 Load"]
pub mod _1_load;
#[doc = "PWM1 Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_count](_1_count) module"]
pub type _1_COUNT = crate::Reg<u32, __1_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_COUNT;
#[doc = "`read()` method returns [_1_count::R](_1_count::R) reader structure"]
impl crate::Readable for _1_COUNT {}
#[doc = "`write(|w| ..)` method takes [_1_count::W](_1_count::W) writer structure"]
impl crate::Writable for _1_COUNT {}
#[doc = "PWM1 Counter"]
pub mod _1_count;
#[doc = "PWM1 Compare A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_cmpa](_1_cmpa) module"]
pub type _1_CMPA = crate::Reg<u32, __1_CMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_CMPA;
#[doc = "`read()` method returns [_1_cmpa::R](_1_cmpa::R) reader structure"]
impl crate::Readable for _1_CMPA {}
#[doc = "`write(|w| ..)` method takes [_1_cmpa::W](_1_cmpa::W) writer structure"]
impl crate::Writable for _1_CMPA {}
#[doc = "PWM1 Compare A"]
pub mod _1_cmpa;
#[doc = "PWM1 Compare B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_cmpb](_1_cmpb) module"]
pub type _1_CMPB = crate::Reg<u32, __1_CMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_CMPB;
#[doc = "`read()` method returns [_1_cmpb::R](_1_cmpb::R) reader structure"]
impl crate::Readable for _1_CMPB {}
#[doc = "`write(|w| ..)` method takes [_1_cmpb::W](_1_cmpb::W) writer structure"]
impl crate::Writable for _1_CMPB {}
#[doc = "PWM1 Compare B"]
pub mod _1_cmpb;
#[doc = "PWM1 Generator A Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_gena](_1_gena) module"]
pub type _1_GENA = crate::Reg<u32, __1_GENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_GENA;
#[doc = "`read()` method returns [_1_gena::R](_1_gena::R) reader structure"]
impl crate::Readable for _1_GENA {}
#[doc = "`write(|w| ..)` method takes [_1_gena::W](_1_gena::W) writer structure"]
impl crate::Writable for _1_GENA {}
#[doc = "PWM1 Generator A Control"]
pub mod _1_gena;
#[doc = "PWM1 Generator B Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_genb](_1_genb) module"]
pub type _1_GENB = crate::Reg<u32, __1_GENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_GENB;
#[doc = "`read()` method returns [_1_genb::R](_1_genb::R) reader structure"]
impl crate::Readable for _1_GENB {}
#[doc = "`write(|w| ..)` method takes [_1_genb::W](_1_genb::W) writer structure"]
impl crate::Writable for _1_GENB {}
#[doc = "PWM1 Generator B Control"]
pub mod _1_genb;
#[doc = "PWM1 Dead-Band Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_dbctl](_1_dbctl) module"]
pub type _1_DBCTL = crate::Reg<u32, __1_DBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_DBCTL;
#[doc = "`read()` method returns [_1_dbctl::R](_1_dbctl::R) reader structure"]
impl crate::Readable for _1_DBCTL {}
#[doc = "`write(|w| ..)` method takes [_1_dbctl::W](_1_dbctl::W) writer structure"]
impl crate::Writable for _1_DBCTL {}
#[doc = "PWM1 Dead-Band Control"]
pub mod _1_dbctl;
#[doc = "PWM1 Dead-Band Rising-Edge Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_dbrise](_1_dbrise) module"]
pub type _1_DBRISE = crate::Reg<u32, __1_DBRISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_DBRISE;
#[doc = "`read()` method returns [_1_dbrise::R](_1_dbrise::R) reader structure"]
impl crate::Readable for _1_DBRISE {}
#[doc = "`write(|w| ..)` method takes [_1_dbrise::W](_1_dbrise::W) writer structure"]
impl crate::Writable for _1_DBRISE {}
#[doc = "PWM1 Dead-Band Rising-Edge Delay"]
pub mod _1_dbrise;
#[doc = "PWM1 Dead-Band Falling-Edge-Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_1_dbfall](_1_dbfall) module"]
pub type _1_DBFALL = crate::Reg<u32, __1_DBFALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __1_DBFALL;
#[doc = "`read()` method returns [_1_dbfall::R](_1_dbfall::R) reader structure"]
impl crate::Readable for _1_DBFALL {}
#[doc = "`write(|w| ..)` method takes [_1_dbfall::W](_1_dbfall::W) writer structure"]
impl crate::Writable for _1_DBFALL {}
#[doc = "PWM1 Dead-Band Falling-Edge-Delay"]
pub mod _1_dbfall;
#[doc = "PWM2 Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_ctl](_2_ctl) module"]
pub type _2_CTL = crate::Reg<u32, __2_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_CTL;
#[doc = "`read()` method returns [_2_ctl::R](_2_ctl::R) reader structure"]
impl crate::Readable for _2_CTL {}
#[doc = "`write(|w| ..)` method takes [_2_ctl::W](_2_ctl::W) writer structure"]
impl crate::Writable for _2_CTL {}
#[doc = "PWM2 Control"]
pub mod _2_ctl;
#[doc = "PWM2 Interrupt and Trigger Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_inten](_2_inten) module"]
pub type _2_INTEN = crate::Reg<u32, __2_INTEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_INTEN;
#[doc = "`read()` method returns [_2_inten::R](_2_inten::R) reader structure"]
impl crate::Readable for _2_INTEN {}
#[doc = "`write(|w| ..)` method takes [_2_inten::W](_2_inten::W) writer structure"]
impl crate::Writable for _2_INTEN {}
#[doc = "PWM2 Interrupt and Trigger Enable"]
pub mod _2_inten;
#[doc = "PWM2 Raw Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_ris](_2_ris) module"]
pub type _2_RIS = crate::Reg<u32, __2_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_RIS;
#[doc = "`read()` method returns [_2_ris::R](_2_ris::R) reader structure"]
impl crate::Readable for _2_RIS {}
#[doc = "`write(|w| ..)` method takes [_2_ris::W](_2_ris::W) writer structure"]
impl crate::Writable for _2_RIS {}
#[doc = "PWM2 Raw Interrupt Status"]
pub mod _2_ris;
#[doc = "PWM2 Interrupt Status and Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_isc](_2_isc) module"]
pub type _2_ISC = crate::Reg<u32, __2_ISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_ISC;
#[doc = "`read()` method returns [_2_isc::R](_2_isc::R) reader structure"]
impl crate::Readable for _2_ISC {}
#[doc = "`write(|w| ..)` method takes [_2_isc::W](_2_isc::W) writer structure"]
impl crate::Writable for _2_ISC {}
#[doc = "PWM2 Interrupt Status and Clear"]
pub mod _2_isc;
#[doc = "PWM2 Load\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_load](_2_load) module"]
pub type _2_LOAD = crate::Reg<u32, __2_LOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_LOAD;
#[doc = "`read()` method returns [_2_load::R](_2_load::R) reader structure"]
impl crate::Readable for _2_LOAD {}
#[doc = "`write(|w| ..)` method takes [_2_load::W](_2_load::W) writer structure"]
impl crate::Writable for _2_LOAD {}
#[doc = "PWM2 Load"]
pub mod _2_load;
#[doc = "PWM2 Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_count](_2_count) module"]
pub type _2_COUNT = crate::Reg<u32, __2_COUNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_COUNT;
#[doc = "`read()` method returns [_2_count::R](_2_count::R) reader structure"]
impl crate::Readable for _2_COUNT {}
#[doc = "`write(|w| ..)` method takes [_2_count::W](_2_count::W) writer structure"]
impl crate::Writable for _2_COUNT {}
#[doc = "PWM2 Counter"]
pub mod _2_count;
#[doc = "PWM2 Compare A\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_cmpa](_2_cmpa) module"]
pub type _2_CMPA = crate::Reg<u32, __2_CMPA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_CMPA;
#[doc = "`read()` method returns [_2_cmpa::R](_2_cmpa::R) reader structure"]
impl crate::Readable for _2_CMPA {}
#[doc = "`write(|w| ..)` method takes [_2_cmpa::W](_2_cmpa::W) writer structure"]
impl crate::Writable for _2_CMPA {}
#[doc = "PWM2 Compare A"]
pub mod _2_cmpa;
#[doc = "PWM2 Compare B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_cmpb](_2_cmpb) module"]
pub type _2_CMPB = crate::Reg<u32, __2_CMPB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_CMPB;
#[doc = "`read()` method returns [_2_cmpb::R](_2_cmpb::R) reader structure"]
impl crate::Readable for _2_CMPB {}
#[doc = "`write(|w| ..)` method takes [_2_cmpb::W](_2_cmpb::W) writer structure"]
impl crate::Writable for _2_CMPB {}
#[doc = "PWM2 Compare B"]
pub mod _2_cmpb;
#[doc = "PWM2 Generator A Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_gena](_2_gena) module"]
pub type _2_GENA = crate::Reg<u32, __2_GENA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_GENA;
#[doc = "`read()` method returns [_2_gena::R](_2_gena::R) reader structure"]
impl crate::Readable for _2_GENA {}
#[doc = "`write(|w| ..)` method takes [_2_gena::W](_2_gena::W) writer structure"]
impl crate::Writable for _2_GENA {}
#[doc = "PWM2 Generator A Control"]
pub mod _2_gena;
#[doc = "PWM2 Generator B Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_genb](_2_genb) module"]
pub type _2_GENB = crate::Reg<u32, __2_GENB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_GENB;
#[doc = "`read()` method returns [_2_genb::R](_2_genb::R) reader structure"]
impl crate::Readable for _2_GENB {}
#[doc = "`write(|w| ..)` method takes [_2_genb::W](_2_genb::W) writer structure"]
impl crate::Writable for _2_GENB {}
#[doc = "PWM2 Generator B Control"]
pub mod _2_genb;
#[doc = "PWM2 Dead-Band Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_dbctl](_2_dbctl) module"]
pub type _2_DBCTL = crate::Reg<u32, __2_DBCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_DBCTL;
#[doc = "`read()` method returns [_2_dbctl::R](_2_dbctl::R) reader structure"]
impl crate::Readable for _2_DBCTL {}
#[doc = "`write(|w| ..)` method takes [_2_dbctl::W](_2_dbctl::W) writer structure"]
impl crate::Writable for _2_DBCTL {}
#[doc = "PWM2 Dead-Band Control"]
pub mod _2_dbctl;
#[doc = "PWM2 Dead-Band Rising-Edge Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_dbrise](_2_dbrise) module"]
pub type _2_DBRISE = crate::Reg<u32, __2_DBRISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_DBRISE;
#[doc = "`read()` method returns [_2_dbrise::R](_2_dbrise::R) reader structure"]
impl crate::Readable for _2_DBRISE {}
#[doc = "`write(|w| ..)` method takes [_2_dbrise::W](_2_dbrise::W) writer structure"]
impl crate::Writable for _2_DBRISE {}
#[doc = "PWM2 Dead-Band Rising-Edge Delay"]
pub mod _2_dbrise;
#[doc = "PWM2 Dead-Band Falling-Edge-Delay\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [_2_dbfall](_2_dbfall) module"]
pub type _2_DBFALL = crate::Reg<u32, __2_DBFALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct __2_DBFALL;
#[doc = "`read()` method returns [_2_dbfall::R](_2_dbfall::R) reader structure"]
impl crate::Readable for _2_DBFALL {}
#[doc = "`write(|w| ..)` method takes [_2_dbfall::W](_2_dbfall::W) writer structure"]
impl crate::Writable for _2_DBFALL {}
#[doc = "PWM2 Dead-Band Falling-Edge-Delay"]
pub mod _2_dbfall;
