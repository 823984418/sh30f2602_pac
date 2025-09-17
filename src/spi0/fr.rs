#[doc = "Register `FR` reader"]
pub type R = crate::R<FrSpec>;
#[doc = "Register `FR` writer"]
pub type W = crate::W<FrSpec>;
#[doc = "Field `SPRI` reader - "]
pub type SpriR = crate::BitReader;
#[doc = "Field `SPTI` reader - "]
pub type SptiR = crate::BitReader;
#[doc = "Field `BUSY` reader - "]
pub type BusyR = crate::BitReader;
#[doc = "Field `MODF` reader - "]
pub type ModfR = crate::BitReader;
#[doc = "Field `RXOV` reader - "]
pub type RxovR = crate::BitReader;
#[doc = "Field `WCOL` reader - "]
pub type WcolR = crate::BitReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader<u16>;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SPRIC` reader - "]
pub type SpricR = crate::BitReader;
#[doc = "Field `SPRIC` writer - "]
pub type SpricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPTIC` reader - "]
pub type SpticR = crate::BitReader;
#[doc = "Field `SPTIC` writer - "]
pub type SpticW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFC` reader - "]
pub type ModfcR = crate::BitReader;
#[doc = "Field `MODFC` writer - "]
pub type ModfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVC` reader - "]
pub type RxovcR = crate::BitReader;
#[doc = "Field `RXOVC` writer - "]
pub type RxovcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCOLC` reader - "]
pub type WcolcR = crate::BitReader;
#[doc = "Field `WCOLC` writer - "]
pub type WcolcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn spri(&self) -> SpriR {
        SpriR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn spti(&self) -> SptiR {
        SptiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn modf(&self) -> ModfR {
        ModfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rxov(&self) -> RxovR {
        RxovR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wcol(&self) -> WcolR {
        WcolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spric(&self) -> SpricR {
        SpricR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sptic(&self) -> SpticR {
        SpticR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn modfc(&self) -> ModfcR {
        ModfcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rxovc(&self) -> RxovcR {
        RxovcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wcolc(&self) -> WcolcR {
        WcolcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, FrSpec> {
        Rev2W::new(self, 6)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn spric(&mut self) -> SpricW<'_, FrSpec> {
        SpricW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn sptic(&mut self) -> SpticW<'_, FrSpec> {
        SpticW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, FrSpec> {
        Rev1W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn modfc(&mut self) -> ModfcW<'_, FrSpec> {
        ModfcW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rxovc(&mut self) -> RxovcW<'_, FrSpec> {
        RxovcW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn wcolc(&mut self) -> WcolcW<'_, FrSpec> {
        WcolcW::new(self, 21)
    }
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, FrSpec> {
        Rev0W::new(self, 22)
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
#[doc = "`reset()` method sets FR to value 0x02"]
impl crate::Resettable for FrSpec {
    const RESET_VALUE: u32 = 0x02;
}
