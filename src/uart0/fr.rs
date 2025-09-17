#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FrSpec>;
#[doc = "Field `RI` reader - "]
pub type RiR = crate::BitReader;
#[doc = "Field `TI` reader - "]
pub type TiR = crate::BitReader;
#[doc = "Field `TC` reader - "]
pub type TcR = crate::BitReader;
#[doc = "Field `TXCOL` reader - "]
pub type TxcolR = crate::BitReader;
#[doc = "Field `RXOV` reader - "]
pub type RxovR = crate::BitReader;
#[doc = "Field `FE` reader - "]
pub type FeR = crate::BitReader;
#[doc = "Field `PE` reader - "]
pub type PeR = crate::BitReader;
#[doc = "Field `LBD` reader - "]
pub type LbdR = crate::BitReader;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader<u16>;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TCC` reader - "]
pub type TccR = crate::BitReader;
#[doc = "Field `TCC` writer - "]
pub type TccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXCOLC` reader - "]
pub type TxcolcR = crate::BitReader;
#[doc = "Field `TXCOLC` writer - "]
pub type TxcolcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVC` reader - "]
pub type RxovcR = crate::BitReader;
#[doc = "Field `RXOVC` writer - "]
pub type RxovcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEC` reader - "]
pub type FecR = crate::BitReader;
#[doc = "Field `FEC` writer - "]
pub type FecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEC` reader - "]
pub type PecR = crate::BitReader;
#[doc = "Field `PEC` writer - "]
pub type PecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBDC` reader - "]
pub type LbdcR = crate::BitReader;
#[doc = "Field `LBDC` writer - "]
pub type LbdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn txcol(&self) -> TxcolR {
        TxcolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxov(&self) -> RxovR {
        RxovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn lbd(&self) -> LbdR {
        LbdR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:17"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 8) & 0x03ff) as u16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn txcolc(&self) -> TxcolcR {
        TxcolcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rxovc(&self) -> RxovcR {
        RxovcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fec(&self) -> FecR {
        FecR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pec(&self) -> PecR {
        PecR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn lbdc(&self) -> LbdcR {
        LbdcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:17"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, FrSpec> {
        Rev1W::new(self, 8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn tcc(&mut self) -> TccW<'_, FrSpec> {
        TccW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn txcolc(&mut self) -> TxcolcW<'_, FrSpec> {
        TxcolcW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rxovc(&mut self) -> RxovcW<'_, FrSpec> {
        RxovcW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn fec(&mut self) -> FecW<'_, FrSpec> {
        FecW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pec(&mut self) -> PecW<'_, FrSpec> {
        PecW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn lbdc(&mut self) -> LbdcW<'_, FrSpec> {
        LbdcW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, FrSpec> {
        Rev0W::new(self, 24)
    }
}
#[doc = "FR\n\nYou can [`read`](crate::Reg::read) this register and get [`fr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrSpec;
impl crate::RegisterSpec for FrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fr::R`](R) reader structure"]
impl crate::Readable for FrSpec {}
#[doc = "`write(|w| ..)` method takes [`fr::W`](W) writer structure"]
impl crate::Writable for FrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FR to value 0x06"]
impl crate::Resettable for FrSpec {
    const RESET_VALUE: u32 = 0x06;
}
