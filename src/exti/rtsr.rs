#[doc = "Register `RTSR` reader"]
pub type R = crate::R<RtsrSpec>;
#[doc = "Register `RTSR` writer"]
pub type W = crate::W<RtsrSpec>;
#[doc = "Field `RTR` reader - "]
pub type RtrR = crate::FieldReader;
#[doc = "Field `RTR` writer - "]
pub type RtrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new((self.bits & 0xff) as u8)
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
    pub fn rtr(&mut self) -> RtrW<'_, RtsrSpec> {
        RtrW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, RtsrSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "RTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtsrSpec;
impl crate::RegisterSpec for RtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr::R`](R) reader structure"]
impl crate::Readable for RtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtsr::W`](W) writer structure"]
impl crate::Writable for RtsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTSR to value 0"]
impl crate::Resettable for RtsrSpec {}
