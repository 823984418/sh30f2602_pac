#[doc = "Register `PWMP` reader"]
pub type R = crate::R<PwmpSpec>;
#[doc = "Register `PWMP` writer"]
pub type W = crate::W<PwmpSpec>;
#[doc = "Field `PWMP` reader - "]
pub type PwmpR = crate::FieldReader<u16>;
#[doc = "Field `PWMP` writer - "]
pub type PwmpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn pwmp(&self) -> PwmpR {
        PwmpR::new((self.bits & 0xffff) as u16)
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
    pub fn pwmp(&mut self) -> PwmpW<'_, PwmpSpec> {
        PwmpW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmpSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "PWMP\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmpSpec;
impl crate::RegisterSpec for PwmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmp::R`](R) reader structure"]
impl crate::Readable for PwmpSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmp::W`](W) writer structure"]
impl crate::Writable for PwmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMP to value 0"]
impl crate::Resettable for PwmpSpec {}
