#[doc = "Register `ADCON2` reader"]
pub type R = crate::R<Adcon2Spec>;
#[doc = "Register `ADCON2` writer"]
pub type W = crate::W<Adcon2Spec>;
#[doc = "Field `TADC` reader - "]
pub type TadcR = crate::FieldReader;
#[doc = "Field `TADC` writer - "]
pub type TadcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADMAXCH` reader - "]
pub type AdmaxchR = crate::FieldReader;
#[doc = "Field `ADMAXCH` writer - "]
pub type AdmaxchW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TS` reader - "]
pub type TsR = crate::FieldReader;
#[doc = "Field `TS` writer - "]
pub type TsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TGAP` reader - "]
pub type TgapR = crate::FieldReader;
#[doc = "Field `TGAP` writer - "]
pub type TgapW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADMAXCH1` reader - "]
pub type Admaxch1R = crate::FieldReader;
#[doc = "Field `ADMAXCH1` writer - "]
pub type Admaxch1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADMAXCH2` reader - "]
pub type Admaxch2R = crate::FieldReader;
#[doc = "Field `ADMAXCH2` writer - "]
pub type Admaxch2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ADOUT` reader - "]
pub type AdoutR = crate::BitReader;
#[doc = "Field `ADOUT` writer - "]
pub type AdoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tadc(&self) -> TadcR {
        TadcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn admaxch(&self) -> AdmaxchR {
        AdmaxchR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn tgap(&self) -> TgapR {
        TgapR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn admaxch1(&self) -> Admaxch1R {
        Admaxch1R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn admaxch2(&self) -> Admaxch2R {
        Admaxch2R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adout(&self) -> AdoutR {
        AdoutR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn tadc(&mut self) -> TadcW<'_, Adcon2Spec> {
        TadcW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn admaxch(&mut self) -> AdmaxchW<'_, Adcon2Spec> {
        AdmaxchW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn ts(&mut self) -> TsW<'_, Adcon2Spec> {
        TsW::new(self, 8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Adcon2Spec> {
        Rev1W::new(self, 12)
    }
    #[doc = "Bits 17:19"]
    #[inline(always)]
    pub fn tgap(&mut self) -> TgapW<'_, Adcon2Spec> {
        TgapW::new(self, 17)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn admaxch1(&mut self) -> Admaxch1W<'_, Adcon2Spec> {
        Admaxch1W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn admaxch2(&mut self) -> Admaxch2W<'_, Adcon2Spec> {
        Admaxch2W::new(self, 24)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn adout(&mut self) -> AdoutW<'_, Adcon2Spec> {
        AdoutW::new(self, 28)
    }
    #[doc = "Bits 29:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Adcon2Spec> {
        Rev0W::new(self, 29)
    }
}
#[doc = "ADCON2\n\nYou can [`read`](crate::Reg::read) this register and get [`adcon2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcon2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcon2Spec;
impl crate::RegisterSpec for Adcon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcon2::R`](R) reader structure"]
impl crate::Readable for Adcon2Spec {}
#[doc = "`write(|w| ..)` method takes [`adcon2::W`](W) writer structure"]
impl crate::Writable for Adcon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCON2 to value 0"]
impl crate::Resettable for Adcon2Spec {}
