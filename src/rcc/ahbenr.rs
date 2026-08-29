#[doc = "Register `AHBENR` reader"]
pub type R = crate::R<AhbenrSpec>;
#[doc = "Register `AHBENR` writer"]
pub type W = crate::W<AhbenrSpec>;
#[doc = "Field `GPIOEN` reader - "]
pub type GpioenR = crate::BitReader;
#[doc = "Field `GPIOEN` writer - "]
pub type GpioenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMEN` reader - "]
pub type McmenR = crate::BitReader;
#[doc = "Field `MCMEN` writer - "]
pub type McmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGEN` reader - "]
pub type SyscfgenR = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - "]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACPEN` reader - "]
pub type MacpenR = crate::BitReader;
#[doc = "Field `MACPEN` writer - "]
pub type MacpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCEN` reader - "]
pub type AdcenR = crate::BitReader;
#[doc = "Field `ADCEN` writer - "]
pub type AdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpioen(&self) -> GpioenR {
        GpioenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mcmen(&self) -> McmenR {
        McmenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn macpen(&self) -> MacpenR {
        MacpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adcen(&self) -> AdcenR {
        AdcenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBENR")
            .field("rev0", &self.rev0())
            .field("adcen", &self.adcen())
            .field("macpen", &self.macpen())
            .field("rev1", &self.rev1())
            .field("syscfgen", &self.syscfgen())
            .field("mcmen", &self.mcmen())
            .field("gpioen", &self.gpioen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpioen(&mut self) -> GpioenW<'_, AhbenrSpec> {
        GpioenW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mcmen(&mut self) -> McmenW<'_, AhbenrSpec> {
        McmenW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<'_, AhbenrSpec> {
        SyscfgenW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, AhbenrSpec> {
        Rev1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn macpen(&mut self) -> MacpenW<'_, AhbenrSpec> {
        MacpenW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adcen(&mut self) -> AdcenW<'_, AhbenrSpec> {
        AdcenW::new(self, 5)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AhbenrSpec> {
        Rev0W::new(self, 6)
    }
}
#[doc = "AHBENR\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbenrSpec;
impl crate::RegisterSpec for AhbenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbenr::R`](R) reader structure"]
impl crate::Readable for AhbenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbenr::W`](W) writer structure"]
impl crate::Writable for AhbenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBENR to value 0"]
impl crate::Resettable for AhbenrSpec {}
