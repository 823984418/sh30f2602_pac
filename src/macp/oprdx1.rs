#[doc = "Register `OPRDX1` reader"]
pub type R = crate::R<Oprdx1Spec>;
#[doc = "Register `OPRDX1` writer"]
pub type W = crate::W<Oprdx1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "OPRDX1\n\nYou can [`read`](crate::Reg::read) this register and get [`oprdx1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oprdx1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Oprdx1Spec;
impl crate::RegisterSpec for Oprdx1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oprdx1::R`](R) reader structure"]
impl crate::Readable for Oprdx1Spec {}
#[doc = "`write(|w| ..)` method takes [`oprdx1::W`](W) writer structure"]
impl crate::Writable for Oprdx1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPRDX1 to value 0"]
impl crate::Resettable for Oprdx1Spec {}
