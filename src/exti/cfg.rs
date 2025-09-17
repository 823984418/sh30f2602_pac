#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Field `EXTI0_EN` reader - "]
pub type Exti0EnR = crate::BitReader;
#[doc = "Field `EXTI0_EN` writer - "]
pub type Exti0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI1_EN` reader - "]
pub type Exti1EnR = crate::BitReader;
#[doc = "Field `EXTI1_EN` writer - "]
pub type Exti1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI2_EN` reader - "]
pub type Exti2EnR = crate::BitReader;
#[doc = "Field `EXTI2_EN` writer - "]
pub type Exti2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI3_EN` reader - "]
pub type Exti3EnR = crate::BitReader;
#[doc = "Field `EXTI3_EN` writer - "]
pub type Exti3EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI4_EN` reader - "]
pub type Exti4EnR = crate::BitReader;
#[doc = "Field `EXTI4_EN` writer - "]
pub type Exti4EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI5_EN` reader - "]
pub type Exti5EnR = crate::BitReader;
#[doc = "Field `EXTI5_EN` writer - "]
pub type Exti5EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI6_EN` reader - "]
pub type Exti6EnR = crate::BitReader;
#[doc = "Field `EXTI6_EN` writer - "]
pub type Exti6EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXTI7_EN` reader - "]
pub type Exti7EnR = crate::BitReader;
#[doc = "Field `EXTI7_EN` writer - "]
pub type Exti7EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn exti0_en(&self) -> Exti0EnR {
        Exti0EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn exti1_en(&self) -> Exti1EnR {
        Exti1EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn exti2_en(&self) -> Exti2EnR {
        Exti2EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn exti3_en(&self) -> Exti3EnR {
        Exti3EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn exti4_en(&self) -> Exti4EnR {
        Exti4EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn exti5_en(&self) -> Exti5EnR {
        Exti5EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn exti6_en(&self) -> Exti6EnR {
        Exti6EnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn exti7_en(&self) -> Exti7EnR {
        Exti7EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn exti0_en(&mut self) -> Exti0EnW<'_, CfgSpec> {
        Exti0EnW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn exti1_en(&mut self) -> Exti1EnW<'_, CfgSpec> {
        Exti1EnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn exti2_en(&mut self) -> Exti2EnW<'_, CfgSpec> {
        Exti2EnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn exti3_en(&mut self) -> Exti3EnW<'_, CfgSpec> {
        Exti3EnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn exti4_en(&mut self) -> Exti4EnW<'_, CfgSpec> {
        Exti4EnW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn exti5_en(&mut self) -> Exti5EnW<'_, CfgSpec> {
        Exti5EnW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn exti6_en(&mut self) -> Exti6EnW<'_, CfgSpec> {
        Exti6EnW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn exti7_en(&mut self) -> Exti7EnW<'_, CfgSpec> {
        Exti7EnW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CfgSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
