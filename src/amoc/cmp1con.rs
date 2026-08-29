#[doc = "Register `CMP1CON` reader"]
pub type R = crate::R<Cmp1conSpec>;
#[doc = "Register `CMP1CON` writer"]
pub type W = crate::W<Cmp1conSpec>;
#[doc = "Field `C1DEB` reader - "]
pub type C1debR = crate::FieldReader;
#[doc = "Field `C1DEB` writer - "]
pub type C1debW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `C1OUT` reader - "]
pub type C1outR = crate::BitReader;
#[doc = "Field `C1PCHS` reader - "]
pub type C1pchsR = crate::FieldReader;
#[doc = "Field `C1PCHS` writer - "]
pub type C1pchsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `C1NCHS` reader - "]
pub type C1nchsR = crate::BitReader;
#[doc = "Field `C1NCHS` writer - "]
pub type C1nchsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1IES` reader - "]
pub type C1iesR = crate::FieldReader;
#[doc = "Field `C1IES` writer - "]
pub type C1iesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `C1OUTEN` reader - "]
pub type C1outenR = crate::BitReader;
#[doc = "Field `C1OUTEN` writer - "]
pub type C1outenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1SMT` reader - "]
pub type C1smtR = crate::BitReader;
#[doc = "Field `C1SMT` writer - "]
pub type C1smtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1EN` reader - "]
pub type Cmp1enR = crate::BitReader;
#[doc = "Field `CMP1EN` writer - "]
pub type Cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader<u16>;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CMP1VRS` reader - "]
pub type Cmp1vrsR = crate::FieldReader;
#[doc = "Field `CMP1VRS` writer - "]
pub type Cmp1vrsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CP1NOUTEN` reader - "]
pub type Cp1noutenR = crate::BitReader;
#[doc = "Field `CP1NOUTEN` writer - "]
pub type Cp1noutenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn c1deb(&self) -> C1debR {
        C1debR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn c1out(&self) -> C1outR {
        C1outR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn c1pchs(&self) -> C1pchsR {
        C1pchsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn c1nchs(&self) -> C1nchsR {
        C1nchsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn c1ies(&self) -> C1iesR {
        C1iesR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn c1outen(&self) -> C1outenR {
        C1outenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn c1smt(&self) -> C1smtR {
        C1smtR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cmp1en(&self) -> Cmp1enR {
        Cmp1enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn cmp1vrs(&self) -> Cmp1vrsR {
        Cmp1vrsR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cp1nouten(&self) -> Cp1noutenR {
        Cp1noutenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMP1CON")
            .field("rev0", &self.rev0())
            .field("cp1nouten", &self.cp1nouten())
            .field("cmp1vrs", &self.cmp1vrs())
            .field("rev1", &self.rev1())
            .field("cmp1en", &self.cmp1en())
            .field("rev2", &self.rev2())
            .field("c1smt", &self.c1smt())
            .field("c1outen", &self.c1outen())
            .field("c1ies", &self.c1ies())
            .field("rev3", &self.rev3())
            .field("c1nchs", &self.c1nchs())
            .field("c1pchs", &self.c1pchs())
            .field("c1out", &self.c1out())
            .field("c1deb", &self.c1deb())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn c1deb(&mut self) -> C1debW<'_, Cmp1conSpec> {
        C1debW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn c1pchs(&mut self) -> C1pchsW<'_, Cmp1conSpec> {
        C1pchsW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn c1nchs(&mut self) -> C1nchsW<'_, Cmp1conSpec> {
        C1nchsW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, Cmp1conSpec> {
        Rev3W::new(self, 9)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn c1ies(&mut self) -> C1iesW<'_, Cmp1conSpec> {
        C1iesW::new(self, 10)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn c1outen(&mut self) -> C1outenW<'_, Cmp1conSpec> {
        C1outenW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn c1smt(&mut self) -> C1smtW<'_, Cmp1conSpec> {
        C1smtW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, Cmp1conSpec> {
        Rev2W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cmp1en(&mut self) -> Cmp1enW<'_, Cmp1conSpec> {
        Cmp1enW::new(self, 15)
    }
    #[doc = "Bits 16:25"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Cmp1conSpec> {
        Rev1W::new(self, 16)
    }
    #[doc = "Bits 26:29"]
    #[inline(always)]
    pub fn cmp1vrs(&mut self) -> Cmp1vrsW<'_, Cmp1conSpec> {
        Cmp1vrsW::new(self, 26)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn cp1nouten(&mut self) -> Cp1noutenW<'_, Cmp1conSpec> {
        Cp1noutenW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Cmp1conSpec> {
        Rev0W::new(self, 31)
    }
}
#[doc = "CMP1CON\n\nYou can [`read`](crate::Reg::read) this register and get [`cmp1con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmp1con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmp1conSpec;
impl crate::RegisterSpec for Cmp1conSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmp1con::R`](R) reader structure"]
impl crate::Readable for Cmp1conSpec {}
#[doc = "`write(|w| ..)` method takes [`cmp1con::W`](W) writer structure"]
impl crate::Writable for Cmp1conSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMP1CON to value 0"]
impl crate::Resettable for Cmp1conSpec {}
