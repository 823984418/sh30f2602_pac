#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `PWMEN` reader - "]
pub type PwmenR = crate::BitReader;
#[doc = "Field `PWMEN` writer - "]
pub type PwmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCK` reader - "]
pub type TckR = crate::FieldReader;
#[doc = "Field `TCK` writer - "]
pub type TckW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FLTC` reader - "]
pub type FltcR = crate::BitReader;
#[doc = "Field `FLTC` writer - "]
pub type FltcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFLT` reader - "]
pub type EfltR = crate::BitReader;
#[doc = "Field `EFLT` writer - "]
pub type EfltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSB` reader - "]
pub type PwmsbR = crate::BitReader;
#[doc = "Field `PWMSB` writer - "]
pub type PwmsbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMSA` reader - "]
pub type PwmsaR = crate::BitReader;
#[doc = "Field `PWMSA` writer - "]
pub type PwmsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFLTIE` reader - "]
pub type EfltieR = crate::BitReader;
#[doc = "Field `EFLTIE` writer - "]
pub type EfltieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMIE` reader - "]
pub type PwmieR = crate::BitReader;
#[doc = "Field `PWMIE` writer - "]
pub type PwmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADTEN` reader - "]
pub type AdtenR = crate::BitReader;
#[doc = "Field `ADTEN` writer - "]
pub type AdtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0S` reader - "]
pub type Flt0sR = crate::BitReader;
#[doc = "Field `FLT0S` writer - "]
pub type Flt0sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1S` reader - "]
pub type Flt1sR = crate::BitReader;
#[doc = "Field `FLT1S` writer - "]
pub type Flt1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2S` reader - "]
pub type Flt2sR = crate::BitReader;
#[doc = "Field `FLT2S` writer - "]
pub type Flt2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0EN` reader - "]
pub type Flt0enR = crate::BitReader;
#[doc = "Field `FLT0EN` writer - "]
pub type Flt0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1EN` reader - "]
pub type Flt1enR = crate::BitReader;
#[doc = "Field `FLT1EN` writer - "]
pub type Flt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2EN` reader - "]
pub type Flt2enR = crate::BitReader;
#[doc = "Field `FLT2EN` writer - "]
pub type Flt2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0IE` reader - "]
pub type Flt0ieR = crate::BitReader;
#[doc = "Field `FLT0IE` writer - "]
pub type Flt0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1IE` reader - "]
pub type Flt1ieR = crate::BitReader;
#[doc = "Field `FLT1IE` writer - "]
pub type Flt1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2IE` reader - "]
pub type Flt2ieR = crate::BitReader;
#[doc = "Field `FLT2IE` writer - "]
pub type Flt2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOUTB` reader - "]
pub type FoutbR = crate::FieldReader;
#[doc = "Field `FOUTB` writer - "]
pub type FoutbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FOUTA` reader - "]
pub type FoutaR = crate::FieldReader;
#[doc = "Field `FOUTA` writer - "]
pub type FoutaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLTDEB` reader - "]
pub type FltdebR = crate::FieldReader;
#[doc = "Field `FLTDEB` writer - "]
pub type FltdebW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwmen(&self) -> PwmenR {
        PwmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn tck(&self) -> TckR {
        TckR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fltc(&self) -> FltcR {
        FltcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn eflt(&self) -> EfltR {
        EfltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwmsb(&self) -> PwmsbR {
        PwmsbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwmsa(&self) -> PwmsaR {
        PwmsaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efltie(&self) -> EfltieR {
        EfltieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwmie(&self) -> PwmieR {
        PwmieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adten(&self) -> AdtenR {
        AdtenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn flt0s(&self) -> Flt0sR {
        Flt0sR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn flt1s(&self) -> Flt1sR {
        Flt1sR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn flt2s(&self) -> Flt2sR {
        Flt2sR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn flt0en(&self) -> Flt0enR {
        Flt0enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flt1en(&self) -> Flt1enR {
        Flt1enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn flt2en(&self) -> Flt2enR {
        Flt2enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flt0ie(&self) -> Flt0ieR {
        Flt0ieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn flt1ie(&self) -> Flt1ieR {
        Flt1ieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn flt2ie(&self) -> Flt2ieR {
        Flt2ieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn foutb(&self) -> FoutbR {
        FoutbR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn fouta(&self) -> FoutaR {
        FoutaR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn fltdeb(&self) -> FltdebR {
        FltdebR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pwmen(&mut self) -> PwmenW<'_, CrSpec> {
        PwmenW::new(self, 0)
    }
    #[doc = "Bits 1:3"]
    #[inline(always)]
    pub fn tck(&mut self) -> TckW<'_, CrSpec> {
        TckW::new(self, 1)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn fltc(&mut self) -> FltcW<'_, CrSpec> {
        FltcW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn eflt(&mut self) -> EfltW<'_, CrSpec> {
        EfltW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwmsb(&mut self) -> PwmsbW<'_, CrSpec> {
        PwmsbW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwmsa(&mut self) -> PwmsaW<'_, CrSpec> {
        PwmsaW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efltie(&mut self) -> EfltieW<'_, CrSpec> {
        EfltieW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn pwmie(&mut self) -> PwmieW<'_, CrSpec> {
        PwmieW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn adten(&mut self) -> AdtenW<'_, CrSpec> {
        AdtenW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, CrSpec> {
        Rev3W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn flt0s(&mut self) -> Flt0sW<'_, CrSpec> {
        Flt0sW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn flt1s(&mut self) -> Flt1sW<'_, CrSpec> {
        Flt1sW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn flt2s(&mut self) -> Flt2sW<'_, CrSpec> {
        Flt2sW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CrSpec> {
        Rev2W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn flt0en(&mut self) -> Flt0enW<'_, CrSpec> {
        Flt0enW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> Flt1enW<'_, CrSpec> {
        Flt1enW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> Flt2enW<'_, CrSpec> {
        Flt2enW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CrSpec> {
        Rev1W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flt0ie(&mut self) -> Flt0ieW<'_, CrSpec> {
        Flt0ieW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn flt1ie(&mut self) -> Flt1ieW<'_, CrSpec> {
        Flt1ieW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn flt2ie(&mut self) -> Flt2ieW<'_, CrSpec> {
        Flt2ieW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CrSpec> {
        Rev0W::new(self, 23)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn foutb(&mut self) -> FoutbW<'_, CrSpec> {
        FoutbW::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn fouta(&mut self) -> FoutaW<'_, CrSpec> {
        FoutaW::new(self, 26)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn fltdeb(&mut self) -> FltdebW<'_, CrSpec> {
        FltdebW::new(self, 28)
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
