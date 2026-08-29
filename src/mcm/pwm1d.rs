#[doc = "Register `PWM1D` reader"]
pub type R = crate::R<Pwm1dSpec>;
#[doc = "Register `PWM1D` writer"]
pub type W = crate::W<Pwm1dSpec>;
#[doc = "Field `PWM1D` reader - "]
pub type Pwm1dR = crate::FieldReader<u16>;
#[doc = "Field `PWM1D` writer - "]
pub type Pwm1dW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm1d(&self) -> Pwm1dR {
        Pwm1dR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWM1D")
            .field("rev0", &self.rev0())
            .field("pwm1d", &self.pwm1d())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm1d(&mut self) -> Pwm1dW<'_, Pwm1dSpec> {
        Pwm1dW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwm1dSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWM1D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm1d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm1d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm1dSpec;
impl crate::RegisterSpec for Pwm1dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm1d::R`](R) reader structure"]
impl crate::Readable for Pwm1dSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm1d::W`](W) writer structure"]
impl crate::Writable for Pwm1dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM1D to value 0"]
impl crate::Resettable for Pwm1dSpec {}
