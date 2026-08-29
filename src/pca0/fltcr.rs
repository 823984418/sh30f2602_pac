#[doc = "Register `FLTCR` reader"]
pub type R = crate::R<FltcrSpec>;
#[doc = "Register `FLTCR` writer"]
pub type W = crate::W<FltcrSpec>;
#[doc = "Field `FLTSTAT` reader - "]
pub type FltstatR = crate::BitReader;
#[doc = "Field `FLTSTAT` writer - "]
pub type FltstatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTM` reader - "]
pub type FltmR = crate::BitReader;
#[doc = "Field `FLTM` writer - "]
pub type FltmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2S` reader - "]
pub type Flt2sR = crate::BitReader;
#[doc = "Field `FLT2S` writer - "]
pub type Flt2sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2EN` reader - "]
pub type Flt2enR = crate::BitReader;
#[doc = "Field `FLT2EN` writer - "]
pub type Flt2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2DEB` reader - "]
pub type Flt2debR = crate::FieldReader;
#[doc = "Field `FLT2DEB` writer - "]
pub type Flt2debW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT1SEL` reader - "]
pub type Flt1selR = crate::FieldReader;
#[doc = "Field `FLT1SEL` writer - "]
pub type Flt1selW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLT1EN` reader - "]
pub type Flt1enR = crate::BitReader;
#[doc = "Field `FLT1EN` writer - "]
pub type Flt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOUT` reader - "]
pub type FoutR = crate::FieldReader;
#[doc = "Field `FOUT` writer - "]
pub type FoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLTIE` reader - "]
pub type FltieR = crate::BitReader;
#[doc = "Field `FLTIE` writer - "]
pub type FltieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fltstat(&self) -> FltstatR {
        FltstatR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fltm(&self) -> FltmR {
        FltmR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flt2s(&self) -> Flt2sR {
        Flt2sR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flt2en(&self) -> Flt2enR {
        Flt2enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn flt2deb(&self) -> Flt2debR {
        Flt2debR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn flt1sel(&self) -> Flt1selR {
        Flt1selR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn flt1en(&self) -> Flt1enR {
        Flt1enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn fout(&self) -> FoutR {
        FoutR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fltie(&self) -> FltieR {
        FltieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLTCR")
            .field("rev0", &self.rev0())
            .field("fltie", &self.fltie())
            .field("fout", &self.fout())
            .field("rev1", &self.rev1())
            .field("flt1en", &self.flt1en())
            .field("flt1sel", &self.flt1sel())
            .field("flt2deb", &self.flt2deb())
            .field("flt2en", &self.flt2en())
            .field("flt2s", &self.flt2s())
            .field("fltm", &self.fltm())
            .field("fltstat", &self.fltstat())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fltstat(&mut self) -> FltstatW<'_, FltcrSpec> {
        FltstatW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fltm(&mut self) -> FltmW<'_, FltcrSpec> {
        FltmW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flt2s(&mut self) -> Flt2sW<'_, FltcrSpec> {
        Flt2sW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> Flt2enW<'_, FltcrSpec> {
        Flt2enW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn flt2deb(&mut self) -> Flt2debW<'_, FltcrSpec> {
        Flt2debW::new(self, 4)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn flt1sel(&mut self) -> Flt1selW<'_, FltcrSpec> {
        Flt1selW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> Flt1enW<'_, FltcrSpec> {
        Flt1enW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, FltcrSpec> {
        Rev1W::new(self, 11)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn fout(&mut self) -> FoutW<'_, FltcrSpec> {
        FoutW::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn fltie(&mut self) -> FltieW<'_, FltcrSpec> {
        FltieW::new(self, 14)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, FltcrSpec> {
        Rev0W::new(self, 15)
    }
}
#[doc = "FLTCR\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltcrSpec;
impl crate::RegisterSpec for FltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltcr::R`](R) reader structure"]
impl crate::Readable for FltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fltcr::W`](W) writer structure"]
impl crate::Writable for FltcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FltcrSpec {}
