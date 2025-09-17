#[doc = "Register `FLTCON` reader"]
pub type R = crate::R<FltconSpec>;
#[doc = "Register `FLTCON` writer"]
pub type W = crate::W<FltconSpec>;
#[doc = "Field `FLT2STAT` reader - "]
pub type Flt2statR = crate::BitReader;
#[doc = "Field `FLT2M` reader - "]
pub type Flt2mR = crate::BitReader;
#[doc = "Field `FLT2M` writer - "]
pub type Flt2mW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `FLT1STAT` reader - "]
pub type Flt1statR = crate::BitReader;
#[doc = "Field `FLT1M` reader - "]
pub type Flt1mR = crate::BitReader;
#[doc = "Field `FLT1M` writer - "]
pub type Flt1mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1S` reader - "]
pub type Flt1sR = crate::BitReader;
#[doc = "Field `FLT1S` writer - "]
pub type Flt1sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1EN` reader - "]
pub type Flt1enR = crate::BitReader;
#[doc = "Field `FLT1EN` writer - "]
pub type Flt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0STAT` reader - "]
pub type Flt0statR = crate::BitReader;
#[doc = "Field `FLT0M` reader - "]
pub type Flt0mR = crate::BitReader;
#[doc = "Field `FLT0M` writer - "]
pub type Flt0mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0S` reader - "]
pub type Flt0sR = crate::BitReader;
#[doc = "Field `FLT0S` writer - "]
pub type Flt0sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0EN` reader - "]
pub type Flt0enR = crate::BitReader;
#[doc = "Field `FLT0EN` writer - "]
pub type Flt0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2STATC` reader - "]
pub type Flt2statcR = crate::BitReader;
#[doc = "Field `FLT2STATC` writer - "]
pub type Flt2statcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1STATC` reader - "]
pub type Flt1statcR = crate::BitReader;
#[doc = "Field `FLT1STATC` writer - "]
pub type Flt1statcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0STATC` reader - "]
pub type Flt0statcR = crate::BitReader;
#[doc = "Field `FLT0STATC` writer - "]
pub type Flt0statcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FOUT0` reader - "]
pub type Fout0R = crate::FieldReader;
#[doc = "Field `FOUT0` writer - "]
pub type Fout0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FOUT1` reader - "]
pub type Fout1R = crate::FieldReader;
#[doc = "Field `FOUT1` writer - "]
pub type Fout1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLT3STAT` reader - "]
pub type Flt3statR = crate::BitReader;
#[doc = "Field `FLT3M` reader - "]
pub type Flt3mR = crate::BitReader;
#[doc = "Field `FLT3M` writer - "]
pub type Flt3mW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3S` reader - "]
pub type Flt3sR = crate::BitReader;
#[doc = "Field `FLT3S` writer - "]
pub type Flt3sW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3EN` reader - "]
pub type Flt3enR = crate::BitReader;
#[doc = "Field `FLT3EN` writer - "]
pub type Flt3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3STATC` reader - "]
pub type Flt3statcR = crate::BitReader;
#[doc = "Field `FLT3STATC` writer - "]
pub type Flt3statcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flt2stat(&self) -> Flt2statR {
        Flt2statR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flt2m(&self) -> Flt2mR {
        Flt2mR::new(((self.bits >> 1) & 1) != 0)
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
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn flt1stat(&self) -> Flt1statR {
        Flt1statR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn flt1m(&self) -> Flt1mR {
        Flt1mR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn flt1s(&self) -> Flt1sR {
        Flt1sR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn flt1en(&self) -> Flt1enR {
        Flt1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn flt0stat(&self) -> Flt0statR {
        Flt0statR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn flt0m(&self) -> Flt0mR {
        Flt0mR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn flt0s(&self) -> Flt0sR {
        Flt0sR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn flt0en(&self) -> Flt0enR {
        Flt0enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn flt2statc(&self) -> Flt2statcR {
        Flt2statcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flt1statc(&self) -> Flt1statcR {
        Flt1statcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn flt0statc(&self) -> Flt0statcR {
        Flt0statcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn fout0(&self) -> Fout0R {
        Fout0R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn fout1(&self) -> Fout1R {
        Fout1R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn flt3stat(&self) -> Flt3statR {
        Flt3statR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn flt3m(&self) -> Flt3mR {
        Flt3mR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn flt3s(&self) -> Flt3sR {
        Flt3sR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn flt3en(&self) -> Flt3enR {
        Flt3enR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn flt3statc(&self) -> Flt3statcR {
        Flt3statcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn flt2m(&mut self) -> Flt2mW<'_, FltconSpec> {
        Flt2mW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flt2s(&mut self) -> Flt2sW<'_, FltconSpec> {
        Flt2sW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> Flt2enW<'_, FltconSpec> {
        Flt2enW::new(self, 3)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn flt2deb(&mut self) -> Flt2debW<'_, FltconSpec> {
        Flt2debW::new(self, 4)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn flt1m(&mut self) -> Flt1mW<'_, FltconSpec> {
        Flt1mW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn flt1s(&mut self) -> Flt1sW<'_, FltconSpec> {
        Flt1sW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> Flt1enW<'_, FltconSpec> {
        Flt1enW::new(self, 11)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn flt0m(&mut self) -> Flt0mW<'_, FltconSpec> {
        Flt0mW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn flt0s(&mut self) -> Flt0sW<'_, FltconSpec> {
        Flt0sW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn flt0en(&mut self) -> Flt0enW<'_, FltconSpec> {
        Flt0enW::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn flt2statc(&mut self) -> Flt2statcW<'_, FltconSpec> {
        Flt2statcW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn flt1statc(&mut self) -> Flt1statcW<'_, FltconSpec> {
        Flt1statcW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn flt0statc(&mut self) -> Flt0statcW<'_, FltconSpec> {
        Flt0statcW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, FltconSpec> {
        Rev1W::new(self, 19)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn fout0(&mut self) -> Fout0W<'_, FltconSpec> {
        Fout0W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn fout1(&mut self) -> Fout1W<'_, FltconSpec> {
        Fout1W::new(self, 22)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn flt3m(&mut self) -> Flt3mW<'_, FltconSpec> {
        Flt3mW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn flt3s(&mut self) -> Flt3sW<'_, FltconSpec> {
        Flt3sW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn flt3en(&mut self) -> Flt3enW<'_, FltconSpec> {
        Flt3enW::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn flt3statc(&mut self) -> Flt3statcW<'_, FltconSpec> {
        Flt3statcW::new(self, 28)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, FltconSpec> {
        Rev0W::new(self, 29)
    }
}
#[doc = "FLTCON\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltconSpec;
impl crate::RegisterSpec for FltconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltcon::R`](R) reader structure"]
impl crate::Readable for FltconSpec {}
#[doc = "`write(|w| ..)` method takes [`fltcon::W`](W) writer structure"]
impl crate::Writable for FltconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTCON to value 0"]
impl crate::Resettable for FltconSpec {}
