#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SPR` reader - "]
pub type SprR = crate::FieldReader;
#[doc = "Field `SPR` writer - "]
pub type SprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SSDIS` reader - "]
pub type SsdisR = crate::BitReader;
#[doc = "Field `SSDIS` writer - "]
pub type SsdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - "]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - "]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - "]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - "]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSTR` reader - "]
pub type MstrR = crate::BitReader;
#[doc = "Field `MSTR` writer - "]
pub type MstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - "]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - "]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPRIE` reader - "]
pub type SprieR = crate::BitReader;
#[doc = "Field `SPRIE` writer - "]
pub type SprieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPTIE` reader - "]
pub type SptieR = crate::BitReader;
#[doc = "Field `SPTIE` writer - "]
pub type SptieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPIEN` reader - "]
pub type SpienR = crate::BitReader;
#[doc = "Field `SPIEN` writer - "]
pub type SpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPSFF` reader - "]
pub type SpsffR = crate::BitReader;
#[doc = "Field `SPSFF` writer - "]
pub type SpsffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPDATL` reader - "]
pub type SpdatlR = crate::FieldReader;
#[doc = "Field `SPDATL` writer - "]
pub type SpdatlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn spr(&self) -> SprR {
        SprR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ssdis(&self) -> SsdisR {
        SsdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mstr(&self) -> MstrR {
        MstrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sprie(&self) -> SprieR {
        SprieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sptie(&self) -> SptieR {
        SptieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spien(&self) -> SpienR {
        SpienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spsff(&self) -> SpsffR {
        SpsffR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn spdatl(&self) -> SpdatlR {
        SpdatlR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rev0", &self.rev0())
            .field("spdatl", &self.spdatl())
            .field("spsff", &self.spsff())
            .field("spien", &self.spien())
            .field("rev1", &self.rev1())
            .field("rev2", &self.rev2())
            .field("sptie", &self.sptie())
            .field("sprie", &self.sprie())
            .field("rev3", &self.rev3())
            .field("dir", &self.dir())
            .field("mstr", &self.mstr())
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("ssdis", &self.ssdis())
            .field("spr", &self.spr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn spr(&mut self) -> SprW<'_, CrSpec> {
        SprW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ssdis(&mut self) -> SsdisW<'_, CrSpec> {
        SsdisW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, CrSpec> {
        CpolW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, CrSpec> {
        CphaW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mstr(&mut self) -> MstrW<'_, CrSpec> {
        MstrW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, CrSpec> {
        DirW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, CrSpec> {
        Rev3W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sprie(&mut self) -> SprieW<'_, CrSpec> {
        SprieW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sptie(&mut self) -> SptieW<'_, CrSpec> {
        SptieW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CrSpec> {
        Rev2W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CrSpec> {
        Rev1W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spien(&mut self) -> SpienW<'_, CrSpec> {
        SpienW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn spsff(&mut self) -> SpsffW<'_, CrSpec> {
        SpsffW::new(self, 15)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn spdatl(&mut self) -> SpdatlW<'_, CrSpec> {
        SpdatlW::new(self, 16)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CrSpec> {
        Rev0W::new(self, 18)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
