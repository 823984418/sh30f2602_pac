#[doc = "Register `TPR` reader"]
pub type R = crate::R<TprSpec>;
#[doc = "Register `TPR` writer"]
pub type W = crate::W<TprSpec>;
#[doc = "Field `TPRL` reader - "]
pub type TprlR = crate::FieldReader<u16>;
#[doc = "Field `TPRL` writer - "]
pub type TprlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TPRH` reader - "]
pub type TprhR = crate::FieldReader<u16>;
#[doc = "Field `TPRH` writer - "]
pub type TprhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tprl(&self) -> TprlR {
        TprlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tprh(&self) -> TprhR {
        TprhR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tprl(&mut self) -> TprlW<'_, TprSpec> {
        TprlW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tprh(&mut self) -> TprhW<'_, TprSpec> {
        TprhW::new(self, 16)
    }
}
#[doc = "TPR\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TprSpec;
impl crate::RegisterSpec for TprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr::R`](R) reader structure"]
impl crate::Readable for TprSpec {}
#[doc = "`write(|w| ..)` method takes [`tpr::W`](W) writer structure"]
impl crate::Writable for TprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPR to value 0"]
impl crate::Resettable for TprSpec {}
