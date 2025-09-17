#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `STR` reader - "]
pub type StrR = crate::BitReader;
#[doc = "Field `STR` writer - "]
pub type StrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev4` reader - "]
pub type Rev4R = crate::FieldReader;
#[doc = "Field `rev4` writer - "]
pub type Rev4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPM` reader - "]
pub type OpmR = crate::BitReader;
#[doc = "Field `OPM` writer - "]
pub type OpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKS` reader - "]
pub type ClksR = crate::FieldReader;
#[doc = "Field `CLKS` writer - "]
pub type ClksW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE` reader - "]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - "]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIGEN` reader - "]
pub type TrigenR = crate::BitReader;
#[doc = "Field `TRIGEN` writer - "]
pub type TrigenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETEN` reader - "]
pub type EtenR = crate::BitReader;
#[doc = "Field `ETEN` writer - "]
pub type EtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC` reader - "]
pub type TcR = crate::BitReader;
#[doc = "Field `TC` writer - "]
pub type TcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECF` reader - "]
pub type EcfR = crate::FieldReader;
#[doc = "Field `ECF` writer - "]
pub type EcfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CASCEN` reader - "]
pub type CascenR = crate::BitReader;
#[doc = "Field `CASCEN` writer - "]
pub type CascenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn str(&self) -> StrR {
        StrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn opm(&self) -> OpmR {
        OpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clks(&self) -> ClksR {
        ClksR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trigen(&self) -> TrigenR {
        TrigenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn eten(&self) -> EtenR {
        EtenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ecf(&self) -> EcfR {
        EcfR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cascen(&self) -> CascenR {
        CascenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn str(&mut self) -> StrW<'_, CrSpec> {
        StrW::new(self, 0)
    }
    #[doc = "Bits 1:2"]
    #[inline(always)]
    pub fn rev4(&mut self) -> Rev4W<'_, CrSpec> {
        Rev4W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn opm(&mut self) -> OpmW<'_, CrSpec> {
        OpmW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn clks(&mut self) -> ClksW<'_, CrSpec> {
        ClksW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, CrSpec> {
        Rev3W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, CrSpec> {
        IeW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn trigen(&mut self) -> TrigenW<'_, CrSpec> {
        TrigenW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn eten(&mut self) -> EtenW<'_, CrSpec> {
        EtenW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn tc(&mut self) -> TcW<'_, CrSpec> {
        TcW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CrSpec> {
        Rev2W::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn ecf(&mut self) -> EcfW<'_, CrSpec> {
        EcfW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CrSpec> {
        Rev1W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cascen(&mut self) -> CascenW<'_, CrSpec> {
        CascenW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CrSpec> {
        Rev0W::new(self, 16)
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
