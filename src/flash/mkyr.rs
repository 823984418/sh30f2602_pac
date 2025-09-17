#[doc = "Register `MKYR` writer"]
pub type W = crate::W<MkyrSpec>;
impl core::fmt::Debug for crate::generic::Reg<MkyrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "MKYR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mkyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MkyrSpec;
impl crate::RegisterSpec for MkyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mkyr::W`](W) writer structure"]
impl crate::Writable for MkyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MKYR to value 0"]
impl crate::Resettable for MkyrSpec {}
