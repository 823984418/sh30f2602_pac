#[doc = "Register `WTR` reader"]
pub type R = crate::R<WtrSpec>;
#[doc = "Register `WTR` writer"]
pub type W = crate::W<WtrSpec>;
#[doc = "Field `WWDTWTR` reader - "]
pub type WwdtwtrR = crate::FieldReader;
#[doc = "Field `WWDTWTR` writer - "]
pub type WwdtwtrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wwdtwtr(&self) -> WwdtwtrR {
        WwdtwtrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WTR")
            .field("lock", &self.lock())
            .field("rev0", &self.rev0())
            .field("wwdtwtr", &self.wwdtwtr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wwdtwtr(&mut self) -> WwdtwtrW<'_, WtrSpec> {
        WwdtwtrW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, WtrSpec> {
        Rev0W::new(self, 8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, WtrSpec> {
        LockW::new(self, 16)
    }
}
#[doc = "WTR\n\nYou can [`read`](crate::Reg::read) this register and get [`wtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WtrSpec;
impl crate::RegisterSpec for WtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtr::R`](R) reader structure"]
impl crate::Readable for WtrSpec {}
#[doc = "`write(|w| ..)` method takes [`wtr::W`](W) writer structure"]
impl crate::Writable for WtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WTR to value 0"]
impl crate::Resettable for WtrSpec {}
