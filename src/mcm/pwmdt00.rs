#[doc = "Register `PWMDT00` reader"]
pub type R = crate::R<Pwmdt00Spec>;
#[doc = "Register `PWMDT00` writer"]
pub type W = crate::W<Pwmdt00Spec>;
#[doc = "Field `PWMDT00` reader - "]
pub type Pwmdt00R = crate::FieldReader<u16>;
#[doc = "Field `PWMDT00` writer - "]
pub type Pwmdt00W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt00(&self) -> Pwmdt00R {
        Pwmdt00R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMDT00")
            .field("rev0", &self.rev0())
            .field("pwmdt00", &self.pwmdt00())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt00(&mut self) -> Pwmdt00W<'_, Pwmdt00Spec> {
        Pwmdt00W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmdt00Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDT00\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmdt00Spec;
impl crate::RegisterSpec for Pwmdt00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdt00::R`](R) reader structure"]
impl crate::Readable for Pwmdt00Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmdt00::W`](W) writer structure"]
impl crate::Writable for Pwmdt00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDT00 to value 0"]
impl crate::Resettable for Pwmdt00Spec {}
