#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Field `LATENCY` reader - "]
pub type LatencyR = crate::FieldReader;
#[doc = "Field `LATENCY` writer - "]
pub type LatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CACHEN` reader - "]
pub type CachenR = crate::BitReader;
#[doc = "Field `CACHEN` writer - "]
pub type CachenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBEN` reader - "]
pub type IbenR = crate::BitReader;
#[doc = "Field `IBEN` writer - "]
pub type IbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBEN` reader - "]
pub type DbenR = crate::BitReader;
#[doc = "Field `DBEN` writer - "]
pub type DbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRST` reader - "]
pub type CrstR = crate::BitReader;
#[doc = "Field `CRST` writer - "]
pub type CrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRFTEN` reader - "]
pub type PrftenR = crate::BitReader;
#[doc = "Field `PRFTEN` writer - "]
pub type PrftenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn latency(&self) -> LatencyR {
        LatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cachen(&self) -> CachenR {
        CachenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn iben(&self) -> IbenR {
        IbenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dben(&self) -> DbenR {
        DbenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn crst(&self) -> CrstR {
        CrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn prften(&self) -> PrftenR {
        PrftenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("lock", &self.lock())
            .field("rev0", &self.rev0())
            .field("prften", &self.prften())
            .field("crst", &self.crst())
            .field("dben", &self.dben())
            .field("iben", &self.iben())
            .field("cachen", &self.cachen())
            .field("rev1", &self.rev1())
            .field("latency", &self.latency())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn latency(&mut self) -> LatencyW<'_, AcrSpec> {
        LatencyW::new(self, 0)
    }
    #[doc = "Bits 3:7"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, AcrSpec> {
        Rev1W::new(self, 3)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cachen(&mut self) -> CachenW<'_, AcrSpec> {
        CachenW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn iben(&mut self) -> IbenW<'_, AcrSpec> {
        IbenW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dben(&mut self) -> DbenW<'_, AcrSpec> {
        DbenW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn crst(&mut self) -> CrstW<'_, AcrSpec> {
        CrstW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn prften(&mut self) -> PrftenW<'_, AcrSpec> {
        PrftenW::new(self, 12)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AcrSpec> {
        Rev0W::new(self, 13)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, AcrSpec> {
        LockW::new(self, 16)
    }
}
#[doc = "ACR\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR to value 0x04"]
impl crate::Resettable for AcrSpec {
    const RESET_VALUE: u32 = 0x04;
}
