#[doc = "Register `WRPR` reader"]
pub type R = crate::R<WrprSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "WRPR\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrprSpec;
impl crate::RegisterSpec for WrprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr::R`](R) reader structure"]
impl crate::Readable for WrprSpec {}
#[doc = "`reset()` method sets WRPR to value 0"]
impl crate::Resettable for WrprSpec {}
