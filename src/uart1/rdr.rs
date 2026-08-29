#[doc = "Register `RDR` reader"]
pub type R = crate::R<RdrSpec>;
#[doc = "Field `RDR` reader - "]
pub type RdrR = crate::FieldReader;
#[doc = "Field `RB8` reader - "]
pub type Rb8R = crate::BitReader;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rb8(&self) -> Rb8R {
        Rb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RDR")
            .field("rev0", &self.rev0())
            .field("rb8", &self.rb8())
            .field("rdr", &self.rdr())
            .finish()
    }
}
#[doc = "RDR\n\nYou can [`read`](crate::Reg::read) this register and get [`rdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdrSpec;
impl crate::RegisterSpec for RdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdr::R`](R) reader structure"]
impl crate::Readable for RdrSpec {}
#[doc = "`reset()` method sets RDR to value 0"]
impl crate::Resettable for RdrSpec {}
