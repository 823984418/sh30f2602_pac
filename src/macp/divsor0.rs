#[doc = "Register `DIVSOR0` reader"]
pub type R = crate::R<Divsor0Spec>;
#[doc = "Register `DIVSOR0` writer"]
pub type W = crate::W<Divsor0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DIVSOR0\n\nYou can [`read`](crate::Reg::read) this register and get [`divsor0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divsor0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Divsor0Spec;
impl crate::RegisterSpec for Divsor0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divsor0::R`](R) reader structure"]
impl crate::Readable for Divsor0Spec {}
#[doc = "`write(|w| ..)` method takes [`divsor0::W`](W) writer structure"]
impl crate::Writable for Divsor0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVSOR0 to value 0"]
impl crate::Resettable for Divsor0Spec {}
