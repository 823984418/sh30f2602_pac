#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `HPRE` reader - "]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - "]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PPRE` reader - "]
pub type PpreR = crate::FieldReader;
#[doc = "Field `PPRE` writer - "]
pub type PpreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev4` reader - "]
pub type Rev4R = crate::FieldReader;
#[doc = "Field `rev4` writer - "]
pub type Rev4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLK` reader - "]
pub type PllkR = crate::FieldReader;
#[doc = "Field `PLLK` writer - "]
pub type PllkW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PLLF` reader - "]
pub type PllfR = crate::FieldReader;
#[doc = "Field `PLLF` writer - "]
pub type PllfW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn ppre(&self) -> PpreR {
        PpreR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn pllk(&self) -> PllkR {
        PllkR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn pllf(&self) -> PllfR {
        PllfR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR")
            .field("rev0", &self.rev0())
            .field("rev1", &self.rev1())
            .field("rev2", &self.rev2())
            .field("rev3", &self.rev3())
            .field("pllf", &self.pllf())
            .field("pllk", &self.pllk())
            .field("rev4", &self.rev4())
            .field("ppre", &self.ppre())
            .field("hpre", &self.hpre())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HpreW<'_, CfgrSpec> {
        HpreW::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn ppre(&mut self) -> PpreW<'_, CfgrSpec> {
        PpreW::new(self, 3)
    }
    #[doc = "Bits 6:8"]
    #[inline(always)]
    pub fn rev4(&mut self) -> Rev4W<'_, CfgrSpec> {
        Rev4W::new(self, 6)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn pllk(&mut self) -> PllkW<'_, CfgrSpec> {
        PllkW::new(self, 9)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn pllf(&mut self) -> PllfW<'_, CfgrSpec> {
        PllfW::new(self, 12)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, CfgrSpec> {
        Rev3W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CfgrSpec> {
        Rev2W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CfgrSpec> {
        Rev1W::new(self, 20)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CfgrSpec> {
        Rev0W::new(self, 21)
    }
}
#[doc = "CFGR\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
