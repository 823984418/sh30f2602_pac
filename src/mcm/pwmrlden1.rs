#[doc = "Register `PWMRLDEN1` reader"]
pub type R = crate::R<Pwmrlden1Spec>;
#[doc = "Register `PWMRLDEN1` writer"]
pub type W = crate::W<Pwmrlden1Spec>;
#[doc = "Field `PWMRLDEN1` reader - "]
pub type Pwmrlden1R = crate::FieldReader;
#[doc = "Field `PWMRLDEN1` writer - "]
pub type Pwmrlden1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pwmrlden1(&self) -> Pwmrlden1R {
        Pwmrlden1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pwmrlden1(&mut self) -> Pwmrlden1W<'_, Pwmrlden1Spec> {
        Pwmrlden1W::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmrlden1Spec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "PWMRLDEN1\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmrlden1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmrlden1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmrlden1Spec;
impl crate::RegisterSpec for Pwmrlden1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmrlden1::R`](R) reader structure"]
impl crate::Readable for Pwmrlden1Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmrlden1::W`](W) writer structure"]
impl crate::Writable for Pwmrlden1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMRLDEN1 to value 0"]
impl crate::Resettable for Pwmrlden1Spec {}
