#[doc = "Register `FLTWEN` reader"]
pub type R = crate::R<FltwenSpec>;
#[doc = "Register `FLTWEN` writer"]
pub type W = crate::W<FltwenSpec>;
#[doc = "Field `FLTWEN` reader - "]
pub type FltwenR = crate::FieldReader<u16>;
#[doc = "Field `FLTWEN` writer - "]
pub type FltwenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn fltwen(&self) -> FltwenR {
        FltwenR::new((self.bits & 0xffff) as u16)
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
    pub fn fltwen(&mut self) -> FltwenW<'_, FltwenSpec> {
        FltwenW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, FltwenSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "FLTWEN\n\nYou can [`read`](crate::Reg::read) this register and get [`fltwen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltwen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltwenSpec;
impl crate::RegisterSpec for FltwenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltwen::R`](R) reader structure"]
impl crate::Readable for FltwenSpec {}
#[doc = "`write(|w| ..)` method takes [`fltwen::W`](W) writer structure"]
impl crate::Writable for FltwenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTWEN to value 0"]
impl crate::Resettable for FltwenSpec {}
