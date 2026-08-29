#[doc = "Register `TMSR` reader"]
pub type R = crate::R<TmsrSpec>;
#[doc = "Register `TMSR` writer"]
pub type W = crate::W<TmsrSpec>;
#[doc = "Field `TMR` reader - "]
pub type TmrR = crate::FieldReader;
#[doc = "Field `TMR` writer - "]
pub type TmrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tmr(&self) -> TmrR {
        TmrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMSR")
            .field("rev0", &self.rev0())
            .field("tmr", &self.tmr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tmr(&mut self) -> TmrW<'_, TmsrSpec> {
        TmrW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, TmsrSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "TMSR\n\nYou can [`read`](crate::Reg::read) this register and get [`tmsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmsrSpec;
impl crate::RegisterSpec for TmsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmsr::R`](R) reader structure"]
impl crate::Readable for TmsrSpec {}
#[doc = "`write(|w| ..)` method takes [`tmsr::W`](W) writer structure"]
impl crate::Writable for TmsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TMSR to value 0"]
impl crate::Resettable for TmsrSpec {}
