#[doc = "Register `PWMDT10` reader"]
pub type R = crate::R<Pwmdt10Spec>;
#[doc = "Register `PWMDT10` writer"]
pub type W = crate::W<Pwmdt10Spec>;
#[doc = "Field `PWMDT10` reader - "]
pub type Pwmdt10R = crate::FieldReader<u16>;
#[doc = "Field `PWMDT10` writer - "]
pub type Pwmdt10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt10(&self) -> Pwmdt10R {
        Pwmdt10R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMDT10")
            .field("rev0", &self.rev0())
            .field("pwmdt10", &self.pwmdt10())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmdt10(&mut self) -> Pwmdt10W<'_, Pwmdt10Spec> {
        Pwmdt10W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmdt10Spec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDT10\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdt10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdt10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmdt10Spec;
impl crate::RegisterSpec for Pwmdt10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdt10::R`](R) reader structure"]
impl crate::Readable for Pwmdt10Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmdt10::W`](W) writer structure"]
impl crate::Writable for Pwmdt10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDT10 to value 0"]
impl crate::Resettable for Pwmdt10Spec {}
