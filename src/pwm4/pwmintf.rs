#[doc = "Register `PWMINTF` reader"]
pub type R = crate::R<PwmintfSpec>;
#[doc = "Register `PWMINTF` writer"]
pub type W = crate::W<PwmintfSpec>;
#[doc = "Field `FLTS` reader - "]
pub type FltsR = crate::BitReader;
#[doc = "Field `PWMIF` reader - "]
pub type PwmifR = crate::BitReader;
#[doc = "Field `FLT0STAT` reader - "]
pub type Flt0statR = crate::BitReader;
#[doc = "Field `FLT1STAT` reader - "]
pub type Flt1statR = crate::BitReader;
#[doc = "Field `FLT2STAT` reader - "]
pub type Flt2statR = crate::BitReader;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader<u16>;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `FLTSC` reader - "]
pub type FltscR = crate::BitReader;
#[doc = "Field `FLTSC` writer - "]
pub type FltscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMIFC` reader - "]
pub type PwmifcR = crate::BitReader;
#[doc = "Field `PWMIFC` writer - "]
pub type PwmifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT0STATC` reader - "]
pub type Flt0statcR = crate::BitReader;
#[doc = "Field `FLT0STATC` writer - "]
pub type Flt0statcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT1STATC` reader - "]
pub type Flt1statcR = crate::BitReader;
#[doc = "Field `FLT1STATC` writer - "]
pub type Flt1statcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2STATC` reader - "]
pub type Flt2statcR = crate::BitReader;
#[doc = "Field `FLT2STATC` writer - "]
pub type Flt2statcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn flts(&self) -> FltsR {
        FltsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn pwmif(&self) -> PwmifR {
        PwmifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn flt0stat(&self) -> Flt0statR {
        Flt0statR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn flt1stat(&self) -> Flt1statR {
        Flt1statR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn flt2stat(&self) -> Flt2statR {
        Flt2statR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 5) & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fltsc(&self) -> FltscR {
        FltscR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwmifc(&self) -> PwmifcR {
        PwmifcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn flt0statc(&self) -> Flt0statcR {
        Flt0statcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flt1statc(&self) -> Flt1statcR {
        Flt1statcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flt2statc(&self) -> Flt2statcR {
        Flt2statcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMINTF")
            .field("rev0", &self.rev0())
            .field("flt2statc", &self.flt2statc())
            .field("flt1statc", &self.flt1statc())
            .field("flt0statc", &self.flt0statc())
            .field("pwmifc", &self.pwmifc())
            .field("fltsc", &self.fltsc())
            .field("rev1", &self.rev1())
            .field("flt2stat", &self.flt2stat())
            .field("flt1stat", &self.flt1stat())
            .field("flt0stat", &self.flt0stat())
            .field("pwmif", &self.pwmif())
            .field("flts", &self.flts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 5:15"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, PwmintfSpec> {
        Rev1W::new(self, 5)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn fltsc(&mut self) -> FltscW<'_, PwmintfSpec> {
        FltscW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn pwmifc(&mut self) -> PwmifcW<'_, PwmintfSpec> {
        PwmifcW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn flt0statc(&mut self) -> Flt0statcW<'_, PwmintfSpec> {
        Flt0statcW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn flt1statc(&mut self) -> Flt1statcW<'_, PwmintfSpec> {
        Flt1statcW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn flt2statc(&mut self) -> Flt2statcW<'_, PwmintfSpec> {
        Flt2statcW::new(self, 20)
    }
    #[doc = "Bits 21:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmintfSpec> {
        Rev0W::new(self, 21)
    }
}
#[doc = "PWMINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmintf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmintf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwmintfSpec;
impl crate::RegisterSpec for PwmintfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmintf::R`](R) reader structure"]
impl crate::Readable for PwmintfSpec {}
#[doc = "`write(|w| ..)` method takes [`pwmintf::W`](W) writer structure"]
impl crate::Writable for PwmintfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMINTF to value 0"]
impl crate::Resettable for PwmintfSpec {}
