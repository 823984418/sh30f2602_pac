#[doc = "Register `CNT` reader"]
pub type R = crate::R<CntSpec>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CntSpec>;
#[doc = "Field `CNTL` reader - "]
pub type CntlR = crate::FieldReader<u16>;
#[doc = "Field `CNTL` writer - "]
pub type CntlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNTH` reader - "]
pub type CnthR = crate::FieldReader<u16>;
#[doc = "Field `CNTH` writer - "]
pub type CnthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cntl(&self) -> CntlR {
        CntlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cnth(&self) -> CnthR {
        CnthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNT")
            .field("cnth", &self.cnth())
            .field("cntl", &self.cntl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cntl(&mut self) -> CntlW<'_, CntSpec> {
        CntlW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cnth(&mut self) -> CnthW<'_, CntSpec> {
        CnthW::new(self, 16)
    }
}
#[doc = "CNT\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntSpec;
impl crate::RegisterSpec for CntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CntSpec {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CntSpec {}
