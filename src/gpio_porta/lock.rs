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
#[doc = "GPIO Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum GPIO_LOCK_A {
    #[doc = "0: The GPIOCR register is unlocked and may be modified"]
    GPIO_LOCK_UNLOCKED = 0,
    #[doc = "1: The GPIOCR register is locked and may not be modified"]
    GPIO_LOCK_LOCKED = 1,
    #[doc = "449635665: Unlocks the GPIO_CR register"]
    GPIO_LOCK_KEY = 449635665,
}
impl From<GPIO_LOCK_A> for u32 {
    #[inline(always)]
    fn from(variant: GPIO_LOCK_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `GPIO_LOCK`"]
pub type GPIO_LOCK_R = crate::R<u32, GPIO_LOCK_A>;
impl GPIO_LOCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u32, GPIO_LOCK_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(GPIO_LOCK_A::GPIO_LOCK_UNLOCKED),
            1 => Val(GPIO_LOCK_A::GPIO_LOCK_LOCKED),
            449635665 => Val(GPIO_LOCK_A::GPIO_LOCK_KEY),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `GPIO_LOCK_UNLOCKED`"]
    #[inline(always)]
    pub fn is_gpio_lock_unlocked(&self) -> bool {
        *self == GPIO_LOCK_A::GPIO_LOCK_UNLOCKED
    }
    #[doc = "Checks if the value of the field is `GPIO_LOCK_LOCKED`"]
    #[inline(always)]
    pub fn is_gpio_lock_locked(&self) -> bool {
        *self == GPIO_LOCK_A::GPIO_LOCK_LOCKED
    }
    #[doc = "Checks if the value of the field is `GPIO_LOCK_KEY`"]
    #[inline(always)]
    pub fn is_gpio_lock_key(&self) -> bool {
        *self == GPIO_LOCK_A::GPIO_LOCK_KEY
    }
}
#[doc = "Write proxy for field `GPIO_LOCK`"]
pub struct GPIO_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_LOCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GPIO_LOCK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The GPIOCR register is unlocked and may be modified"]
    #[inline(always)]
    pub fn gpio_lock_unlocked(self) -> &'a mut W {
        self.variant(GPIO_LOCK_A::GPIO_LOCK_UNLOCKED)
    }
    #[doc = "The GPIOCR register is locked and may not be modified"]
    #[inline(always)]
    pub fn gpio_lock_locked(self) -> &'a mut W {
        self.variant(GPIO_LOCK_A::GPIO_LOCK_LOCKED)
    }
    #[doc = "Unlocks the GPIO_CR register"]
    #[inline(always)]
    pub fn gpio_lock_key(self) -> &'a mut W {
        self.variant(GPIO_LOCK_A::GPIO_LOCK_KEY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - GPIO Lock"]
    #[inline(always)]
    pub fn gpio_lock(&self) -> GPIO_LOCK_R {
        GPIO_LOCK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO Lock"]
    #[inline(always)]
    pub fn gpio_lock(&mut self) -> GPIO_LOCK_W {
        GPIO_LOCK_W { w: self }
    }
}
