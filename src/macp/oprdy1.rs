#[doc = "Register `OPRDY1` reader"]
pub type R = crate::R<Oprdy1Spec>;
#[doc = "Register `OPRDY1` writer"]
pub type W = crate::W<Oprdy1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OPRDY1\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdy1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdy1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oprdy1Spec;
impl crate::RegisterSpec for Oprdy1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oprdy1::R`](R) reader structure"]
impl crate::Readable for Oprdy1Spec {}
#[doc = "`write(|w| ..)` method takes [`oprdy1::W`](W) writer structure"]
impl crate::Writable for Oprdy1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPRDY1 to value 0"]
impl crate::Resettable for Oprdy1Spec {}
