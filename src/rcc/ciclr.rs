#[doc = "Register `CICLR` reader"]
pub type R = crate::R<CiclrSpec>;
#[doc = "Register `CICLR` writer"]
pub type W = crate::W<CiclrSpec>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::FieldReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDYC` reader - "]
pub type PllrdycR = crate::BitReader;
#[doc = "Field `PLLRDYC` writer - "]
pub type PllrdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSMC` reader - "]
pub type CsmcR = crate::BitReader;
#[doc = "Field `CSMC` writer - "]
pub type CsmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pllrdyc(&self) -> PllrdycR {
        PllrdycR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn csmc(&self) -> CsmcR {
        CsmcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, CiclrSpec> {
        Rev3W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, CiclrSpec> {
        Rev2W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pllrdyc(&mut self) -> PllrdycW<'_, CiclrSpec> {
        PllrdycW::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CiclrSpec> {
        Rev1W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn csmc(&mut self) -> CsmcW<'_, CiclrSpec> {
        CsmcW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CiclrSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "CICLR\n\nYou can [`read`](crate::Reg::read) this register and get [`ciclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CiclrSpec;
impl crate::RegisterSpec for CiclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ciclr::R`](R) reader structure"]
impl crate::Readable for CiclrSpec {}
#[doc = "`write(|w| ..)` method takes [`ciclr::W`](W) writer structure"]
impl crate::Writable for CiclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CICLR to value 0"]
impl crate::Resettable for CiclrSpec {}
