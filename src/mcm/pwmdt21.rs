#[doc = "Register `PWMDT21` reader"]
pub type R = crate::R<Pwmdt21Spec>;
#[doc = "Register `PWMDT21` writer"]
pub type W = crate::W<Pwmdt21Spec>;
#[doc = "Field `PWMDT21` reader - "]
pub type Pwmdt21R = crate::FieldReader<u16>;
#[doc = "Field `PWMDT21` writer - "]
pub type Pwmdt21W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt21(&self) -> Pwmdt21R {
        Pwmdt21R::new((self.bits & 0xffff) as u16)
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
    pub fn pwmdt21(&mut self) -> Pwmdt21W<'_, Pwmdt21Spec> {
        Pwmdt21W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmdt21Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDT21\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmdt21Spec;
impl crate::RegisterSpec for Pwmdt21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdt21::R`](R) reader structure"]
impl crate::Readable for Pwmdt21Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmdt21::W`](W) writer structure"]
impl crate::Writable for Pwmdt21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDT21 to value 0"]
impl crate::Resettable for Pwmdt21Spec {}
