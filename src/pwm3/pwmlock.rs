#[doc = "Register `PWMLOCK` reader"]
pub type R = crate::R<PwmlockSpec>;
#[doc = "Register `PWMLOCK` writer"]
pub type W = crate::W<PwmlockSpec>;
#[doc = "Field `PWMLO` reader - "]
pub type PwmloR = crate::FieldReader<u16>;
#[doc = "Field `PWMLO` writer - "]
pub type PwmloW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmlo(&self) -> PwmloR {
        PwmloR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMLOCK")
            .field("rev0", &self.rev0())
            .field("pwmlo", &self.pwmlo())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmlo(&mut self) -> PwmloW<'_, PwmlockSpec> {
        PwmloW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmlockSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmlockSpec;
impl crate::RegisterSpec for PwmlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmlock::R`](R) reader structure"]
impl crate::Readable for PwmlockSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmlock::W`](W) writer structure"]
impl crate::Writable for PwmlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMLOCK to value 0"]
impl crate::Resettable for PwmlockSpec {}
