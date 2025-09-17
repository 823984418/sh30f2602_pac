#[doc = "Register `PWMCMP3` reader"]
pub type R = crate::R<Pwmcmp3Spec>;
#[doc = "Register `PWMCMP3` writer"]
pub type W = crate::W<Pwmcmp3Spec>;
#[doc = "Field `PWMCMP3` reader - "]
pub type Pwmcmp3R = crate::FieldReader<u16>;
#[doc = "Field `PWMCMP3` writer - "]
pub type Pwmcmp3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmcmp3(&self) -> Pwmcmp3R {
        Pwmcmp3R::new((self.bits & 0xffff) as u16)
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
    pub fn pwmcmp3(&mut self) -> Pwmcmp3W<'_, Pwmcmp3Spec> {
        Pwmcmp3W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmcmp3Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMCMP3\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcmp3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcmp3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmcmp3Spec;
impl crate::RegisterSpec for Pwmcmp3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcmp3::R`](R) reader structure"]
impl crate::Readable for Pwmcmp3Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmcmp3::W`](W) writer structure"]
impl crate::Writable for Pwmcmp3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMCMP3 to value 0"]
impl crate::Resettable for Pwmcmp3Spec {}
