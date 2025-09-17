#[doc = "Register `PWMDTR` reader"]
pub type R = crate::R<PwmdtrSpec>;
#[doc = "Register `PWMDTR` writer"]
pub type W = crate::W<PwmdtrSpec>;
#[doc = "Field `DT` reader - "]
pub type DtR = crate::FieldReader<u16>;
#[doc = "Field `DT` writer - "]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new((self.bits & 0xffff) as u16)
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
    pub fn dt(&mut self) -> DtW<'_, PwmdtrSpec> {
        DtW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmdtrSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMDTR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmdtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmdtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmdtrSpec;
impl crate::RegisterSpec for PwmdtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmdtr::R`](R) reader structure"]
impl crate::Readable for PwmdtrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmdtr::W`](W) writer structure"]
impl crate::Writable for PwmdtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMDTR to value 0"]
impl crate::Resettable for PwmdtrSpec {}
