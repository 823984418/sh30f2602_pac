#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PrSpec>;
#[doc = "Field `PR` reader - "]
pub type PrR = crate::FieldReader;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRC` reader - "]
pub type PrcR = crate::FieldReader;
#[doc = "Field `PRC` writer - "]
pub type PrcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn prc(&self) -> PrcR {
        PrcR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, PrSpec> {
        Rev1W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn prc(&mut self) -> PrcW<'_, PrSpec> {
        PrcW::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PrSpec> {
        Rev0W::new(self, 24)
    }
}
#[doc = "PR\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PrSpec {}
