#[doc = "Register `PWMCMP4` reader"]
pub type R = crate::R<Pwmcmp4Spec>;
#[doc = "Register `PWMCMP4` writer"]
pub type W = crate::W<Pwmcmp4Spec>;
#[doc = "Field `PWMCMP4` reader - "]
pub type Pwmcmp4R = crate::FieldReader<u16>;
#[doc = "Field `PWMCMP4` writer - "]
pub type Pwmcmp4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmcmp4(&self) -> Pwmcmp4R {
        Pwmcmp4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMCMP4")
            .field("rev0", &self.rev0())
            .field("pwmcmp4", &self.pwmcmp4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmcmp4(&mut self) -> Pwmcmp4W<'_, Pwmcmp4Spec> {
        Pwmcmp4W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmcmp4Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMCMP4\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmcmp4Spec;
impl crate::RegisterSpec for Pwmcmp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcmp4::R`](R) reader structure"]
impl crate::Readable for Pwmcmp4Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmcmp4::W`](W) writer structure"]
impl crate::Writable for Pwmcmp4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMCMP4 to value 0"]
impl crate::Resettable for Pwmcmp4Spec {}
