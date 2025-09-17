#[doc = "Register `E2KYR` writer"]
pub type W = crate::W<E2kyrSpec>;
impl core::fmt::Debug for crate::generic::Reg<E2kyrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "E2KYR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`e2kyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct E2kyrSpec;
impl crate::RegisterSpec for E2kyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`e2kyr::W`](W) writer structure"]
impl crate::Writable for E2kyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets E2KYR to value 0"]
impl crate::Resettable for E2kyrSpec {}
