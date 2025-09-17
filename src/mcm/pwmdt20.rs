#[doc = "Register `PWMDT20` reader"]
pub type R = crate::R<Pwmdt20Spec>;
#[doc = "Register `PWMDT20` writer"]
pub type W = crate::W<Pwmdt20Spec>;
#[doc = "Field `PWMDT20` reader - "]
pub type Pwmdt20R = crate::FieldReader<u16>;
#[doc = "Field `PWMDT20` writer - "]
pub type Pwmdt20W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt20(&self) -> Pwmdt20R {
        Pwmdt20R::new((self.bits & 0xffff) as u16)
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
    pub fn pwmdt20(&mut self) -> Pwmdt20W<'_, Pwmdt20Spec> {
        Pwmdt20W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmdt20Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDT20\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmdt20Spec;
impl crate::RegisterSpec for Pwmdt20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdt20::R`](R) reader structure"]
impl crate::Readable for Pwmdt20Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmdt20::W`](W) writer structure"]
impl crate::Writable for Pwmdt20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDT20 to value 0"]
impl crate::Resettable for Pwmdt20Spec {}
