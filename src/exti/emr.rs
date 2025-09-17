#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Field `EMR` reader - "]
pub type EmrR = crate::FieldReader;
#[doc = "Field `EMR` writer - "]
pub type EmrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn emr(&self) -> EmrR {
        EmrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn emr(&mut self) -> EmrW<'_, EmrSpec> {
        EmrW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, EmrSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "EMR\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {}
