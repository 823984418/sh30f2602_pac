#[doc = "Register `RSTSTR` reader"]
pub type R = crate::R<RststrSpec>;
#[doc = "Field `PINRSTF` reader - "]
pub type PinrstfR = crate::BitReader;
#[doc = "Field `LVRSTF` reader - "]
pub type LvrstfR = crate::BitReader;
#[doc = "Field `PORSTF` reader - "]
pub type PorstfR = crate::BitReader;
#[doc = "Field `SWRSTF` reader - "]
pub type SwrstfR = crate::BitReader;
#[doc = "Field `IWDTRSTF` reader - "]
pub type IwdtrstfR = crate::BitReader;
#[doc = "Field `WWDTRSTF` reader - "]
pub type WwdtrstfR = crate::BitReader;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn lvrstf(&self) -> LvrstfR {
        LvrstfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn porstf(&self) -> PorstfR {
        PorstfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn swrstf(&self) -> SwrstfR {
        SwrstfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn iwdtrstf(&self) -> IwdtrstfR {
        IwdtrstfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wwdtrstf(&self) -> WwdtrstfR {
        WwdtrstfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSTSTR")
            .field("rev0", &self.rev0())
            .field("wwdtrstf", &self.wwdtrstf())
            .field("iwdtrstf", &self.iwdtrstf())
            .field("swrstf", &self.swrstf())
            .field("porstf", &self.porstf())
            .field("lvrstf", &self.lvrstf())
            .field("pinrstf", &self.pinrstf())
            .finish()
    }
}
#[doc = "RSTSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`rststr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RststrSpec;
impl crate::RegisterSpec for RststrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rststr::R`](R) reader structure"]
impl crate::Readable for RststrSpec {}
#[doc = "`reset()` method sets RSTSTR to value 0x04"]
impl crate::Resettable for RststrSpec {
    const RESET_VALUE: u32 = 0x04;
}
