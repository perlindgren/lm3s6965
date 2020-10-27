#[doc = "Reader of register IM"]
pub type R = crate::R<u32, super::IM>;
#[doc = "Writer for register IM"]
pub type W = crate::W<u32, super::IM>;
#[doc = "Register IM `reset()`'s with value 0"]
impl crate::ResetValue for super::IM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSI_IM_RORIM`"]
pub type SSI_IM_RORIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_IM_RORIM`"]
pub struct SSI_IM_RORIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_IM_RORIM_W<'a> {
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
#[doc = "Reader of field `SSI_IM_RTIM`"]
pub type SSI_IM_RTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_IM_RTIM`"]
pub struct SSI_IM_RTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_IM_RTIM_W<'a> {
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
#[doc = "Reader of field `SSI_IM_RXIM`"]
pub type SSI_IM_RXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_IM_RXIM`"]
pub struct SSI_IM_RXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_IM_RXIM_W<'a> {
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
#[doc = "Reader of field `SSI_IM_TXIM`"]
pub type SSI_IM_TXIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_IM_TXIM`"]
pub struct SSI_IM_TXIM_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_IM_TXIM_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rorim(&self) -> SSI_IM_RORIM_R {
        SSI_IM_RORIM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rtim(&self) -> SSI_IM_RTIM_R {
        SSI_IM_RTIM_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rxim(&self) -> SSI_IM_RXIM_R {
        SSI_IM_RXIM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_txim(&self) -> SSI_IM_TXIM_R {
        SSI_IM_TXIM_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Receive Overrun Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rorim(&mut self) -> SSI_IM_RORIM_W {
        SSI_IM_RORIM_W { w: self }
    }
    #[doc = "Bit 1 - SSI Receive Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rtim(&mut self) -> SSI_IM_RTIM_W {
        SSI_IM_RTIM_W { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_rxim(&mut self) -> SSI_IM_RXIM_W {
        SSI_IM_RXIM_W { w: self }
    }
    #[doc = "Bit 3 - SSI Transmit FIFO Interrupt Mask"]
    #[inline(always)]
    pub fn ssi_im_txim(&mut self) -> SSI_IM_TXIM_W {
        SSI_IM_TXIM_W { w: self }
    }
}
