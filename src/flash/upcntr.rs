#[doc = "Register `UPCNTR` reader"]
pub type R = crate::R<UpcntrSpec>;
#[doc = "Register `UPCNTR` writer"]
pub type W = crate::W<UpcntrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "UPCNTR\n\nYou can [`read`](crate::Reg::read) this register and get [`upcntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upcntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpcntrSpec;
impl crate::RegisterSpec for UpcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`upcntr::R`](R) reader structure"]
impl crate::Readable for UpcntrSpec {}
#[doc = "`write(|w| ..)` method takes [`upcntr::W`](W) writer structure"]
impl crate::Writable for UpcntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPCNTR to value 0"]
impl crate::Resettable for UpcntrSpec {}
