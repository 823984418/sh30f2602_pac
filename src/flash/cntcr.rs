#[doc = "Register `CNTCR` reader"]
pub type R = crate::R<CntcrSpec>;
#[doc = "Register `CNTCR` writer"]
pub type W = crate::W<CntcrSpec>;
#[doc = "Field `CNTEN` reader - "]
pub type CntenR = crate::BitReader;
#[doc = "Field `CNTEN` writer - "]
pub type CntenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnten(&self) -> CntenR {
        CntenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTCR")
            .field("rev0", &self.rev0())
            .field("cnten", &self.cnten())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cnten(&mut self) -> CntenW<'_, CntcrSpec> {
        CntenW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CntcrSpec> {
        Rev0W::new(self, 1)
    }
}
#[doc = "CNTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`cntcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntcrSpec;
impl crate::RegisterSpec for CntcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntcr::R`](R) reader structure"]
impl crate::Readable for CntcrSpec {}
#[doc = "`write(|w| ..)` method takes [`cntcr::W`](W) writer structure"]
impl crate::Writable for CntcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTCR to value 0"]
impl crate::Resettable for CntcrSpec {}
