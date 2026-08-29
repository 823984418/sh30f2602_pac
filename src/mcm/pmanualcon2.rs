#[doc = "Register `PMANUALCON2` reader"]
pub type R = crate::R<Pmanualcon2Spec>;
#[doc = "Register `PMANUALCON2` writer"]
pub type W = crate::W<Pmanualcon2Spec>;
#[doc = "Field `POUT0` reader - "]
pub type Pout0R = crate::BitReader;
#[doc = "Field `POUT0` writer - "]
pub type Pout0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT1` reader - "]
pub type Pout1R = crate::BitReader;
#[doc = "Field `POUT1` writer - "]
pub type Pout1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT2` reader - "]
pub type Pout2R = crate::BitReader;
#[doc = "Field `POUT2` writer - "]
pub type Pout2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT01` reader - "]
pub type Pout01R = crate::BitReader;
#[doc = "Field `POUT01` writer - "]
pub type Pout01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT11` reader - "]
pub type Pout11R = crate::BitReader;
#[doc = "Field `POUT11` writer - "]
pub type Pout11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POUT21` reader - "]
pub type Pout21R = crate::BitReader;
#[doc = "Field `POUT21` writer - "]
pub type Pout21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pout0(&self) -> Pout0R {
        Pout0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pout1(&self) -> Pout1R {
        Pout1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pout2(&self) -> Pout2R {
        Pout2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pout01(&self) -> Pout01R {
        Pout01R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pout11(&self) -> Pout11R {
        Pout11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pout21(&self) -> Pout21R {
        Pout21R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PMANUALCON2")
            .field("rev0", &self.rev0())
            .field("pout21", &self.pout21())
            .field("pout11", &self.pout11())
            .field("pout01", &self.pout01())
            .field("pout2", &self.pout2())
            .field("pout1", &self.pout1())
            .field("pout0", &self.pout0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pout0(&mut self) -> Pout0W<'_, Pmanualcon2Spec> {
        Pout0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pout1(&mut self) -> Pout1W<'_, Pmanualcon2Spec> {
        Pout1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pout2(&mut self) -> Pout2W<'_, Pmanualcon2Spec> {
        Pout2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pout01(&mut self) -> Pout01W<'_, Pmanualcon2Spec> {
        Pout01W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pout11(&mut self) -> Pout11W<'_, Pmanualcon2Spec> {
        Pout11W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn pout21(&mut self) -> Pout21W<'_, Pmanualcon2Spec> {
        Pout21W::new(self, 5)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pmanualcon2Spec> {
        Rev0W::new(self, 6)
    }
}
#[doc = "PMANUALCON2\n\nYou can [`read`](crate::Reg::read) this register and get [`pmanualcon2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmanualcon2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pmanualcon2Spec;
impl crate::RegisterSpec for Pmanualcon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmanualcon2::R`](R) reader structure"]
impl crate::Readable for Pmanualcon2Spec {}
#[doc = "`write(|w| ..)` method takes [`pmanualcon2::W`](W) writer structure"]
impl crate::Writable for Pmanualcon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMANUALCON2 to value 0"]
impl crate::Resettable for Pmanualcon2Spec {}
