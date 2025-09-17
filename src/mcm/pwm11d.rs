#[doc = "Register `PWM11D` reader"]
pub type R = crate::R<Pwm11dSpec>;
#[doc = "Register `PWM11D` writer"]
pub type W = crate::W<Pwm11dSpec>;
#[doc = "Field `PWM11D` reader - "]
pub type Pwm11dR = crate::FieldReader<u16>;
#[doc = "Field `PWM11D` writer - "]
pub type Pwm11dW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm11d(&self) -> Pwm11dR {
        Pwm11dR::new((self.bits & 0xffff) as u16)
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
    pub fn pwm11d(&mut self) -> Pwm11dW<'_, Pwm11dSpec> {
        Pwm11dW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwm11dSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWM11D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm11d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm11d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm11dSpec;
impl crate::RegisterSpec for Pwm11dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm11d::R`](R) reader structure"]
impl crate::Readable for Pwm11dSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm11d::W`](W) writer structure"]
impl crate::Writable for Pwm11dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM11D to value 0"]
impl crate::Resettable for Pwm11dSpec {}
