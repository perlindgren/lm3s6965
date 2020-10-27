#[doc = "Reader of register CTL"]
pub type R = crate::R<u32, super::CTL>;
#[doc = "Writer for register CTL"]
pub type W = crate::W<u32, super::CTL>;
#[doc = "Register CTL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `QEI_CTL_ENABLE`"]
pub type QEI_CTL_ENABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_ENABLE`"]
pub struct QEI_CTL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_ENABLE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_SWAP`"]
pub type QEI_CTL_SWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_SWAP`"]
pub struct QEI_CTL_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_SWAP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_SIGMODE`"]
pub type QEI_CTL_SIGMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_SIGMODE`"]
pub struct QEI_CTL_SIGMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_SIGMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_CAPMODE`"]
pub type QEI_CTL_CAPMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_CAPMODE`"]
pub struct QEI_CTL_CAPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_CAPMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_RESMODE`"]
pub type QEI_CTL_RESMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_RESMODE`"]
pub struct QEI_CTL_RESMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_RESMODE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_VELEN`"]
pub type QEI_CTL_VELEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_VELEN`"]
pub struct QEI_CTL_VELEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_VELEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Predivide Velocity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum QEI_CTL_VELDIV_A {
    #[doc = "0: QEI clock /1"]
    QEI_CTL_VELDIV_1 = 0,
    #[doc = "1: QEI clock /2"]
    QEI_CTL_VELDIV_2 = 1,
    #[doc = "2: QEI clock /4"]
    QEI_CTL_VELDIV_4 = 2,
    #[doc = "3: QEI clock /8"]
    QEI_CTL_VELDIV_8 = 3,
    #[doc = "4: QEI clock /16"]
    QEI_CTL_VELDIV_16 = 4,
    #[doc = "5: QEI clock /32"]
    QEI_CTL_VELDIV_32 = 5,
    #[doc = "6: QEI clock /64"]
    QEI_CTL_VELDIV_64 = 6,
    #[doc = "7: QEI clock /128"]
    QEI_CTL_VELDIV_128 = 7,
}
impl From<QEI_CTL_VELDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: QEI_CTL_VELDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `QEI_CTL_VELDIV`"]
pub type QEI_CTL_VELDIV_R = crate::R<u8, QEI_CTL_VELDIV_A>;
impl QEI_CTL_VELDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QEI_CTL_VELDIV_A {
        match self.bits {
            0 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1,
            1 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2,
            2 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4,
            3 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8,
            4 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16,
            5 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32,
            6 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64,
            7 => QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_1`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_1(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_2`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_2(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_4`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_4(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_8`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_8(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_16`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_16(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_32`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_32(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_64`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_64(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64
    }
    #[doc = "Checks if the value of the field is `QEI_CTL_VELDIV_128`"]
    #[inline(always)]
    pub fn is_qei_ctl_veldiv_128(&self) -> bool {
        *self == QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128
    }
}
#[doc = "Write proxy for field `QEI_CTL_VELDIV`"]
pub struct QEI_CTL_VELDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_VELDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QEI_CTL_VELDIV_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "QEI clock /1"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_1(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_1)
    }
    #[doc = "QEI clock /2"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_2(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_2)
    }
    #[doc = "QEI clock /4"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_4(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_4)
    }
    #[doc = "QEI clock /8"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_8(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_8)
    }
    #[doc = "QEI clock /16"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_16(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_16)
    }
    #[doc = "QEI clock /32"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_32(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_32)
    }
    #[doc = "QEI clock /64"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_64(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_64)
    }
    #[doc = "QEI clock /128"]
    #[inline(always)]
    pub fn qei_ctl_veldiv_128(self) -> &'a mut W {
        self.variant(QEI_CTL_VELDIV_A::QEI_CTL_VELDIV_128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_INVA`"]
pub type QEI_CTL_INVA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_INVA`"]
pub struct QEI_CTL_INVA_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_INVA_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_INVB`"]
pub type QEI_CTL_INVB_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_INVB`"]
pub struct QEI_CTL_INVB_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_INVB_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_INVI`"]
pub type QEI_CTL_INVI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_INVI`"]
pub struct QEI_CTL_INVI_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_INVI_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `QEI_CTL_STALLEN`"]
pub type QEI_CTL_STALLEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `QEI_CTL_STALLEN`"]
pub struct QEI_CTL_STALLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> QEI_CTL_STALLEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn qei_ctl_enable(&self) -> QEI_CTL_ENABLE_R {
        QEI_CTL_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn qei_ctl_swap(&self) -> QEI_CTL_SWAP_R {
        QEI_CTL_SWAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn qei_ctl_sigmode(&self) -> QEI_CTL_SIGMODE_R {
        QEI_CTL_SIGMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn qei_ctl_capmode(&self) -> QEI_CTL_CAPMODE_R {
        QEI_CTL_CAPMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn qei_ctl_resmode(&self) -> QEI_CTL_RESMODE_R {
        QEI_CTL_RESMODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn qei_ctl_velen(&self) -> QEI_CTL_VELEN_R {
        QEI_CTL_VELEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn qei_ctl_veldiv(&self) -> QEI_CTL_VELDIV_R {
        QEI_CTL_VELDIV_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn qei_ctl_inva(&self) -> QEI_CTL_INVA_R {
        QEI_CTL_INVA_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn qei_ctl_invb(&self) -> QEI_CTL_INVB_R {
        QEI_CTL_INVB_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn qei_ctl_invi(&self) -> QEI_CTL_INVI_R {
        QEI_CTL_INVI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn qei_ctl_stallen(&self) -> QEI_CTL_STALLEN_R {
        QEI_CTL_STALLEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable QEI"]
    #[inline(always)]
    pub fn qei_ctl_enable(&mut self) -> QEI_CTL_ENABLE_W {
        QEI_CTL_ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Swap Signals"]
    #[inline(always)]
    pub fn qei_ctl_swap(&mut self) -> QEI_CTL_SWAP_W {
        QEI_CTL_SWAP_W { w: self }
    }
    #[doc = "Bit 2 - Signal Mode"]
    #[inline(always)]
    pub fn qei_ctl_sigmode(&mut self) -> QEI_CTL_SIGMODE_W {
        QEI_CTL_SIGMODE_W { w: self }
    }
    #[doc = "Bit 3 - Capture Mode"]
    #[inline(always)]
    pub fn qei_ctl_capmode(&mut self) -> QEI_CTL_CAPMODE_W {
        QEI_CTL_CAPMODE_W { w: self }
    }
    #[doc = "Bit 4 - Reset Mode"]
    #[inline(always)]
    pub fn qei_ctl_resmode(&mut self) -> QEI_CTL_RESMODE_W {
        QEI_CTL_RESMODE_W { w: self }
    }
    #[doc = "Bit 5 - Capture Velocity"]
    #[inline(always)]
    pub fn qei_ctl_velen(&mut self) -> QEI_CTL_VELEN_W {
        QEI_CTL_VELEN_W { w: self }
    }
    #[doc = "Bits 6:8 - Predivide Velocity"]
    #[inline(always)]
    pub fn qei_ctl_veldiv(&mut self) -> QEI_CTL_VELDIV_W {
        QEI_CTL_VELDIV_W { w: self }
    }
    #[doc = "Bit 9 - Invert PhA"]
    #[inline(always)]
    pub fn qei_ctl_inva(&mut self) -> QEI_CTL_INVA_W {
        QEI_CTL_INVA_W { w: self }
    }
    #[doc = "Bit 10 - Invert PhB"]
    #[inline(always)]
    pub fn qei_ctl_invb(&mut self) -> QEI_CTL_INVB_W {
        QEI_CTL_INVB_W { w: self }
    }
    #[doc = "Bit 11 - Invert Index Pulse"]
    #[inline(always)]
    pub fn qei_ctl_invi(&mut self) -> QEI_CTL_INVI_W {
        QEI_CTL_INVI_W { w: self }
    }
    #[doc = "Bit 12 - Stall QEI"]
    #[inline(always)]
    pub fn qei_ctl_stallen(&mut self) -> QEI_CTL_STALLEN_W {
        QEI_CTL_STALLEN_W { w: self }
    }
}
