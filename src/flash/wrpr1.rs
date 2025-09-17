#[doc = "Register `WRPR1` reader"]
pub type R = crate::R<Wrpr1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "WRPR1\n\nYou can [`read`](crate::Reg::read) this register and get [`wrpr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Wrpr1Spec;
impl crate::RegisterSpec for Wrpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrpr1::R`](R) reader structure"]
impl crate::Readable for Wrpr1Spec {}
#[doc = "`reset()` method sets WRPR1 to value 0"]
impl crate::Resettable for Wrpr1Spec {}
