#[doc = "Register `OPRDZ1` reader"]
pub type R = crate::R<Oprdz1Spec>;
#[doc = "Register `OPRDZ1` writer"]
pub type W = crate::W<Oprdz1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OPRDZ1\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oprdz1Spec;
impl crate::RegisterSpec for Oprdz1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oprdz1::R`](R) reader structure"]
impl crate::Readable for Oprdz1Spec {}
#[doc = "`write(|w| ..)` method takes [`oprdz1::W`](W) writer structure"]
impl crate::Writable for Oprdz1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPRDZ1 to value 0"]
impl crate::Resettable for Oprdz1Spec {}
