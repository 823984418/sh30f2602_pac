#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `WWDTRLR` reader - "]
pub type WwdtrlrR = crate::FieldReader;
#[doc = "Field `WWDTRLR` writer - "]
pub type WwdtrlrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WWDTPR` reader - "]
pub type WwdtprR = crate::FieldReader;
#[doc = "Field `WWDTPR` writer - "]
pub type WwdtprW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WWDTIE` reader - "]
pub type WwdtieR = crate::BitReader;
#[doc = "Field `WWDTIE` writer - "]
pub type WwdtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDTON` reader - "]
pub type WwdtonR = crate::BitReader;
#[doc = "Field `WWDTON` writer - "]
pub type WwdtonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wwdtrlr(&self) -> WwdtrlrR {
        WwdtrlrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn wwdtpr(&self) -> WwdtprR {
        WwdtprR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wwdtie(&self) -> WwdtieR {
        WwdtieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wwdton(&self) -> WwdtonR {
        WwdtonR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lock", &self.lock())
            .field("wwdton", &self.wwdton())
            .field("wwdtie", &self.wwdtie())
            .field("rev0", &self.rev0())
            .field("wwdtpr", &self.wwdtpr())
            .field("wwdtrlr", &self.wwdtrlr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn wwdtrlr(&mut self) -> WwdtrlrW<'_, CrSpec> {
        WwdtrlrW::new(self, 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn wwdtpr(&mut self) -> WwdtprW<'_, CrSpec> {
        WwdtprW::new(self, 8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CrSpec> {
        Rev0W::new(self, 11)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn wwdtie(&mut self) -> WwdtieW<'_, CrSpec> {
        WwdtieW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wwdton(&mut self) -> WwdtonW<'_, CrSpec> {
        WwdtonW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, CrSpec> {
        LockW::new(self, 16)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
