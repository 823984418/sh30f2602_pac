#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `EOP` reader - "]
pub type EopR = crate::BitReader;
#[doc = "Field `OPERR` reader - "]
pub type OperrR = crate::BitReader;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLSERR` reader - "]
pub type FlserrR = crate::BitReader;
#[doc = "Field `WRPRTERR` reader - "]
pub type WrprterrR = crate::BitReader;
#[doc = "Field `PGPERR` reader - "]
pub type PgperrR = crate::BitReader;
#[doc = "Field `PGWERR` reader - "]
pub type PgwerrR = crate::BitReader;
#[doc = "Field `STAERR` reader - "]
pub type StaerrR = crate::BitReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BSY` reader - "]
pub type BsyR = crate::BitReader;
#[doc = "Field `EOPC` reader - "]
pub type EopcR = crate::BitReader;
#[doc = "Field `EOPC` writer - "]
pub type EopcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERRC` reader - "]
pub type OperrcR = crate::BitReader;
#[doc = "Field `OPERRC` writer - "]
pub type OperrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLSERRC` reader - "]
pub type FlserrcR = crate::BitReader;
#[doc = "Field `FLSERRC` writer - "]
pub type FlserrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPRTERRC` reader - "]
pub type WrprterrcR = crate::BitReader;
#[doc = "Field `WRPRTERRC` writer - "]
pub type WrprterrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGPERRC` reader - "]
pub type PgperrcR = crate::BitReader;
#[doc = "Field `PGPERRC` writer - "]
pub type PgperrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGWERRC` reader - "]
pub type PgwerrcR = crate::BitReader;
#[doc = "Field `PGWERRC` writer - "]
pub type PgwerrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAERRC` reader - "]
pub type StaerrcR = crate::BitReader;
#[doc = "Field `STAERRC` writer - "]
pub type StaerrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn eop(&self) -> EopR {
        EopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn operr(&self) -> OperrR {
        OperrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flserr(&self) -> FlserrR {
        FlserrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wrprterr(&self) -> WrprterrR {
        WrprterrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pgperr(&self) -> PgperrR {
        PgperrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pgwerr(&self) -> PgwerrR {
        PgwerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn staerr(&self) -> StaerrR {
        StaerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn eopc(&self) -> EopcR {
        EopcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn operrc(&self) -> OperrcR {
        OperrcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flserrc(&self) -> FlserrcR {
        FlserrcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wrprterrc(&self) -> WrprterrcR {
        WrprterrcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pgperrc(&self) -> PgperrcR {
        PgperrcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pgwerrc(&self) -> PgwerrcR {
        PgwerrcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn staerrc(&self) -> StaerrcR {
        StaerrcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, SrSpec> {
        Rev3W::new(self, 2)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, SrSpec> {
        Rev2W::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn eopc(&mut self) -> EopcW<'_, SrSpec> {
        EopcW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn operrc(&mut self) -> OperrcW<'_, SrSpec> {
        OperrcW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, SrSpec> {
        Rev1W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flserrc(&mut self) -> FlserrcW<'_, SrSpec> {
        FlserrcW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn wrprterrc(&mut self) -> WrprterrcW<'_, SrSpec> {
        WrprterrcW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pgperrc(&mut self) -> PgperrcW<'_, SrSpec> {
        PgperrcW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pgwerrc(&mut self) -> PgwerrcW<'_, SrSpec> {
        PgwerrcW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn staerrc(&mut self) -> StaerrcW<'_, SrSpec> {
        StaerrcW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, SrSpec> {
        Rev0W::new(self, 24)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
