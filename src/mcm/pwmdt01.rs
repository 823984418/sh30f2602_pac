#[doc = "Register `PWMDT01` reader"]
pub type R = crate::R<Pwmdt01Spec>;
#[doc = "Register `PWMDT01` writer"]
pub type W = crate::W<Pwmdt01Spec>;
#[doc = "Field `PWMDT01` reader - "]
pub type Pwmdt01R = crate::FieldReader<u16>;
#[doc = "Field `PWMDT01` writer - "]
pub type Pwmdt01W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt01(&self) -> Pwmdt01R {
        Pwmdt01R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMDT01")
            .field("rev0", &self.rev0())
            .field("pwmdt01", &self.pwmdt01())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt01(&mut self) -> Pwmdt01W<'_, Pwmdt01Spec> {
        Pwmdt01W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmdt01Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDT01\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmdt01Spec;
impl crate::RegisterSpec for Pwmdt01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdt01::R`](R) reader structure"]
impl crate::Readable for Pwmdt01Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmdt01::W`](W) writer structure"]
impl crate::Writable for Pwmdt01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDT01 to value 0"]
impl crate::Resettable for Pwmdt01Spec {}
