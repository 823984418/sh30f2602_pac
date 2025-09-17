#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AhbrstrSpec>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AhbrstrSpec>;
#[doc = "Field `GPIORST` reader - "]
pub type GpiorstR = crate::BitReader;
#[doc = "Field `GPIORST` writer - "]
pub type GpiorstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMRST` reader - "]
pub type McmrstR = crate::BitReader;
#[doc = "Field `MCMRST` writer - "]
pub type McmrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGRST` reader - "]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - "]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MACPRST` reader - "]
pub type MacprstR = crate::BitReader;
#[doc = "Field `MACPRST` writer - "]
pub type MacprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - "]
pub type AdcrstR = crate::BitReader;
#[doc = "Field `ADCRST` writer - "]
pub type AdcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpiorst(&self) -> GpiorstR {
        GpiorstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mcmrst(&self) -> McmrstR {
        McmrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn macprst(&self) -> MacprstR {
        MacprstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adcrst(&self) -> AdcrstR {
        AdcrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gpiorst(&mut self) -> GpiorstW<'_, AhbrstrSpec> {
        GpiorstW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn mcmrst(&mut self) -> McmrstW<'_, AhbrstrSpec> {
        McmrstW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<'_, AhbrstrSpec> {
        SyscfgrstW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, AhbrstrSpec> {
        Rev1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn macprst(&mut self) -> MacprstW<'_, AhbrstrSpec> {
        MacprstW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn adcrst(&mut self) -> AdcrstW<'_, AhbrstrSpec> {
        AdcrstW::new(self, 5)
    }
    #[doc = "Bits 6:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AhbrstrSpec> {
        Rev0W::new(self, 6)
    }
}
#[doc = "AHBRSTR\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbrstrSpec;
impl crate::RegisterSpec for AhbrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AhbrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AhbrstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AhbrstrSpec {}
