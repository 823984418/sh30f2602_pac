#[doc = "Register `ODRVR` reader"]
pub type R = crate::R<OdrvrSpec>;
#[doc = "Register `ODRVR` writer"]
pub type W = crate::W<OdrvrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "ODRVR\n\nYou can [`read`](crate::Reg::read) this register and get [`odrvr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`odrvr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OdrvrSpec;
impl crate::RegisterSpec for OdrvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`odrvr::R`](R) reader structure"]
impl crate::Readable for OdrvrSpec {}
#[doc = "`write(|w| ..)` method takes [`odrvr::W`](W) writer structure"]
impl crate::Writable for OdrvrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ODRVR to value 0"]
impl crate::Resettable for OdrvrSpec {}
