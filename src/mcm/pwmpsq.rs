#[doc = "Register `PWMPSQ` reader"]
pub type R = crate::R<PwmpsqSpec>;
#[doc = "Register `PWMPSQ` writer"]
pub type W = crate::W<PwmpsqSpec>;
#[doc = "Field `PWMPSQ` reader - "]
pub type PwmpsqR = crate::FieldReader<u16>;
#[doc = "Field `PWMPSQ` writer - "]
pub type PwmpsqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmpsq(&self) -> PwmpsqR {
        PwmpsqR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMPSQ")
            .field("rev0", &self.rev0())
            .field("pwmpsq", &self.pwmpsq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmpsq(&mut self) -> PwmpsqW<'_, PwmpsqSpec> {
        PwmpsqW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmpsqSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMPSQ\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmpsq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmpsq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmpsqSpec;
impl crate::RegisterSpec for PwmpsqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmpsq::R`](R) reader structure"]
impl crate::Readable for PwmpsqSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmpsq::W`](W) writer structure"]
impl crate::Writable for PwmpsqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMPSQ to value 0"]
impl crate::Resettable for PwmpsqSpec {}
