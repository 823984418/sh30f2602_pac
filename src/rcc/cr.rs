#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SW` reader - "]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - "]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWS` reader - "]
pub type SwsR = crate::FieldReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `PLLON` reader - "]
pub type PllonR = crate::BitReader;
#[doc = "Field `PLLON` writer - "]
pub type PllonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - "]
pub type PllrdyR = crate::BitReader;
#[doc = "Field `CSMON` reader - "]
pub type CsmonR = crate::BitReader;
#[doc = "Field `CSMON` writer - "]
pub type CsmonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pllon(&self) -> PllonR {
        PllonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pllrdy(&self) -> PllrdyR {
        PllrdyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn csmon(&self) -> CsmonR {
        CsmonR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rev0", &self.rev0())
            .field("csmon", &self.csmon())
            .field("pllrdy", &self.pllrdy())
            .field("pllon", &self.pllon())
            .field("rev1", &self.rev1())
            .field("rev2", &self.rev2())
            .field("sws", &self.sws())
            .field("sw", &self.sw())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<'_, CrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CrSpec> {
        Rev2W::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pllon(&mut self) -> PllonW<'_, CrSpec> {
        PllonW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn csmon(&mut self) -> CsmonW<'_, CrSpec> {
        CsmonW::new(self, 8)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CrSpec> {
        Rev0W::new(self, 9)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
