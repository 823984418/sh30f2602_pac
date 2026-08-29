#[doc = "Register `PMANUALCON1` reader"]
pub type R = crate::R<Pmanualcon1Spec>;
#[doc = "Register `PMANUALCON1` writer"]
pub type W = crate::W<Pmanualcon1Spec>;
#[doc = "Field `PMANUAL0` reader - "]
pub type Pmanual0R = crate::BitReader;
#[doc = "Field `PMANUAL0` writer - "]
pub type Pmanual0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMANUAL1` reader - "]
pub type Pmanual1R = crate::BitReader;
#[doc = "Field `PMANUAL1` writer - "]
pub type Pmanual1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMANUAL2` reader - "]
pub type Pmanual2R = crate::BitReader;
#[doc = "Field `PMANUAL2` writer - "]
pub type Pmanual2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMANUAL01` reader - "]
pub type Pmanual01R = crate::BitReader;
#[doc = "Field `PMANUAL01` writer - "]
pub type Pmanual01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMANUAL11` reader - "]
pub type Pmanual11R = crate::BitReader;
#[doc = "Field `PMANUAL11` writer - "]
pub type Pmanual11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMANUAL21` reader - "]
pub type Pmanual21R = crate::BitReader;
#[doc = "Field `PMANUAL21` writer - "]
pub type Pmanual21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pmanual0(&self) -> Pmanual0R {
        Pmanual0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pmanual1(&self) -> Pmanual1R {
        Pmanual1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pmanual2(&self) -> Pmanual2R {
        Pmanual2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pmanual01(&self) -> Pmanual01R {
        Pmanual01R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pmanual11(&self) -> Pmanual11R {
        Pmanual11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmanual21(&self) -> Pmanual21R {
        Pmanual21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMANUALCON1")
            .field("rev0", &self.rev0())
            .field("pmanual21", &self.pmanual21())
            .field("pmanual11", &self.pmanual11())
            .field("pmanual01", &self.pmanual01())
            .field("pmanual2", &self.pmanual2())
            .field("pmanual1", &self.pmanual1())
            .field("pmanual0", &self.pmanual0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pmanual0(&mut self) -> Pmanual0W<'_, Pmanualcon1Spec> {
        Pmanual0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pmanual1(&mut self) -> Pmanual1W<'_, Pmanualcon1Spec> {
        Pmanual1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pmanual2(&mut self) -> Pmanual2W<'_, Pmanualcon1Spec> {
        Pmanual2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pmanual01(&mut self) -> Pmanual01W<'_, Pmanualcon1Spec> {
        Pmanual01W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pmanual11(&mut self) -> Pmanual11W<'_, Pmanualcon1Spec> {
        Pmanual11W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pmanual21(&mut self) -> Pmanual21W<'_, Pmanualcon1Spec> {
        Pmanual21W::new(self, 5)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pmanualcon1Spec> {
        Rev0W::new(self, 6)
    }
}
#[doc = "PMANUALCON1\n\nYou can [`read`](crate::Reg::read) this register and get [`pmanualcon1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmanualcon1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmanualcon1Spec;
impl crate::RegisterSpec for Pmanualcon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmanualcon1::R`](R) reader structure"]
impl crate::Readable for Pmanualcon1Spec {}
#[doc = "`write(|w| ..)` method takes [`pmanualcon1::W`](W) writer structure"]
impl crate::Writable for Pmanualcon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMANUALCON1 to value 0"]
impl crate::Resettable for Pmanualcon1Spec {}
