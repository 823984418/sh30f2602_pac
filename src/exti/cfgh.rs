#[doc = "Register `CFGH` reader"]
pub type R = crate::R<CfghSpec>;
#[doc = "Register `CFGH` writer"]
pub type W = crate::W<CfghSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "CFGH\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfghSpec;
impl crate::RegisterSpec for CfghSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgh::R`](R) reader structure"]
impl crate::Readable for CfghSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgh::W`](W) writer structure"]
impl crate::Writable for CfghSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGH to value 0"]
impl crate::Resettable for CfghSpec {}
