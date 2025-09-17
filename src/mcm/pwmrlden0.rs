#[doc = "Register `PWMRLDEN0` reader"]
pub type R = crate::R<Pwmrlden0Spec>;
#[doc = "Register `PWMRLDEN0` writer"]
pub type W = crate::W<Pwmrlden0Spec>;
#[doc = "Field `PWMRLDEN0` reader - "]
pub type Pwmrlden0R = crate::FieldReader;
#[doc = "Field `PWMRLDEN0` writer - "]
pub type Pwmrlden0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pwmrlden0(&self) -> Pwmrlden0R {
        Pwmrlden0R::new((self.bits & 0xff) as u8)
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
    pub fn pwmrlden0(&mut self) -> Pwmrlden0W<'_, Pwmrlden0Spec> {
        Pwmrlden0W::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmrlden0Spec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "PWMRLDEN0\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmrlden0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmrlden0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmrlden0Spec;
impl crate::RegisterSpec for Pwmrlden0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmrlden0::R`](R) reader structure"]
impl crate::Readable for Pwmrlden0Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmrlden0::W`](W) writer structure"]
impl crate::Writable for Pwmrlden0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMRLDEN0 to value 0"]
impl crate::Resettable for Pwmrlden0Spec {}
