#[doc = "Register `PWMOE` reader"]
pub type R = crate::R<PwmoeSpec>;
#[doc = "Register `PWMOE` writer"]
pub type W = crate::W<PwmoeSpec>;
#[doc = "Field `PWMOE` reader - "]
pub type PwmoeR = crate::BitReader;
#[doc = "Field `PWMOE` writer - "]
pub type PwmoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwmoe(&self) -> PwmoeR {
        PwmoeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwmoe(&mut self) -> PwmoeW<'_, PwmoeSpec> {
        PwmoeW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmoeSpec> {
        Rev0W::new(self, 1)
    }
}
#[doc = "PWMOE\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmoe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmoe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmoeSpec;
impl crate::RegisterSpec for PwmoeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmoe::R`](R) reader structure"]
impl crate::Readable for PwmoeSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmoe::W`](W) writer structure"]
impl crate::Writable for PwmoeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMOE to value 0"]
impl crate::Resettable for PwmoeSpec {}
