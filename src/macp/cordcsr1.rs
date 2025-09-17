#[doc = "Register `CORDCSR1` reader"]
pub type R = crate::R<Cordcsr1Spec>;
#[doc = "Register `CORDCSR1` writer"]
pub type W = crate::W<Cordcsr1Spec>;
#[doc = "Field `RUN` reader - "]
pub type RunR = crate::BitReader;
#[doc = "Field `RUN` writer - "]
pub type RunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVF` reader - "]
pub type OvfR = crate::BitReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XYMRS` reader - "]
pub type XymrsR = crate::BitReader;
#[doc = "Field `XYMRS` writer - "]
pub type XymrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE` reader - "]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - "]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn run(&self) -> RunR {
        RunR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xymrs(&self) -> XymrsR {
        XymrsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 7) & 1) != 0)
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
    pub fn run(&mut self) -> RunW<'_, Cordcsr1Spec> {
        RunW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, Cordcsr1Spec> {
        Rev2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn xymrs(&mut self) -> XymrsW<'_, Cordcsr1Spec> {
        XymrsW::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Cordcsr1Spec> {
        Rev1W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, Cordcsr1Spec> {
        ModeW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Cordcsr1Spec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "CORDCSR1\n\nYou can [`read`](crate::Reg::read) this register and get [`cordcsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cordcsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cordcsr1Spec;
impl crate::RegisterSpec for Cordcsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cordcsr1::R`](R) reader structure"]
impl crate::Readable for Cordcsr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cordcsr1::W`](W) writer structure"]
impl crate::Writable for Cordcsr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CORDCSR1 to value 0"]
impl crate::Resettable for Cordcsr1Spec {}
