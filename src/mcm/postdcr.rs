#[doc = "Register `POSTDCR` reader"]
pub type R = crate::R<PostdcrSpec>;
#[doc = "Register `POSTDCR` writer"]
pub type W = crate::W<PostdcrSpec>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OSTDEN` reader - "]
pub type OstdenR = crate::BitReader;
#[doc = "Field `OSTDEN` writer - "]
pub type OstdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ostden(&self) -> OstdenR {
        OstdenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, PostdcrSpec> {
        Rev1W::new(self, 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ostden(&mut self) -> OstdenW<'_, PostdcrSpec> {
        OstdenW::new(self, 7)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PostdcrSpec> {
        Rev0W::new(self, 8)
    }
}
#[doc = "POSTDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`postdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`postdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PostdcrSpec;
impl crate::RegisterSpec for PostdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`postdcr::R`](R) reader structure"]
impl crate::Readable for PostdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`postdcr::W`](W) writer structure"]
impl crate::Writable for PostdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POSTDCR to value 0"]
impl crate::Resettable for PostdcrSpec {}
