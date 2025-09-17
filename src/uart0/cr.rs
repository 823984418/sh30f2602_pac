#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `STOP` reader - "]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - "]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBRTEN` reader - "]
pub type SbrtenR = crate::BitReader;
#[doc = "Field `SBRTEN` writer - "]
pub type SbrtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMOD2` reader - "]
pub type Smod2R = crate::BitReader;
#[doc = "Field `SMOD2` writer - "]
pub type Smod2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - "]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - "]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - "]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - "]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - "]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - "]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDIE` reader - "]
pub type LbdieR = crate::BitReader;
#[doc = "Field `LBDIE` writer - "]
pub type LbdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDL` reader - "]
pub type LbdlR = crate::BitReader;
#[doc = "Field `LBDL` writer - "]
pub type LbdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMOD0` reader - "]
pub type Smod0R = crate::BitReader;
#[doc = "Field `SMOD0` writer - "]
pub type Smod0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS` reader - "]
pub type PsR = crate::BitReader;
#[doc = "Field `PS` writer - "]
pub type PsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MULTIE` reader - "]
pub type MultieR = crate::FieldReader;
#[doc = "Field `MULTIE` writer - "]
pub type MultieW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SM` reader - "]
pub type SmR = crate::FieldReader;
#[doc = "Field `SM` writer - "]
pub type SmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SBK` reader - "]
pub type SbkR = crate::BitReader;
#[doc = "Field `SBK` writer - "]
pub type SbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINEN` reader - "]
pub type LinenR = crate::BitReader;
#[doc = "Field `LINEN` writer - "]
pub type LinenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REN` reader - "]
pub type RenR = crate::BitReader;
#[doc = "Field `REN` writer - "]
pub type RenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEN` reader - "]
pub type TenR = crate::BitReader;
#[doc = "Field `TEN` writer - "]
pub type TenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RNEG` reader - "]
pub type RnegR = crate::BitReader;
#[doc = "Field `RNEG` writer - "]
pub type RnegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TNEG` reader - "]
pub type TnegR = crate::BitReader;
#[doc = "Field `TNEG` writer - "]
pub type TnegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FER` reader - "]
pub type FerR = crate::BitReader;
#[doc = "Field `FER` writer - "]
pub type FerW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sbrten(&self) -> SbrtenR {
        SbrtenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn smod2(&self) -> Smod2R {
        Smod2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lbdie(&self) -> LbdieR {
        LbdieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lbdl(&self) -> LbdlR {
        LbdlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn smod0(&self) -> Smod0R {
        Smod0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn multie(&self) -> MultieR {
        MultieR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sbk(&self) -> SbkR {
        SbkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn linen(&self) -> LinenR {
        LinenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ren(&self) -> RenR {
        RenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten(&self) -> TenR {
        TenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 19) & 3) as u8)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rneg(&self) -> RnegR {
        RnegR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tneg(&self) -> TnegR {
        TnegR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fer(&self) -> FerR {
        FerR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, CrSpec> {
        StopW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sbrten(&mut self) -> SbrtenW<'_, CrSpec> {
        SbrtenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn smod2(&mut self) -> Smod2W<'_, CrSpec> {
        Smod2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<'_, CrSpec> {
        RieW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<'_, CrSpec> {
        TieW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<'_, CrSpec> {
        TcieW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LbdieW<'_, CrSpec> {
        LbdieW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LbdlW<'_, CrSpec> {
        LbdlW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn smod0(&mut self) -> Smod0W<'_, CrSpec> {
        Smod0W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CrSpec> {
        Rev2W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, CrSpec> {
        PsW::new(self, 10)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn multie(&mut self) -> MultieW<'_, CrSpec> {
        MultieW::new(self, 11)
    }
    #[doc = "Bits 13:14"]
    #[inline(always)]
    pub fn sm(&mut self) -> SmW<'_, CrSpec> {
        SmW::new(self, 13)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn sbk(&mut self) -> SbkW<'_, CrSpec> {
        SbkW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn linen(&mut self) -> LinenW<'_, CrSpec> {
        LinenW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ren(&mut self) -> RenW<'_, CrSpec> {
        RenW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ten(&mut self) -> TenW<'_, CrSpec> {
        TenW::new(self, 18)
    }
    #[doc = "Bits 19:20"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CrSpec> {
        Rev1W::new(self, 19)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn rneg(&mut self) -> RnegW<'_, CrSpec> {
        RnegW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn tneg(&mut self) -> TnegW<'_, CrSpec> {
        TnegW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn fer(&mut self) -> FerW<'_, CrSpec> {
        FerW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CrSpec> {
        Rev0W::new(self, 24)
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
