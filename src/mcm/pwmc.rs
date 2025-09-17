#[doc = "Register `PWMC` reader"]
pub type R = crate::R<PwmcSpec>;
#[doc = "Register `PWMC` writer"]
pub type W = crate::W<PwmcSpec>;
#[doc = "Field `PWMC` reader - "]
pub type PwmcR = crate::FieldReader<u16>;
#[doc = "Field `PWMC` writer - "]
pub type PwmcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmc(&self) -> PwmcR {
        PwmcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmc(&mut self) -> PwmcW<'_, PwmcSpec> {
        PwmcW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmcSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMC\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmcSpec;
impl crate::RegisterSpec for PwmcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmc::R`](R) reader structure"]
impl crate::Readable for PwmcSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmc::W`](W) writer structure"]
impl crate::Writable for PwmcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMC to value 0"]
impl crate::Resettable for PwmcSpec {}
