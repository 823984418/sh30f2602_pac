#[doc = "Register `OPRDY0` reader"]
pub type R = crate::R<Oprdy0Spec>;
#[doc = "Register `OPRDY0` writer"]
pub type W = crate::W<Oprdy0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OPRDY0\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdy0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdy0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oprdy0Spec;
impl crate::RegisterSpec for Oprdy0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oprdy0::R`](R) reader structure"]
impl crate::Readable for Oprdy0Spec {}
#[doc = "`write(|w| ..)` method takes [`oprdy0::W`](W) writer structure"]
impl crate::Writable for Oprdy0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPRDY0 to value 0"]
impl crate::Resettable for Oprdy0Spec {}
