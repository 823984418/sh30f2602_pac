#[doc = "Register `CIENR` reader"]
pub type R = crate::R<CienrSpec>;
#[doc = "Register `CIENR` writer"]
pub type W = crate::W<CienrSpec>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYIE` reader - "]
pub type PllrdyieR = crate::BitReader;
#[doc = "Field `PLLRDYIE` writer - "]
pub type PllrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pllrdyie(&self) -> PllrdyieR {
        PllrdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIENR")
            .field("rev0", &self.rev0())
            .field("pllrdyie", &self.pllrdyie())
            .field("rev1", &self.rev1())
            .field("rev2", &self.rev2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CienrSpec> {
        Rev2W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CienrSpec> {
        Rev1W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pllrdyie(&mut self) -> PllrdyieW<'_, CienrSpec> {
        PllrdyieW::new(self, 4)
    }
    #[doc = "Bits 5:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CienrSpec> {
        Rev0W::new(self, 5)
    }
}
#[doc = "CIENR\n\nYou can [`read`](crate::Reg::read) this register and get [`cienr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cienr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CienrSpec;
impl crate::RegisterSpec for CienrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cienr::R`](R) reader structure"]
impl crate::Readable for CienrSpec {}
#[doc = "`write(|w| ..)` method takes [`cienr::W`](W) writer structure"]
impl crate::Writable for CienrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIENR to value 0"]
impl crate::Resettable for CienrSpec {}
