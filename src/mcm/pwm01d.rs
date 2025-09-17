#[doc = "Register `PWM01D` reader"]
pub type R = crate::R<Pwm01dSpec>;
#[doc = "Register `PWM01D` writer"]
pub type W = crate::W<Pwm01dSpec>;
#[doc = "Field `PWM01D` reader - "]
pub type Pwm01dR = crate::FieldReader<u16>;
#[doc = "Field `PWM01D` writer - "]
pub type Pwm01dW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwm01d(&self) -> Pwm01dR {
        Pwm01dR::new((self.bits & 0xffff) as u16)
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
    pub fn pwm01d(&mut self) -> Pwm01dW<'_, Pwm01dSpec> {
        Pwm01dW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwm01dSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWM01D\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm01d::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm01d::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwm01dSpec;
impl crate::RegisterSpec for Pwm01dSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm01d::R`](R) reader structure"]
impl crate::Readable for Pwm01dSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm01d::W`](W) writer structure"]
impl crate::Writable for Pwm01dSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM01D to value 0"]
impl crate::Resettable for Pwm01dSpec {}
