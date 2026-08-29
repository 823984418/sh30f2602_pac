#[doc = "Register `LCKR` reader"]
pub type R = crate::R<LckrSpec>;
#[doc = "Register `LCKR` writer"]
pub type W = crate::W<LckrSpec>;
#[doc = "Field `KEY` reader - "]
pub type KeyR = crate::FieldReader<u16>;
#[doc = "Field `KEY` writer - "]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LCKR")
            .field("rev0", &self.rev0())
            .field("key", &self.key())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<'_, LckrSpec> {
        KeyW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, LckrSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "LCKR\n\nYou can [`read`](crate::Reg::read) this register and get [`lckr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lckr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LckrSpec;
impl crate::RegisterSpec for LckrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lckr::R`](R) reader structure"]
impl crate::Readable for LckrSpec {}
#[doc = "`write(|w| ..)` method takes [`lckr::W`](W) writer structure"]
impl crate::Writable for LckrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCKR to value 0"]
impl crate::Resettable for LckrSpec {}
