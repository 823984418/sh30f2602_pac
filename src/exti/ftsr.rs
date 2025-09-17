#[doc = "Register `FTSR` reader"]
pub type R = crate::R<FtsrSpec>;
#[doc = "Register `FTSR` writer"]
pub type W = crate::W<FtsrSpec>;
#[doc = "Field `FTR` reader - "]
pub type FtrR = crate::FieldReader;
#[doc = "Field `FTR` writer - "]
pub type FtrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ftr(&self) -> FtrR {
        FtrR::new((self.bits & 0xff) as u8)
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
    pub fn ftr(&mut self) -> FtrW<'_, FtsrSpec> {
        FtrW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, FtsrSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "FTSR\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FtsrSpec;
impl crate::RegisterSpec for FtsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr::R`](R) reader structure"]
impl crate::Readable for FtsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ftsr::W`](W) writer structure"]
impl crate::Writable for FtsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTSR to value 0"]
impl crate::Resettable for FtsrSpec {}
