#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `CPS` reader - "]
pub type CpsR = crate::FieldReader;
#[doc = "Field `CPS` writer - "]
pub type CpsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SDEN` reader - "]
pub type SdenR = crate::BitReader;
#[doc = "Field `SDEN` writer - "]
pub type SdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIE` reader - "]
pub type CieR = crate::BitReader;
#[doc = "Field `CIE` writer - "]
pub type CieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIE` reader - "]
pub type PieR = crate::BitReader;
#[doc = "Field `PIE` writer - "]
pub type PieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTSEL` reader - "]
pub type AdtselR = crate::FieldReader;
#[doc = "Field `ADTSEL` writer - "]
pub type AdtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC0UE` reader - "]
pub type Cc0ueR = crate::BitReader;
#[doc = "Field `CC0UE` writer - "]
pub type Cc0ueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1UE` reader - "]
pub type Cc1ueR = crate::BitReader;
#[doc = "Field `CC1UE` writer - "]
pub type Cc1ueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2UE` reader - "]
pub type Cc2ueR = crate::BitReader;
#[doc = "Field `CC2UE` writer - "]
pub type Cc2ueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PUE` reader - "]
pub type PueR = crate::BitReader;
#[doc = "Field `PUE` writer - "]
pub type PueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECF` reader - "]
pub type EcfR = crate::FieldReader;
#[doc = "Field `ECF` writer - "]
pub type EcfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYN` reader - "]
pub type SynR = crate::FieldReader;
#[doc = "Field `SYN` writer - "]
pub type SynW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cps(&self) -> CpsR {
        CpsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sden(&self) -> SdenR {
        SdenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cie(&self) -> CieR {
        CieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pie(&self) -> PieR {
        PieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn adtsel(&self) -> AdtselR {
        AdtselR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cc0ue(&self) -> Cc0ueR {
        Cc0ueR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cc1ue(&self) -> Cc1ueR {
        Cc1ueR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cc2ue(&self) -> Cc2ueR {
        Cc2ueR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pue(&self) -> PueR {
        PueR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ecf(&self) -> EcfR {
        EcfR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn syn(&self) -> SynR {
        SynR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("rev0", &self.rev0())
            .field("syn", &self.syn())
            .field("ecf", &self.ecf())
            .field("pue", &self.pue())
            .field("cc2ue", &self.cc2ue())
            .field("cc1ue", &self.cc1ue())
            .field("cc0ue", &self.cc0ue())
            .field("rev1", &self.rev1())
            .field("adtsel", &self.adtsel())
            .field("pie", &self.pie())
            .field("cie", &self.cie())
            .field("sden", &self.sden())
            .field("rev2", &self.rev2())
            .field("cps", &self.cps())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cps(&mut self) -> CpsW<'_, CfgrSpec> {
        CpsW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CfgrSpec> {
        Rev2W::new(self, 3)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sden(&mut self) -> SdenW<'_, CfgrSpec> {
        SdenW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cie(&mut self) -> CieW<'_, CfgrSpec> {
        CieW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn pie(&mut self) -> PieW<'_, CfgrSpec> {
        PieW::new(self, 8)
    }
    #[doc = "Bits 9:10"]
    #[inline(always)]
    pub fn adtsel(&mut self) -> AdtselW<'_, CfgrSpec> {
        AdtselW::new(self, 9)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CfgrSpec> {
        Rev1W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cc0ue(&mut self) -> Cc0ueW<'_, CfgrSpec> {
        Cc0ueW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cc1ue(&mut self) -> Cc1ueW<'_, CfgrSpec> {
        Cc1ueW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cc2ue(&mut self) -> Cc2ueW<'_, CfgrSpec> {
        Cc2ueW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn pue(&mut self) -> PueW<'_, CfgrSpec> {
        PueW::new(self, 15)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn ecf(&mut self) -> EcfW<'_, CfgrSpec> {
        EcfW::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn syn(&mut self) -> SynW<'_, CfgrSpec> {
        SynW::new(self, 18)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CfgrSpec> {
        Rev0W::new(self, 20)
    }
}
#[doc = "CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
