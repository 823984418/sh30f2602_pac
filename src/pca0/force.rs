#[doc = "Register `FORCE` reader"]
pub type R = crate::R<ForceSpec>;
#[doc = "Register `FORCE` writer"]
pub type W = crate::W<ForceSpec>;
#[doc = "Field `FCO0` reader - "]
pub type Fco0R = crate::BitReader;
#[doc = "Field `FCO0` writer - "]
pub type Fco0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCO1` reader - "]
pub type Fco1R = crate::BitReader;
#[doc = "Field `FCO1` writer - "]
pub type Fco1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCO2` reader - "]
pub type Fco2R = crate::BitReader;
#[doc = "Field `FCO2` writer - "]
pub type Fco2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fco0(&self) -> Fco0R {
        Fco0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fco1(&self) -> Fco1R {
        Fco1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fco2(&self) -> Fco2R {
        Fco2R::new(((self.bits >> 2) & 1) != 0)
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
    pub fn fco0(&mut self) -> Fco0W<'_, ForceSpec> {
        Fco0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fco1(&mut self) -> Fco1W<'_, ForceSpec> {
        Fco1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn fco2(&mut self) -> Fco2W<'_, ForceSpec> {
        Fco2W::new(self, 2)
    }
    #[doc = "Bits 3:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, ForceSpec> {
        Rev0W::new(self, 3)
    }
}
#[doc = "FORCE\n\nYou can [`read`](crate::Reg::read) this register and get [`force::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`force::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ForceSpec;
impl crate::RegisterSpec for ForceSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`force::R`](R) reader structure"]
impl crate::Readable for ForceSpec {}
#[doc = "`write(|w| ..)` method takes [`force::W`](W) writer structure"]
impl crate::Writable for ForceSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FORCE to value 0"]
impl crate::Resettable for ForceSpec {}
