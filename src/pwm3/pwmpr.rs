#[doc = "Register `PWMPR` reader"]
pub type R = crate::R<PwmprSpec>;
#[doc = "Register `PWMPR` writer"]
pub type W = crate::W<PwmprSpec>;
#[doc = "Field `PP` reader - "]
pub type PpR = crate::FieldReader<u16>;
#[doc = "Field `PP` writer - "]
pub type PpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pp(&self) -> PpR {
        PpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMPR")
            .field("rev0", &self.rev0())
            .field("pp", &self.pp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pp(&mut self) -> PpW<'_, PwmprSpec> {
        PpW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmprSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMPR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmprSpec;
impl crate::RegisterSpec for PwmprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmpr::R`](R) reader structure"]
impl crate::Readable for PwmprSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmpr::W`](W) writer structure"]
impl crate::Writable for PwmprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMPR to value 0"]
impl crate::Resettable for PwmprSpec {}
