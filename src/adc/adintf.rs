#[doc = "Register `ADINTF` reader"]
pub type R = crate::R<AdintfSpec>;
#[doc = "Register `ADINTF` writer"]
pub type W = crate::W<AdintfSpec>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::FieldReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADIF` reader - "]
pub type AdifR = crate::BitReader;
#[doc = "Field `ADIF1` reader - "]
pub type Adif1R = crate::BitReader;
#[doc = "Field `ADIF2` reader - "]
pub type Adif2R = crate::BitReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader<u16>;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADIFC` reader - "]
pub type AdifcR = crate::BitReader;
#[doc = "Field `ADIFC` writer - "]
pub type AdifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIF1C` reader - "]
pub type Adif1cR = crate::BitReader;
#[doc = "Field `ADIF1C` writer - "]
pub type Adif1cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADIF2C` reader - "]
pub type Adif2cR = crate::BitReader;
#[doc = "Field `ADIF2C` writer - "]
pub type Adif2cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn adif(&self) -> AdifR {
        AdifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn adif1(&self) -> Adif1R {
        Adif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn adif2(&self) -> Adif2R {
        Adif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adifc(&self) -> AdifcR {
        AdifcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adif1c(&self) -> Adif1cR {
        Adif1cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adif2c(&self) -> Adif2cR {
        Adif2cR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, AdintfSpec> {
        Rev3W::new(self, 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, AdintfSpec> {
        Rev2W::new(self, 5)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, AdintfSpec> {
        Rev1W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn adifc(&mut self) -> AdifcW<'_, AdintfSpec> {
        AdifcW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn adif1c(&mut self) -> Adif1cW<'_, AdintfSpec> {
        Adif1cW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn adif2c(&mut self) -> Adif2cW<'_, AdintfSpec> {
        Adif2cW::new(self, 20)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AdintfSpec> {
        Rev0W::new(self, 21)
    }
}
#[doc = "ADINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`adintf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adintf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdintfSpec;
impl crate::RegisterSpec for AdintfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adintf::R`](R) reader structure"]
impl crate::Readable for AdintfSpec {}
#[doc = "`write(|w| ..)` method takes [`adintf::W`](W) writer structure"]
impl crate::Writable for AdintfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADINTF to value 0"]
impl crate::Resettable for AdintfSpec {}
