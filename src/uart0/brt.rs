#[doc = "Register `BRT` reader"]
pub type R = crate::R<BrtSpec>;
#[doc = "Register `BRT` writer"]
pub type W = crate::W<BrtSpec>;
#[doc = "Field `SBRT` reader - "]
pub type SbrtR = crate::FieldReader<u16>;
#[doc = "Field `SBRT` writer - "]
pub type SbrtW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BFINE` reader - "]
pub type BfineR = crate::FieldReader;
#[doc = "Field `BFINE` writer - "]
pub type BfineW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn sbrt(&self) -> SbrtR {
        SbrtR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn bfine(&self) -> BfineR {
        BfineR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14"]
    #[inline(always)]
    pub fn sbrt(&mut self) -> SbrtW<'_, BrtSpec> {
        SbrtW::new(self, 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, BrtSpec> {
        Rev1W::new(self, 15)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn bfine(&mut self) -> BfineW<'_, BrtSpec> {
        BfineW::new(self, 16)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, BrtSpec> {
        Rev0W::new(self, 20)
    }
}
#[doc = "BRT\n\nYou can [`read`](crate::Reg::read) this register and get [`brt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrtSpec;
impl crate::RegisterSpec for BrtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brt::R`](R) reader structure"]
impl crate::Readable for BrtSpec {}
#[doc = "`write(|w| ..)` method takes [`brt::W`](W) writer structure"]
impl crate::Writable for BrtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRT to value 0"]
impl crate::Resettable for BrtSpec {}
