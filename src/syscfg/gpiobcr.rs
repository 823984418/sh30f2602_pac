#[doc = "Register `GPIOBCR` reader"]
pub type R = crate::R<GpiobcrSpec>;
#[doc = "Register `GPIOBCR` writer"]
pub type W = crate::W<GpiobcrSpec>;
#[doc = "Field `BUS` reader - "]
pub type BusR = crate::BitReader;
#[doc = "Field `BUS` writer - "]
pub type BusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bus(&self) -> BusR {
        BusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bus(&mut self) -> BusW<'_, GpiobcrSpec> {
        BusW::new(self, 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, GpiobcrSpec> {
        Rev0W::new(self, 1)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, GpiobcrSpec> {
        LockW::new(self, 16)
    }
}
#[doc = "GPIOBCR\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiobcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpiobcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiobcrSpec;
impl crate::RegisterSpec for GpiobcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiobcr::R`](R) reader structure"]
impl crate::Readable for GpiobcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gpiobcr::W`](W) writer structure"]
impl crate::Writable for GpiobcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIOBCR to value 0x01"]
impl crate::Resettable for GpiobcrSpec {
    const RESET_VALUE: u32 = 0x01;
}
