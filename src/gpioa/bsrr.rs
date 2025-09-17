#[doc = "Register `BSRR` reader"]
pub type R = crate::R<BsrrSpec>;
#[doc = "Register `BSRR` writer"]
pub type W = crate::W<BsrrSpec>;
#[doc = "Field `BS` reader - "]
pub type BsR = crate::FieldReader<u16>;
#[doc = "Field `BS` writer - "]
pub type BsW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BR` reader - "]
pub type BrR = crate::FieldReader<u16>;
#[doc = "Field `BR` writer - "]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bs(&self) -> BsR {
        BsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bs(&mut self) -> BsW<'_, BsrrSpec> {
        BsW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<'_, BsrrSpec> {
        BrW::new(self, 16)
    }
}
#[doc = "BSRR\n\nYou can [`read`](crate::Reg::read) this register and get [`bsrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BsrrSpec;
impl crate::RegisterSpec for BsrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bsrr::R`](R) reader structure"]
impl crate::Readable for BsrrSpec {}
#[doc = "`write(|w| ..)` method takes [`bsrr::W`](W) writer structure"]
impl crate::Writable for BsrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BSRR to value 0"]
impl crate::Resettable for BsrrSpec {}
