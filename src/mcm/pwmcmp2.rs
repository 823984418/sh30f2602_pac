#[doc = "Register `PWMCMP2` reader"]
pub type R = crate::R<Pwmcmp2Spec>;
#[doc = "Register `PWMCMP2` writer"]
pub type W = crate::W<Pwmcmp2Spec>;
#[doc = "Field `PWMCMP2` reader - "]
pub type Pwmcmp2R = crate::FieldReader<u16>;
#[doc = "Field `PWMCMP2` writer - "]
pub type Pwmcmp2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmcmp2(&self) -> Pwmcmp2R {
        Pwmcmp2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMCMP2")
            .field("rev0", &self.rev0())
            .field("pwmcmp2", &self.pwmcmp2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmcmp2(&mut self) -> Pwmcmp2W<'_, Pwmcmp2Spec> {
        Pwmcmp2W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmcmp2Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMCMP2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmcmp2Spec;
impl crate::RegisterSpec for Pwmcmp2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcmp2::R`](R) reader structure"]
impl crate::Readable for Pwmcmp2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmcmp2::W`](W) writer structure"]
impl crate::Writable for Pwmcmp2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMCMP2 to value 0"]
impl crate::Resettable for Pwmcmp2Spec {}
