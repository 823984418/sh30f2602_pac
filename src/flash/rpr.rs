#[doc = "Register `RPR` reader"]
pub type R = crate::R<RprSpec>;
#[doc = "Field `RDP` reader - "]
pub type RdpR = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RPR")
            .field("rev0", &self.rev0())
            .field("rdp", &self.rdp())
            .finish()
    }
}
#[doc = "RPR\n\nYou can [`read`](crate::Reg::read) this register and get [`rpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RprSpec;
impl crate::RegisterSpec for RprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rpr::R`](R) reader structure"]
impl crate::Readable for RprSpec {}
#[doc = "`reset()` method sets RPR to value 0"]
impl crate::Resettable for RprSpec {}
