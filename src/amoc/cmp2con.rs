#[doc = "Register `CMP2CON` reader"]
pub type R = crate::R<Cmp2conSpec>;
#[doc = "Register `CMP2CON` writer"]
pub type W = crate::W<Cmp2conSpec>;
#[doc = "Field `C2DEB` reader - "]
pub type C2debR = crate::FieldReader;
#[doc = "Field `C2DEB` writer - "]
pub type C2debW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C2OUT` reader - "]
pub type C2outR = crate::BitReader;
#[doc = "Field `C2PCHS` reader - "]
pub type C2pchsR = crate::FieldReader;
#[doc = "Field `C2PCHS` writer - "]
pub type C2pchsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C2NCHS` reader - "]
pub type C2nchsR = crate::BitReader;
#[doc = "Field `C2NCHS` writer - "]
pub type C2nchsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2IES` reader - "]
pub type C2iesR = crate::FieldReader;
#[doc = "Field `C2IES` writer - "]
pub type C2iesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C2OUTEN` reader - "]
pub type C2outenR = crate::BitReader;
#[doc = "Field `C2OUTEN` writer - "]
pub type C2outenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2SMT` reader - "]
pub type C2smtR = crate::BitReader;
#[doc = "Field `C2SMT` writer - "]
pub type C2smtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP2EN` reader - "]
pub type Cmp2enR = crate::BitReader;
#[doc = "Field `CMP2EN` writer - "]
pub type Cmp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader<u16>;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CMP2VRS` reader - "]
pub type Cmp2vrsR = crate::FieldReader;
#[doc = "Field `CMP2VRS` writer - "]
pub type Cmp2vrsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CP2NOUTEN` reader - "]
pub type Cp2noutenR = crate::BitReader;
#[doc = "Field `CP2NOUTEN` writer - "]
pub type Cp2noutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn c2deb(&self) -> C2debR {
        C2debR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c2out(&self) -> C2outR {
        C2outR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn c2pchs(&self) -> C2pchsR {
        C2pchsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn c2nchs(&self) -> C2nchsR {
        C2nchsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn c2ies(&self) -> C2iesR {
        C2iesR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn c2outen(&self) -> C2outenR {
        C2outenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn c2smt(&self) -> C2smtR {
        C2smtR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cmp2en(&self) -> Cmp2enR {
        Cmp2enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn cmp2vrs(&self) -> Cmp2vrsR {
        Cmp2vrsR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cp2nouten(&self) -> Cp2noutenR {
        Cp2noutenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP2CON")
            .field("rev0", &self.rev0())
            .field("cp2nouten", &self.cp2nouten())
            .field("cmp2vrs", &self.cmp2vrs())
            .field("rev1", &self.rev1())
            .field("cmp2en", &self.cmp2en())
            .field("rev2", &self.rev2())
            .field("c2smt", &self.c2smt())
            .field("c2outen", &self.c2outen())
            .field("c2ies", &self.c2ies())
            .field("rev3", &self.rev3())
            .field("c2nchs", &self.c2nchs())
            .field("c2pchs", &self.c2pchs())
            .field("c2out", &self.c2out())
            .field("c2deb", &self.c2deb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn c2deb(&mut self) -> C2debW<'_, Cmp2conSpec> {
        C2debW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn c2pchs(&mut self) -> C2pchsW<'_, Cmp2conSpec> {
        C2pchsW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn c2nchs(&mut self) -> C2nchsW<'_, Cmp2conSpec> {
        C2nchsW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, Cmp2conSpec> {
        Rev3W::new(self, 9)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn c2ies(&mut self) -> C2iesW<'_, Cmp2conSpec> {
        C2iesW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn c2outen(&mut self) -> C2outenW<'_, Cmp2conSpec> {
        C2outenW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn c2smt(&mut self) -> C2smtW<'_, Cmp2conSpec> {
        C2smtW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, Cmp2conSpec> {
        Rev2W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cmp2en(&mut self) -> Cmp2enW<'_, Cmp2conSpec> {
        Cmp2enW::new(self, 15)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Cmp2conSpec> {
        Rev1W::new(self, 16)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn cmp2vrs(&mut self) -> Cmp2vrsW<'_, Cmp2conSpec> {
        Cmp2vrsW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cp2nouten(&mut self) -> Cp2noutenW<'_, Cmp2conSpec> {
        Cp2noutenW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Cmp2conSpec> {
        Rev0W::new(self, 31)
    }
}
#[doc = "CMP2CON\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp2con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp2con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp2conSpec;
impl crate::RegisterSpec for Cmp2conSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp2con::R`](R) reader structure"]
impl crate::Readable for Cmp2conSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp2con::W`](W) writer structure"]
impl crate::Writable for Cmp2conSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP2CON to value 0"]
impl crate::Resettable for Cmp2conSpec {}
