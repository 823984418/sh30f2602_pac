#[doc = "Register `PWRSR` reader"]
pub type R = crate::R<PwrsrSpec>;
#[doc = "Register `PWRSR` writer"]
pub type W = crate::W<PwrsrSpec>;
#[doc = "Field `BODIF` reader - "]
pub type BodifR = crate::BitReader;
#[doc = "Field `BODIF` writer - "]
pub type BodifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODF` reader - "]
pub type BodfR = crate::BitReader;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bodif(&self) -> BodifR {
        BodifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn bodf(&self) -> BodfR {
        BodfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRSR")
            .field("rev0", &self.rev0())
            .field("bodf", &self.bodf())
            .field("bodif", &self.bodif())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn bodif(&mut self) -> BodifW<'_, PwrsrSpec> {
        BodifW::new(self, 0)
    }
    #[doc = "Bits 2:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwrsrSpec> {
        Rev0W::new(self, 2)
    }
}
#[doc = "PWRSR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrsrSpec;
impl crate::RegisterSpec for PwrsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrsr::R`](R) reader structure"]
impl crate::Readable for PwrsrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrsr::W`](W) writer structure"]
impl crate::Writable for PwrsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRSR to value 0"]
impl crate::Resettable for PwrsrSpec {}
