#[doc = "Register `PWM0D` reader"]
pub type R = crate::R<Pwm0dSpec>;
#[doc = "Register `PWM0D` writer"]
pub type W = crate::W<Pwm0dSpec>;
#[doc = "Field `PWM0D` reader - "]
pub type Pwm0dR = crate::FieldReader<u16>;
#[doc = "Field `PWM0D` writer - "]
pub type Pwm0dW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm0d(&self) -> Pwm0dR {
        Pwm0dR::new((self.bits & 0xffff) as u16)
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
    pub fn pwm0d(&mut self) -> Pwm0dW<'_, Pwm0dSpec> {
        Pwm0dW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwm0dSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWM0D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm0d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm0d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm0dSpec;
impl crate::RegisterSpec for Pwm0dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm0d::R`](R) reader structure"]
impl crate::Readable for Pwm0dSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm0d::W`](W) writer structure"]
impl crate::Writable for Pwm0dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM0D to value 0"]
impl crate::Resettable for Pwm0dSpec {}
