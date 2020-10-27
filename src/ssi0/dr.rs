#[doc = "Reader of register DR"]
pub type R = crate::R<u32, super::DR>;
#[doc = "Writer for register DR"]
pub type W = crate::W<u32, super::DR>;
#[doc = "Register DR `reset()`'s with value 0"]
impl crate::ResetValue for super::DR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SSI_DR_DATA`"]
pub type SSI_DR_DATA_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SSI_DR_DATA`"]
pub struct SSI_DR_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_DR_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - SSI Receive/Transmit Data"]
    #[inline(always)]
    pub fn ssi_dr_data(&self) -> SSI_DR_DATA_R {
        SSI_DR_DATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SSI Receive/Transmit Data"]
    #[inline(always)]
    pub fn ssi_dr_data(&mut self) -> SSI_DR_DATA_W {
        SSI_DR_DATA_W { w: self }
    }
}
