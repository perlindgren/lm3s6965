#[doc = "Reader of register LOCK"]
pub type R = crate::R<u32, super::LOCK>;
#[doc = "Writer for register LOCK"]
pub type W = crate::W<u32, super::LOCK>;
#[doc = "Register LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Watchdog Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WDT_LOCK_A {
    #[doc = "0: Unlocked"]
    WDT_LOCK_UNLOCKED = 0,
    #[doc = "1: Locked"]
    WDT_LOCK_LOCKED = 1,
    #[doc = "449635665: Unlocks the watchdog timer"]
    WDT_LOCK_UNLOCK = 449635665,
}
impl From<WDT_LOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: WDT_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WDT_LOCK`"]
pub type WDT_LOCK_R = crate::R<u32, WDT_LOCK_A>;
impl WDT_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, WDT_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(WDT_LOCK_A::WDT_LOCK_UNLOCKED),
            1 => Val(WDT_LOCK_A::WDT_LOCK_LOCKED),
            449635665 => Val(WDT_LOCK_A::WDT_LOCK_UNLOCK),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `WDT_LOCK_UNLOCKED`"]
    #[inline(always)]
    pub fn is_wdt_lock_unlocked(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_UNLOCKED
    }
    #[doc = "Checks if the value of the field is `WDT_LOCK_LOCKED`"]
    #[inline(always)]
    pub fn is_wdt_lock_locked(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_LOCKED
    }
    #[doc = "Checks if the value of the field is `WDT_LOCK_UNLOCK`"]
    #[inline(always)]
    pub fn is_wdt_lock_unlock(&self) -> bool {
        *self == WDT_LOCK_A::WDT_LOCK_UNLOCK
    }
}
#[doc = "Write proxy for field `WDT_LOCK`"]
pub struct WDT_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDT_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Unlocked"]
    #[inline(always)]
    pub fn wdt_lock_unlocked(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::WDT_LOCK_UNLOCKED)
    }
    #[doc = "Locked"]
    #[inline(always)]
    pub fn wdt_lock_locked(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::WDT_LOCK_LOCKED)
    }
    #[doc = "Unlocks the watchdog timer"]
    #[inline(always)]
    pub fn wdt_lock_unlock(self) -> &'a mut W {
        self.variant(WDT_LOCK_A::WDT_LOCK_UNLOCK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdt_lock(&self) -> WDT_LOCK_R {
        WDT_LOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Watchdog Lock"]
    #[inline(always)]
    pub fn wdt_lock(&mut self) -> WDT_LOCK_W {
        WDT_LOCK_W { w: self }
    }
}
