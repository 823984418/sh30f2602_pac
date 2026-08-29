#[doc = "Register `ADPCH` reader"]
pub type R = crate::R<AdpchSpec>;
#[doc = "Field `ADPCH` reader - "]
pub type AdpchR = crate::FieldReader;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn adpch(&self) -> AdpchR {
        AdpchR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADPCH")
            .field("rev0", &self.rev0())
            .field("adpch", &self.adpch())
            .finish()
    }
}
#[doc = "ADPCH\n\nYou can [`read`](crate::Reg::read) this register and get [`adpch::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdpchSpec;
impl crate::RegisterSpec for AdpchSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adpch::R`](R) reader structure"]
impl crate::Readable for AdpchSpec {}
#[doc = "`reset()` method sets ADPCH to value 0"]
impl crate::Resettable for AdpchSpec {}
