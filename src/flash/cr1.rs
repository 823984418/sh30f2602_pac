#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `SNB` reader - "]
pub type SnbR = crate::FieldReader<u32>;
#[doc = "Field `SNB` writer - "]
pub type SnbW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn snb(&self) -> SnbR {
        SnbR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("rev0", &self.rev0())
            .field("snb", &self.snb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:17"]
    #[inline(always)]
    pub fn snb(&mut self) -> SnbW<'_, Cr1Spec> {
        SnbW::new(self, 0)
    }
    #[doc = "Bits 18:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Cr1Spec> {
        Rev0W::new(self, 18)
    }
}
#[doc = "CR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}
