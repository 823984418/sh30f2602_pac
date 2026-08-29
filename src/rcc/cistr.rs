#[doc = "Register `CISTR` reader"]
pub type R = crate::R<CistrSpec>;
#[doc = "Field `rev4` reader - "]
pub type Rev4R = crate::FieldReader;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `PLLRDYIF` reader - "]
pub type PllrdyifR = crate::BitReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `CSMPLLF` reader - "]
pub type CsmpllfR = crate::BitReader;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pllrdyif(&self) -> PllrdyifR {
        PllrdyifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn csmpllf(&self) -> CsmpllfR {
        CsmpllfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CISTR")
            .field("rev0", &self.rev0())
            .field("csmpllf", &self.csmpllf())
            .field("rev1", &self.rev1())
            .field("rev2", &self.rev2())
            .field("pllrdyif", &self.pllrdyif())
            .field("rev3", &self.rev3())
            .field("rev4", &self.rev4())
            .finish()
    }
}
#[doc = "CISTR\n\nYou can [`read`](crate::Reg::read) this register and get [`cistr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CistrSpec;
impl crate::RegisterSpec for CistrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cistr::R`](R) reader structure"]
impl crate::Readable for CistrSpec {}
#[doc = "`reset()` method sets CISTR to value 0"]
impl crate::Resettable for CistrSpec {}
