#[doc = "Register `PWMDR` reader"]
pub type R = crate::R<PwmdrSpec>;
#[doc = "Register `PWMDR` writer"]
pub type W = crate::W<PwmdrSpec>;
#[doc = "Field `PD` reader - "]
pub type PdR = crate::FieldReader<u16>;
#[doc = "Field `PD` writer - "]
pub type PdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMDR")
            .field("rev0", &self.rev0())
            .field("pd", &self.pd())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pd(&mut self) -> PdW<'_, PwmdrSpec> {
        PdW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmdrSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmdrSpec;
impl crate::RegisterSpec for PwmdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdr::R`](R) reader structure"]
impl crate::Readable for PwmdrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmdr::W`](W) writer structure"]
impl crate::Writable for PwmdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDR to value 0"]
impl crate::Resettable for PwmdrSpec {}
