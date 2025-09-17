#[doc = "Register `ADCON1` reader"]
pub type R = crate::R<Adcon1Spec>;
#[doc = "Register `ADCON1` writer"]
pub type W = crate::W<Adcon1Spec>;
#[doc = "Field `ADCTU` reader - "]
pub type AdctuR = crate::FieldReader;
#[doc = "Field `ADCTU` writer - "]
pub type AdctuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ADON` reader - "]
pub type AdonR = crate::BitReader;
#[doc = "Field `ADON` writer - "]
pub type AdonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTRS` reader - "]
pub type AdstrsR = crate::FieldReader;
#[doc = "Field `ADSTRS` writer - "]
pub type AdstrsW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADIE` reader - "]
pub type AdieR = crate::BitReader;
#[doc = "Field `ADIE` writer - "]
pub type AdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTRS1` reader - "]
pub type Adstrs1R = crate::FieldReader;
#[doc = "Field `ADSTRS1` writer - "]
pub type Adstrs1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADIE1` reader - "]
pub type Adie1R = crate::BitReader;
#[doc = "Field `ADIE1` writer - "]
pub type Adie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTRS2` reader - "]
pub type Adstrs2R = crate::FieldReader;
#[doc = "Field `ADSTRS2` writer - "]
pub type Adstrs2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ADIE2` reader - "]
pub type Adie2R = crate::BitReader;
#[doc = "Field `ADIE2` writer - "]
pub type Adie2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adctu(&self) -> AdctuR {
        AdctuR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adon(&self) -> AdonR {
        AdonR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn adstrs(&self) -> AdstrsR {
        AdstrsR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adie(&self) -> AdieR {
        AdieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn adstrs1(&self) -> Adstrs1R {
        Adstrs1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adie1(&self) -> Adie1R {
        Adie1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn adstrs2(&self) -> Adstrs2R {
        Adstrs2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adie2(&self) -> Adie2R {
        Adie2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn adctu(&mut self) -> AdctuW<'_, Adcon1Spec> {
        AdctuW::new(self, 0)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Adcon1Spec> {
        Rev0W::new(self, 2)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn adon(&mut self) -> AdonW<'_, Adcon1Spec> {
        AdonW::new(self, 7)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn adstrs(&mut self) -> AdstrsW<'_, Adcon1Spec> {
        AdstrsW::new(self, 8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn adie(&mut self) -> AdieW<'_, Adcon1Spec> {
        AdieW::new(self, 15)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn adstrs1(&mut self) -> Adstrs1W<'_, Adcon1Spec> {
        Adstrs1W::new(self, 16)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn adie1(&mut self) -> Adie1W<'_, Adcon1Spec> {
        Adie1W::new(self, 23)
    }
    #[doc = "Bits 24:30"]
    #[inline(always)]
    pub fn adstrs2(&mut self) -> Adstrs2W<'_, Adcon1Spec> {
        Adstrs2W::new(self, 24)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn adie2(&mut self) -> Adie2W<'_, Adcon1Spec> {
        Adie2W::new(self, 31)
    }
}
#[doc = "ADCON1\n\nYou can [`read`](crate::Reg::read) this register and get [`adcon1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcon1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcon1Spec;
impl crate::RegisterSpec for Adcon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcon1::R`](R) reader structure"]
impl crate::Readable for Adcon1Spec {}
#[doc = "`write(|w| ..)` method takes [`adcon1::W`](W) writer structure"]
impl crate::Writable for Adcon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCON1 to value 0"]
impl crate::Resettable for Adcon1Spec {}
