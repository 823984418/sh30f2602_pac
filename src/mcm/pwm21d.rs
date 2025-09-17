#[doc = "Register `PWM21D` reader"]
pub type R = crate::R<Pwm21dSpec>;
#[doc = "Register `PWM21D` writer"]
pub type W = crate::W<Pwm21dSpec>;
#[doc = "Field `PWM21D` reader - "]
pub type Pwm21dR = crate::FieldReader<u16>;
#[doc = "Field `PWM21D` writer - "]
pub type Pwm21dW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm21d(&self) -> Pwm21dR {
        Pwm21dR::new((self.bits & 0xffff) as u16)
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
    pub fn pwm21d(&mut self) -> Pwm21dW<'_, Pwm21dSpec> {
        Pwm21dW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwm21dSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWM21D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm21d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm21d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm21dSpec;
impl crate::RegisterSpec for Pwm21dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm21d::R`](R) reader structure"]
impl crate::Readable for Pwm21dSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm21d::W`](W) writer structure"]
impl crate::Writable for Pwm21dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM21D to value 0"]
impl crate::Resettable for Pwm21dSpec {}
