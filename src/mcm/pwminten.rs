#[doc = "Register `PWMINTEN` reader"]
pub type R = crate::R<PwmintenSpec>;
#[doc = "Register `PWMINTEN` writer"]
pub type W = crate::W<PwmintenSpec>;
#[doc = "Field `PTUD0IE` reader - "]
pub type Ptud0ieR = crate::BitReader;
#[doc = "Field `PTUD0IE` writer - "]
pub type Ptud0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTDD0IE` reader - "]
pub type Ptdd0ieR = crate::BitReader;
#[doc = "Field `PTDD0IE` writer - "]
pub type Ptdd0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTUD1IE` reader - "]
pub type Ptud1ieR = crate::BitReader;
#[doc = "Field `PTUD1IE` writer - "]
pub type Ptud1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTDD1IE` reader - "]
pub type Ptdd1ieR = crate::BitReader;
#[doc = "Field `PTDD1IE` writer - "]
pub type Ptdd1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTUD2IE` reader - "]
pub type Ptud2ieR = crate::BitReader;
#[doc = "Field `PTUD2IE` writer - "]
pub type Ptud2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTDD2IE` reader - "]
pub type Ptdd2ieR = crate::BitReader;
#[doc = "Field `PTDD2IE` writer - "]
pub type Ptdd2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMZIE` reader - "]
pub type PwmzieR = crate::BitReader;
#[doc = "Field `PWMZIE` writer - "]
pub type PwmzieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMPIE` reader - "]
pub type PwmpieR = crate::BitReader;
#[doc = "Field `PWMPIE` writer - "]
pub type PwmpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTIE` reader - "]
pub type FltieR = crate::BitReader;
#[doc = "Field `FLTIE` writer - "]
pub type FltieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCMP0IE` reader - "]
pub type Fltcmp0ieR = crate::BitReader;
#[doc = "Field `FLTCMP0IE` writer - "]
pub type Fltcmp0ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCMP1IE` reader - "]
pub type Fltcmp1ieR = crate::BitReader;
#[doc = "Field `FLTCMP1IE` writer - "]
pub type Fltcmp1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSTDIE` reader - "]
pub type OstdieR = crate::BitReader;
#[doc = "Field `OSTDIE` writer - "]
pub type OstdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCMP2IE` reader - "]
pub type Fltcmp2ieR = crate::BitReader;
#[doc = "Field `FLTCMP2IE` writer - "]
pub type Fltcmp2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ptud0ie(&self) -> Ptud0ieR {
        Ptud0ieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ptdd0ie(&self) -> Ptdd0ieR {
        Ptdd0ieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ptud1ie(&self) -> Ptud1ieR {
        Ptud1ieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ptdd1ie(&self) -> Ptdd1ieR {
        Ptdd1ieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ptud2ie(&self) -> Ptud2ieR {
        Ptud2ieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ptdd2ie(&self) -> Ptdd2ieR {
        Ptdd2ieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwmzie(&self) -> PwmzieR {
        PwmzieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwmpie(&self) -> PwmpieR {
        PwmpieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fltie(&self) -> FltieR {
        FltieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fltcmp0ie(&self) -> Fltcmp0ieR {
        Fltcmp0ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fltcmp1ie(&self) -> Fltcmp1ieR {
        Fltcmp1ieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ostdie(&self) -> OstdieR {
        OstdieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fltcmp2ie(&self) -> Fltcmp2ieR {
        Fltcmp2ieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMINTEN")
            .field("rev0", &self.rev0())
            .field("fltcmp2ie", &self.fltcmp2ie())
            .field("ostdie", &self.ostdie())
            .field("rev1", &self.rev1())
            .field("fltcmp1ie", &self.fltcmp1ie())
            .field("fltcmp0ie", &self.fltcmp0ie())
            .field("fltie", &self.fltie())
            .field("pwmpie", &self.pwmpie())
            .field("pwmzie", &self.pwmzie())
            .field("ptdd2ie", &self.ptdd2ie())
            .field("ptud2ie", &self.ptud2ie())
            .field("ptdd1ie", &self.ptdd1ie())
            .field("ptud1ie", &self.ptud1ie())
            .field("ptdd0ie", &self.ptdd0ie())
            .field("ptud0ie", &self.ptud0ie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ptud0ie(&mut self) -> Ptud0ieW<'_, PwmintenSpec> {
        Ptud0ieW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ptdd0ie(&mut self) -> Ptdd0ieW<'_, PwmintenSpec> {
        Ptdd0ieW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ptud1ie(&mut self) -> Ptud1ieW<'_, PwmintenSpec> {
        Ptud1ieW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ptdd1ie(&mut self) -> Ptdd1ieW<'_, PwmintenSpec> {
        Ptdd1ieW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ptud2ie(&mut self) -> Ptud2ieW<'_, PwmintenSpec> {
        Ptud2ieW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ptdd2ie(&mut self) -> Ptdd2ieW<'_, PwmintenSpec> {
        Ptdd2ieW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwmzie(&mut self) -> PwmzieW<'_, PwmintenSpec> {
        PwmzieW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwmpie(&mut self) -> PwmpieW<'_, PwmintenSpec> {
        PwmpieW::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fltie(&mut self) -> FltieW<'_, PwmintenSpec> {
        FltieW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fltcmp0ie(&mut self) -> Fltcmp0ieW<'_, PwmintenSpec> {
        Fltcmp0ieW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fltcmp1ie(&mut self) -> Fltcmp1ieW<'_, PwmintenSpec> {
        Fltcmp1ieW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, PwmintenSpec> {
        Rev1W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ostdie(&mut self) -> OstdieW<'_, PwmintenSpec> {
        OstdieW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fltcmp2ie(&mut self) -> Fltcmp2ieW<'_, PwmintenSpec> {
        Fltcmp2ieW::new(self, 13)
    }
    #[doc = "Bits 14:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmintenSpec> {
        Rev0W::new(self, 14)
    }
}
#[doc = "PWMINTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`pwminten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwminten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmintenSpec;
impl crate::RegisterSpec for PwmintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwminten::R`](R) reader structure"]
impl crate::Readable for PwmintenSpec {}
#[doc = "`write(|w| ..)` method takes [`pwminten::W`](W) writer structure"]
impl crate::Writable for PwmintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMINTEN to value 0"]
impl crate::Resettable for PwmintenSpec {}
