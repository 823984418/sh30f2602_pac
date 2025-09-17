#[doc = "Register `RCCLOCK` reader"]
pub type R = crate::R<RcclockSpec>;
#[doc = "Register `RCCLOCK` writer"]
pub type W = crate::W<RcclockSpec>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 0xffff) as u16)
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
    pub fn lock(&mut self) -> LockW<'_, RcclockSpec> {
        LockW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, RcclockSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "RCCLOCK\n\nYou can [`read`](crate::Reg::read) this register and get [`rcclock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcclock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RcclockSpec;
impl crate::RegisterSpec for RcclockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcclock::R`](R) reader structure"]
impl crate::Readable for RcclockSpec {}
#[doc = "`write(|w| ..)` method takes [`rcclock::W`](W) writer structure"]
impl crate::Writable for RcclockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RCCLOCK to value 0"]
impl crate::Resettable for RcclockSpec {}
