#[doc = "Register `CCR0` reader"]
pub type R = crate::R<Ccr0Spec>;
#[doc = "Register `CCR0` writer"]
pub type W = crate::W<Ccr0Spec>;
#[doc = "Field `CCR0L` reader - "]
pub type Ccr0lR = crate::FieldReader<u16>;
#[doc = "Field `CCR0L` writer - "]
pub type Ccr0lW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CCR0H` reader - "]
pub type Ccr0hR = crate::FieldReader<u16>;
#[doc = "Field `CCR0H` writer - "]
pub type Ccr0hW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ccr0l(&self) -> Ccr0lR {
        Ccr0lR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ccr0h(&self) -> Ccr0hR {
        Ccr0hR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR0")
            .field("ccr0h", &self.ccr0h())
            .field("ccr0l", &self.ccr0l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn ccr0l(&mut self) -> Ccr0lW<'_, Ccr0Spec> {
        Ccr0lW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn ccr0h(&mut self) -> Ccr0hW<'_, Ccr0Spec> {
        Ccr0hW::new(self, 16)
    }
}
#[doc = "CCR0\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr0Spec;
impl crate::RegisterSpec for Ccr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr0::R`](R) reader structure"]
impl crate::Readable for Ccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr0::W`](W) writer structure"]
impl crate::Writable for Ccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR0 to value 0"]
impl crate::Resettable for Ccr0Spec {}
