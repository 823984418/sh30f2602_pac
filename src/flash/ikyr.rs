#[doc = "Register `IKYR` writer"]
pub type W = crate::W<IkyrSpec>;
impl core::fmt::Debug for crate::generic::Reg<IkyrSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {}
#[doc = "IKYR\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ikyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IkyrSpec;
impl crate::RegisterSpec for IkyrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ikyr::W`](W) writer structure"]
impl crate::Writable for IkyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IKYR to value 0"]
impl crate::Resettable for IkyrSpec {}
