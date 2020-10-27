#[doc = "Reader of register SR"]
pub type R = crate::R<u32, super::SR>;
#[doc = "Writer for register SR"]
pub type W = crate::W<u32, super::SR>;
#[doc = "Register SR `reset()`'s with value 0"]
impl crate::ResetValue for super::SR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSI_SR_TFE`"]
pub type SSI_SR_TFE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_SR_TFE`"]
pub struct SSI_SR_TFE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_SR_TFE_W<'a> {
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
#[doc = "Reader of field `SSI_SR_TNF`"]
pub type SSI_SR_TNF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_SR_TNF`"]
pub struct SSI_SR_TNF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_SR_TNF_W<'a> {
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
#[doc = "Reader of field `SSI_SR_RNE`"]
pub type SSI_SR_RNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_SR_RNE`"]
pub struct SSI_SR_RNE_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_SR_RNE_W<'a> {
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
#[doc = "Reader of field `SSI_SR_RFF`"]
pub type SSI_SR_RFF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_SR_RFF`"]
pub struct SSI_SR_RFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_SR_RFF_W<'a> {
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
#[doc = "Reader of field `SSI_SR_BSY`"]
pub type SSI_SR_BSY_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI_SR_BSY`"]
pub struct SSI_SR_BSY_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_SR_BSY_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    pub fn ssi_sr_tfe(&self) -> SSI_SR_TFE_R {
        SSI_SR_TFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn ssi_sr_tnf(&self) -> SSI_SR_TNF_R {
        SSI_SR_TNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn ssi_sr_rne(&self) -> SSI_SR_RNE_R {
        SSI_SR_RNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    pub fn ssi_sr_rff(&self) -> SSI_SR_RFF_R {
        SSI_SR_RFF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    pub fn ssi_sr_bsy(&self) -> SSI_SR_BSY_R {
        SSI_SR_BSY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    pub fn ssi_sr_tfe(&mut self) -> SSI_SR_TFE_W {
        SSI_SR_TFE_W { w: self }
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn ssi_sr_tnf(&mut self) -> SSI_SR_TNF_W {
        SSI_SR_TNF_W { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn ssi_sr_rne(&mut self) -> SSI_SR_RNE_W {
        SSI_SR_RNE_W { w: self }
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    pub fn ssi_sr_rff(&mut self) -> SSI_SR_RFF_W {
        SSI_SR_RFF_W { w: self }
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    pub fn ssi_sr_bsy(&mut self) -> SSI_SR_BSY_W {
        SSI_SR_BSY_W { w: self }
    }
}
