#[doc = "Register `DIVCSR1` reader"]
pub type R = crate::R<Divcsr1Spec>;
#[doc = "Register `DIVCSR1` writer"]
pub type W = crate::W<Divcsr1Spec>;
#[doc = "Field `RUN` reader - "]
pub type RunR = crate::BitReader;
#[doc = "Field `RUN` writer - "]
pub type RunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIGN` reader - "]
pub type SignR = crate::BitReader;
#[doc = "Field `SIGN` writer - "]
pub type SignW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAT` reader - "]
pub type SatR = crate::BitReader;
#[doc = "Field `SAT` writer - "]
pub type SatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sign(&self) -> SignR {
        SignR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sat(&self) -> SatR {
        SatR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn run(&mut self) -> RunW<'_, Divcsr1Spec> {
        RunW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sign(&mut self) -> SignW<'_, Divcsr1Spec> {
        SignW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn sat(&mut self) -> SatW<'_, Divcsr1Spec> {
        SatW::new(self, 2)
    }
    #[doc = "Bits 3:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Divcsr1Spec> {
        Rev0W::new(self, 3)
    }
}
#[doc = "DIVCSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`divcsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divcsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Divcsr1Spec;
impl crate::RegisterSpec for Divcsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divcsr1::R`](R) reader structure"]
impl crate::Readable for Divcsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`divcsr1::W`](W) writer structure"]
impl crate::Writable for Divcsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DIVCSR1 to value 0"]
impl crate::Resettable for Divcsr1Spec {}
