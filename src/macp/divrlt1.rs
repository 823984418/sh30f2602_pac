#[doc = "Register `DIVRLT1` reader"]
pub type R = crate::R<Divrlt1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "DIVRLT1\n\nYou can [`read`](crate::Reg::read) this register and get [`divrlt1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Divrlt1Spec;
impl crate::RegisterSpec for Divrlt1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divrlt1::R`](R) reader structure"]
impl crate::Readable for Divrlt1Spec {}
#[doc = "`reset()` method sets DIVRLT1 to value 0"]
impl crate::Resettable for Divrlt1Spec {}
