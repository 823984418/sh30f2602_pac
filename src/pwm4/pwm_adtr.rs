#[doc = "Register `PWM_ADTR` reader"]
pub type R = crate::R<PwmAdtrSpec>;
#[doc = "Register `PWM_ADTR` writer"]
pub type W = crate::W<PwmAdtrSpec>;
#[doc = "Field `DATA` reader - "]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - "]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
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
    pub fn data(&mut self) -> DataW<'_, PwmAdtrSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmAdtrSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWM_ADTR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwm_adtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwm_adtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmAdtrSpec;
impl crate::RegisterSpec for PwmAdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwm_adtr::R`](R) reader structure"]
impl crate::Readable for PwmAdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwm_adtr::W`](W) writer structure"]
impl crate::Writable for PwmAdtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWM_ADTR to value 0"]
impl crate::Resettable for PwmAdtrSpec {}
