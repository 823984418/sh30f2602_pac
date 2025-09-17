#[doc = "Register `PWMREMAP` reader"]
pub type R = crate::R<PwmremapSpec>;
#[doc = "Register `PWMREMAP` writer"]
pub type W = crate::W<PwmremapSpec>;
#[doc = "Field `PWMIO` reader - "]
pub type PwmioR = crate::FieldReader;
#[doc = "Field `PWMIO` writer - "]
pub type PwmioW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pwmio(&self) -> PwmioR {
        PwmioR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn pwmio(&mut self) -> PwmioW<'_, PwmremapSpec> {
        PwmioW::new(self, 0)
    }
    #[doc = "Bits 3:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmremapSpec> {
        Rev0W::new(self, 3)
    }
}
#[doc = "PWMREMAP\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmremap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmremap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmremapSpec;
impl crate::RegisterSpec for PwmremapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmremap::R`](R) reader structure"]
impl crate::Readable for PwmremapSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmremap::W`](W) writer structure"]
impl crate::Writable for PwmremapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMREMAP to value 0"]
impl crate::Resettable for PwmremapSpec {}
