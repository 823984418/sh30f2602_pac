#[doc = "Register `DIVRLT0` reader"]
pub type R = crate::R<Divrlt0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DIVRLT0\n\nYou can [`read`](crate::Reg::read) this register and get [`divrlt0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Divrlt0Spec;
impl crate::RegisterSpec for Divrlt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divrlt0::R`](R) reader structure"]
impl crate::Readable for Divrlt0Spec {}
#[doc = "`reset()` method sets DIVRLT0 to value 0"]
impl crate::Resettable for Divrlt0Spec {}
