#[doc = "Register `CMP0CON` reader"]
pub type R = crate::R<Cmp0conSpec>;
#[doc = "Register `CMP0CON` writer"]
pub type W = crate::W<Cmp0conSpec>;
#[doc = "Field `C0DEB` reader - "]
pub type C0debR = crate::FieldReader;
#[doc = "Field `C0DEB` writer - "]
pub type C0debW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C0OUT` reader - "]
pub type C0outR = crate::BitReader;
#[doc = "Field `OP0PCHS` reader - "]
pub type Op0pchsR = crate::FieldReader;
#[doc = "Field `OP0PCHS` writer - "]
pub type Op0pchsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev4` reader - "]
pub type Rev4R = crate::BitReader;
#[doc = "Field `rev4` writer - "]
pub type Rev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP0NCHS` reader - "]
pub type Op0nchsR = crate::FieldReader;
#[doc = "Field `OP0NCHS` writer - "]
pub type Op0nchsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C0IES` reader - "]
pub type C0iesR = crate::FieldReader;
#[doc = "Field `C0IES` writer - "]
pub type C0iesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C0OUTEN` reader - "]
pub type C0outenR = crate::BitReader;
#[doc = "Field `C0OUTEN` writer - "]
pub type C0outenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::FieldReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP0EN` reader - "]
pub type Cmp0enR = crate::BitReader;
#[doc = "Field `CMP0EN` writer - "]
pub type Cmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHSUMOUTEN` reader - "]
pub type ChsumoutenR = crate::BitReader;
#[doc = "Field `CHSUMOUTEN` writer - "]
pub type ChsumoutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSUMCHS` reader - "]
pub type VsumchsR = crate::FieldReader;
#[doc = "Field `VSUMCHS` writer - "]
pub type VsumchsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMTRGS` reader - "]
pub type PwmtrgsR = crate::FieldReader;
#[doc = "Field `PWMTRGS` writer - "]
pub type PwmtrgsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGPOL` reader - "]
pub type TrgpolR = crate::BitReader;
#[doc = "Field `TRGPOL` writer - "]
pub type TrgpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMP0VREF` reader - "]
pub type Cmp0vrefR = crate::FieldReader;
#[doc = "Field `CMP0VREF` writer - "]
pub type Cmp0vrefW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CP0NOUTEN` reader - "]
pub type Cp0noutenR = crate::BitReader;
#[doc = "Field `CP0NOUTEN` writer - "]
pub type Cp0noutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn c0deb(&self) -> C0debR {
        C0debR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c0out(&self) -> C0outR {
        C0outR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn op0pchs(&self) -> Op0pchsR {
        Op0pchsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn op0nchs(&self) -> Op0nchsR {
        Op0nchsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn c0ies(&self) -> C0iesR {
        C0iesR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn c0outen(&self) -> C0outenR {
        C0outenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cmp0en(&self) -> Cmp0enR {
        Cmp0enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn chsumouten(&self) -> ChsumoutenR {
        ChsumoutenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn vsumchs(&self) -> VsumchsR {
        VsumchsR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pwmtrgs(&self) -> PwmtrgsR {
        PwmtrgsR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn trgpol(&self) -> TrgpolR {
        TrgpolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn cmp0vref(&self) -> Cmp0vrefR {
        Cmp0vrefR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cp0nouten(&self) -> Cp0noutenR {
        Cp0noutenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP0CON")
            .field("rev0", &self.rev0())
            .field("cp0nouten", &self.cp0nouten())
            .field("cmp0vref", &self.cmp0vref())
            .field("rev1", &self.rev1())
            .field("trgpol", &self.trgpol())
            .field("pwmtrgs", &self.pwmtrgs())
            .field("rev2", &self.rev2())
            .field("vsumchs", &self.vsumchs())
            .field("chsumouten", &self.chsumouten())
            .field("cmp0en", &self.cmp0en())
            .field("rev3", &self.rev3())
            .field("c0outen", &self.c0outen())
            .field("c0ies", &self.c0ies())
            .field("op0nchs", &self.op0nchs())
            .field("rev4", &self.rev4())
            .field("op0pchs", &self.op0pchs())
            .field("c0out", &self.c0out())
            .field("c0deb", &self.c0deb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn c0deb(&mut self) -> C0debW<'_, Cmp0conSpec> {
        C0debW::new(self, 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn op0pchs(&mut self) -> Op0pchsW<'_, Cmp0conSpec> {
        Op0pchsW::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rev4(&mut self) -> Rev4W<'_, Cmp0conSpec> {
        Rev4W::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn op0nchs(&mut self) -> Op0nchsW<'_, Cmp0conSpec> {
        Op0nchsW::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn c0ies(&mut self) -> C0iesW<'_, Cmp0conSpec> {
        C0iesW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn c0outen(&mut self) -> C0outenW<'_, Cmp0conSpec> {
        C0outenW::new(self, 12)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, Cmp0conSpec> {
        Rev3W::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cmp0en(&mut self) -> Cmp0enW<'_, Cmp0conSpec> {
        Cmp0enW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn chsumouten(&mut self) -> ChsumoutenW<'_, Cmp0conSpec> {
        ChsumoutenW::new(self, 16)
    }
    #[doc = "Bits 17:18"]
    #[inline(always)]
    pub fn vsumchs(&mut self) -> VsumchsW<'_, Cmp0conSpec> {
        VsumchsW::new(self, 17)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, Cmp0conSpec> {
        Rev2W::new(self, 19)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn pwmtrgs(&mut self) -> PwmtrgsW<'_, Cmp0conSpec> {
        PwmtrgsW::new(self, 20)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn trgpol(&mut self) -> TrgpolW<'_, Cmp0conSpec> {
        TrgpolW::new(self, 22)
    }
    #[doc = "Bits 23:25"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Cmp0conSpec> {
        Rev1W::new(self, 23)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn cmp0vref(&mut self) -> Cmp0vrefW<'_, Cmp0conSpec> {
        Cmp0vrefW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cp0nouten(&mut self) -> Cp0noutenW<'_, Cmp0conSpec> {
        Cp0noutenW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Cmp0conSpec> {
        Rev0W::new(self, 31)
    }
}
#[doc = "CMP0CON\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp0con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp0con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp0conSpec;
impl crate::RegisterSpec for Cmp0conSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp0con::R`](R) reader structure"]
impl crate::Readable for Cmp0conSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp0con::W`](W) writer structure"]
impl crate::Writable for Cmp0conSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP0CON to value 0"]
impl crate::Resettable for Cmp0conSpec {}
