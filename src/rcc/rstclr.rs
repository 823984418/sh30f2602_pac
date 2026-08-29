#[doc = "Register `RSTCLR` reader"]
pub type R = crate::R<RstclrSpec>;
#[doc = "Register `RSTCLR` writer"]
pub type W = crate::W<RstclrSpec>;
#[doc = "Field `PINRSTFC` reader - "]
pub type PinrstfcR = crate::BitReader;
#[doc = "Field `PINRSTFC` writer - "]
pub type PinrstfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVRSTFC` reader - "]
pub type LvrstfcR = crate::BitReader;
#[doc = "Field `LVRSTFC` writer - "]
pub type LvrstfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORSTFC` reader - "]
pub type PorstfcR = crate::BitReader;
#[doc = "Field `PORSTFC` writer - "]
pub type PorstfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRSTFC` reader - "]
pub type SwrstfcR = crate::BitReader;
#[doc = "Field `SWRSTFC` writer - "]
pub type SwrstfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDTRSTFC` reader - "]
pub type IwdtrstfcR = crate::BitReader;
#[doc = "Field `IWDTRSTFC` writer - "]
pub type IwdtrstfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTRSTFC` reader - "]
pub type WwdtrstfcR = crate::BitReader;
#[doc = "Field `WWDTRSTFC` writer - "]
pub type WwdtrstfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pinrstfc(&self) -> PinrstfcR {
        PinrstfcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lvrstfc(&self) -> LvrstfcR {
        LvrstfcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn porstfc(&self) -> PorstfcR {
        PorstfcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrstfc(&self) -> SwrstfcR {
        SwrstfcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iwdtrstfc(&self) -> IwdtrstfcR {
        IwdtrstfcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wwdtrstfc(&self) -> WwdtrstfcR {
        WwdtrstfcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTCLR")
            .field("rev0", &self.rev0())
            .field("wwdtrstfc", &self.wwdtrstfc())
            .field("iwdtrstfc", &self.iwdtrstfc())
            .field("swrstfc", &self.swrstfc())
            .field("porstfc", &self.porstfc())
            .field("lvrstfc", &self.lvrstfc())
            .field("pinrstfc", &self.pinrstfc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pinrstfc(&mut self) -> PinrstfcW<'_, RstclrSpec> {
        PinrstfcW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lvrstfc(&mut self) -> LvrstfcW<'_, RstclrSpec> {
        LvrstfcW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn porstfc(&mut self) -> PorstfcW<'_, RstclrSpec> {
        PorstfcW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrstfc(&mut self) -> SwrstfcW<'_, RstclrSpec> {
        SwrstfcW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iwdtrstfc(&mut self) -> IwdtrstfcW<'_, RstclrSpec> {
        IwdtrstfcW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wwdtrstfc(&mut self) -> WwdtrstfcW<'_, RstclrSpec> {
        WwdtrstfcW::new(self, 5)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, RstclrSpec> {
        Rev0W::new(self, 6)
    }
}
#[doc = "RSTCLR\n\nYou can [`read`](crate::Reg::read) this register and get [`rstclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstclrSpec;
impl crate::RegisterSpec for RstclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstclr::R`](R) reader structure"]
impl crate::Readable for RstclrSpec {}
#[doc = "`write(|w| ..)` method takes [`rstclr::W`](W) writer structure"]
impl crate::Writable for RstclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RSTCLR to value 0"]
impl crate::Resettable for RstclrSpec {}
