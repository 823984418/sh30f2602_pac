#[doc = "Register `PWMDT11` reader"]
pub type R = crate::R<Pwmdt11Spec>;
#[doc = "Register `PWMDT11` writer"]
pub type W = crate::W<Pwmdt11Spec>;
#[doc = "Field `PWMDT11` reader - "]
pub type Pwmdt11R = crate::FieldReader<u16>;
#[doc = "Field `PWMDT11` writer - "]
pub type Pwmdt11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt11(&self) -> Pwmdt11R {
        Pwmdt11R::new((self.bits & 0xffff) as u16)
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
    pub fn pwmdt11(&mut self) -> Pwmdt11W<'_, Pwmdt11Spec> {
        Pwmdt11W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmdt11Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDT11\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmdt11Spec;
impl crate::RegisterSpec for Pwmdt11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdt11::R`](R) reader structure"]
impl crate::Readable for Pwmdt11Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmdt11::W`](W) writer structure"]
impl crate::Writable for Pwmdt11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDT11 to value 0"]
impl crate::Resettable for Pwmdt11Spec {}
