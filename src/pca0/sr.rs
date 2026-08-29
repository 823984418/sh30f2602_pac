#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `CC0IF` reader - "]
pub type Cc0ifR = crate::BitReader;
#[doc = "Field `CC1IF` reader - "]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC2IF` reader - "]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `FLTIF` reader - "]
pub type FltifR = crate::BitReader;
#[doc = "Field `PIF` reader - "]
pub type PifR = crate::BitReader;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::FieldReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CIF` reader - "]
pub type CifR = crate::BitReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CC0IFC` reader - "]
pub type Cc0ifcR = crate::BitReader;
#[doc = "Field `CC0IFC` writer - "]
pub type Cc0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC1IFC` reader - "]
pub type Cc1ifcR = crate::BitReader;
#[doc = "Field `CC1IFC` writer - "]
pub type Cc1ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IFC` reader - "]
pub type Cc2ifcR = crate::BitReader;
#[doc = "Field `CC2IFC` writer - "]
pub type Cc2ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTIFC` reader - "]
pub type FltifcR = crate::BitReader;
#[doc = "Field `FLTIFC` writer - "]
pub type FltifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PIFC` reader - "]
pub type PifcR = crate::BitReader;
#[doc = "Field `PIFC` writer - "]
pub type PifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CIFC` reader - "]
pub type CifcR = crate::BitReader;
#[doc = "Field `CIFC` writer - "]
pub type CifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cc0if(&self) -> Cc0ifR {
        Cc0ifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn fltif(&self) -> FltifR {
        FltifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn pif(&self) -> PifR {
        PifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cif(&self) -> CifR {
        CifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cc0ifc(&self) -> Cc0ifcR {
        Cc0ifcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cc1ifc(&self) -> Cc1ifcR {
        Cc1ifcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cc2ifc(&self) -> Cc2ifcR {
        Cc2ifcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn fltifc(&self) -> FltifcR {
        FltifcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pifc(&self) -> PifcR {
        PifcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cifc(&self) -> CifcR {
        CifcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rev0", &self.rev0())
            .field("cifc", &self.cifc())
            .field("rev1", &self.rev1())
            .field("pifc", &self.pifc())
            .field("fltifc", &self.fltifc())
            .field("cc2ifc", &self.cc2ifc())
            .field("cc1ifc", &self.cc1ifc())
            .field("cc0ifc", &self.cc0ifc())
            .field("rev2", &self.rev2())
            .field("cif", &self.cif())
            .field("rev3", &self.rev3())
            .field("pif", &self.pif())
            .field("fltif", &self.fltif())
            .field("cc2if", &self.cc2if())
            .field("cc1if", &self.cc1if())
            .field("cc0if", &self.cc0if())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, SrSpec> {
        Rev3W::new(self, 5)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, SrSpec> {
        Rev2W::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cc0ifc(&mut self) -> Cc0ifcW<'_, SrSpec> {
        Cc0ifcW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cc1ifc(&mut self) -> Cc1ifcW<'_, SrSpec> {
        Cc1ifcW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cc2ifc(&mut self) -> Cc2ifcW<'_, SrSpec> {
        Cc2ifcW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn fltifc(&mut self) -> FltifcW<'_, SrSpec> {
        FltifcW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pifc(&mut self) -> PifcW<'_, SrSpec> {
        PifcW::new(self, 20)
    }
    #[doc = "Bits 21:22"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, SrSpec> {
        Rev1W::new(self, 21)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn cifc(&mut self) -> CifcW<'_, SrSpec> {
        CifcW::new(self, 23)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, SrSpec> {
        Rev0W::new(self, 24)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
