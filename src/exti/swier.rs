#[doc = "Register `SWIER` writer"]
pub type W = crate::W<SwierSpec>;
#[doc = "Field `SWIER` writer - "]
pub type SwierW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl core::fmt::Debug for crate::generic::Reg<SwierSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn swier(&mut self) -> SwierW<'_, SwierSpec> {
        SwierW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, SwierSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "SWIER\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwierSpec;
impl crate::RegisterSpec for SwierSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swier::W`](W) writer structure"]
impl crate::Writable for SwierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWIER to value 0"]
impl crate::Resettable for SwierSpec {}
