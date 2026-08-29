#[doc = "Register `PWMCMP1` reader"]
pub type R = crate::R<Pwmcmp1Spec>;
#[doc = "Register `PWMCMP1` writer"]
pub type W = crate::W<Pwmcmp1Spec>;
#[doc = "Field `PWMCMP1` reader - "]
pub type Pwmcmp1R = crate::FieldReader<u16>;
#[doc = "Field `PWMCMP1` writer - "]
pub type Pwmcmp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmcmp1(&self) -> Pwmcmp1R {
        Pwmcmp1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMCMP1")
            .field("rev0", &self.rev0())
            .field("pwmcmp1", &self.pwmcmp1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmcmp1(&mut self) -> Pwmcmp1W<'_, Pwmcmp1Spec> {
        Pwmcmp1W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmcmp1Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMCMP1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmcmp1Spec;
impl crate::RegisterSpec for Pwmcmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcmp1::R`](R) reader structure"]
impl crate::Readable for Pwmcmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmcmp1::W`](W) writer structure"]
impl crate::Writable for Pwmcmp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMCMP1 to value 0"]
impl crate::Resettable for Pwmcmp1Spec {}
