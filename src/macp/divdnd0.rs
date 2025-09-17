#[doc = "Register `DIVDND0` reader"]
pub type R = crate::R<Divdnd0Spec>;
#[doc = "Register `DIVDND0` writer"]
pub type W = crate::W<Divdnd0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "DIVDND0\n\nYou can [`read`](crate::Reg::read) this register and get [`divdnd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divdnd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Divdnd0Spec;
impl crate::RegisterSpec for Divdnd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divdnd0::R`](R) reader structure"]
impl crate::Readable for Divdnd0Spec {}
#[doc = "`write(|w| ..)` method takes [`divdnd0::W`](W) writer structure"]
impl crate::Writable for Divdnd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVDND0 to value 0"]
impl crate::Resettable for Divdnd0Spec {}
