#[doc = "Reader of register TEST"]
pub type R = crate::R<u32, super::TEST>;
#[doc = "Writer for register TEST"]
pub type W = crate::W<u32, super::TEST>;
#[doc = "Register TEST `reset()`'s with value 0"]
impl crate::ResetValue for super::TEST {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDT_TEST_STALL`"]
pub type WDT_TEST_STALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WDT_TEST_STALL`"]
pub struct WDT_TEST_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_TEST_STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Watchdog Stall Enable"]
    #[inline(always)]
    pub fn wdt_test_stall(&self) -> WDT_TEST_STALL_R {
        WDT_TEST_STALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Watchdog Stall Enable"]
    #[inline(always)]
    pub fn wdt_test_stall(&mut self) -> WDT_TEST_STALL_W {
        WDT_TEST_STALL_W { w: self }
    }
}
