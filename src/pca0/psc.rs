#[doc = "Register `PSC` reader"]
pub type R = crate::R<PscSpec>;
#[doc = "Register `PSC` writer"]
pub type W = crate::W<PscSpec>;
#[doc = "Field `PSC` reader - "]
pub type PscR = crate::FieldReader<u16>;
#[doc = "Field `PSC` writer - "]
pub type PscW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn psc(&self) -> PscR {
        PscR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn psc(&mut self) -> PscW<'_, PscSpec> {
        PscW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PscSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PSC\n\nYou can [`read`](crate::Reg::read) this register and get [`psc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PscSpec;
impl crate::RegisterSpec for PscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psc::R`](R) reader structure"]
impl crate::Readable for PscSpec {}
#[doc = "`write(|w| ..)` method takes [`psc::W`](W) writer structure"]
impl crate::Writable for PscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSC to value 0"]
impl crate::Resettable for PscSpec {}
