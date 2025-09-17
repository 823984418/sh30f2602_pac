#[doc = "Register `PWM2D` reader"]
pub type R = crate::R<Pwm2dSpec>;
#[doc = "Register `PWM2D` writer"]
pub type W = crate::W<Pwm2dSpec>;
#[doc = "Field `PWM2D` reader - "]
pub type Pwm2dR = crate::FieldReader<u16>;
#[doc = "Field `PWM2D` writer - "]
pub type Pwm2dW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm2d(&self) -> Pwm2dR {
        Pwm2dR::new((self.bits & 0xffff) as u16)
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
    pub fn pwm2d(&mut self) -> Pwm2dW<'_, Pwm2dSpec> {
        Pwm2dW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwm2dSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWM2D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm2d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm2d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm2dSpec;
impl crate::RegisterSpec for Pwm2dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm2d::R`](R) reader structure"]
impl crate::Readable for Pwm2dSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm2d::W`](W) writer structure"]
impl crate::Writable for Pwm2dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM2D to value 0"]
impl crate::Resettable for Pwm2dSpec {}
