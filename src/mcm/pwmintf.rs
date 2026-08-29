#[doc = "Register `PWMINTF` reader"]
pub type R = crate::R<PwmintfSpec>;
#[doc = "Register `PWMINTF` writer"]
pub type W = crate::W<PwmintfSpec>;
#[doc = "Field `PTUD0IF` reader - "]
pub type Ptud0ifR = crate::BitReader;
#[doc = "Field `PTDD0IF` reader - "]
pub type Ptdd0ifR = crate::BitReader;
#[doc = "Field `PTUD1IF` reader - "]
pub type Ptud1ifR = crate::BitReader;
#[doc = "Field `PTDD1IF` reader - "]
pub type Ptdd1ifR = crate::BitReader;
#[doc = "Field `PTUD2IF` reader - "]
pub type Ptud2ifR = crate::BitReader;
#[doc = "Field `PTDD2IF` reader - "]
pub type Ptdd2ifR = crate::BitReader;
#[doc = "Field `PWMZIF` reader - "]
pub type PwmzifR = crate::BitReader;
#[doc = "Field `PWMPIF` reader - "]
pub type PwmpifR = crate::BitReader;
#[doc = "Field `FLTIF` reader - "]
pub type FltifR = crate::BitReader;
#[doc = "Field `FLTCMP0IF` reader - "]
pub type Fltcmp0ifR = crate::BitReader;
#[doc = "Field `FLTCMP1IF` reader - "]
pub type Fltcmp1ifR = crate::BitReader;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `OSTDF` reader - "]
pub type OstdfR = crate::BitReader;
#[doc = "Field `FLTCMP2IF` reader - "]
pub type Fltcmp2ifR = crate::BitReader;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PTUD0IFC` reader - "]
pub type Ptud0ifcR = crate::BitReader;
#[doc = "Field `PTUD0IFC` writer - "]
pub type Ptud0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTDD0IFC` reader - "]
pub type Ptdd0ifcR = crate::BitReader;
#[doc = "Field `PTDD0IFC` writer - "]
pub type Ptdd0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTUD1IFC` reader - "]
pub type Ptud1ifcR = crate::BitReader;
#[doc = "Field `PTUD1IFC` writer - "]
pub type Ptud1ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTDD1IFC` reader - "]
pub type Ptdd1ifcR = crate::BitReader;
#[doc = "Field `PTDD1IFC` writer - "]
pub type Ptdd1ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTUD2IFC` reader - "]
pub type Ptud2ifcR = crate::BitReader;
#[doc = "Field `PTUD2IFC` writer - "]
pub type Ptud2ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTDD2IFC` reader - "]
pub type Ptdd2ifcR = crate::BitReader;
#[doc = "Field `PTDD2IFC` writer - "]
pub type Ptdd2ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMZIFC` reader - "]
pub type PwmzifcR = crate::BitReader;
#[doc = "Field `PWMZIFC` writer - "]
pub type PwmzifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWMPIFC` reader - "]
pub type PwmpifcR = crate::BitReader;
#[doc = "Field `PWMPIFC` writer - "]
pub type PwmpifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTIFC` reader - "]
pub type FltifcR = crate::BitReader;
#[doc = "Field `FLTIFC` writer - "]
pub type FltifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCMP0IFC` reader - "]
pub type Fltcmp0ifcR = crate::BitReader;
#[doc = "Field `FLTCMP0IFC` writer - "]
pub type Fltcmp0ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCMP1IFC` reader - "]
pub type Fltcmp1ifcR = crate::BitReader;
#[doc = "Field `FLTCMP1IFC` writer - "]
pub type Fltcmp1ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSTDFC` reader - "]
pub type OstdfcR = crate::BitReader;
#[doc = "Field `OSTDFC` writer - "]
pub type OstdfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTCMP2IFC` reader - "]
pub type Fltcmp2ifcR = crate::BitReader;
#[doc = "Field `FLTCMP2IFC` writer - "]
pub type Fltcmp2ifcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ptud0if(&self) -> Ptud0ifR {
        Ptud0ifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ptdd0if(&self) -> Ptdd0ifR {
        Ptdd0ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ptud1if(&self) -> Ptud1ifR {
        Ptud1ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ptdd1if(&self) -> Ptdd1ifR {
        Ptdd1ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ptud2if(&self) -> Ptud2ifR {
        Ptud2ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ptdd2if(&self) -> Ptdd2ifR {
        Ptdd2ifR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn pwmzif(&self) -> PwmzifR {
        PwmzifR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn pwmpif(&self) -> PwmpifR {
        PwmpifR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn fltif(&self) -> FltifR {
        FltifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn fltcmp0if(&self) -> Fltcmp0ifR {
        Fltcmp0ifR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn fltcmp1if(&self) -> Fltcmp1ifR {
        Fltcmp1ifR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ostdf(&self) -> OstdfR {
        OstdfR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn fltcmp2if(&self) -> Fltcmp2ifR {
        Fltcmp2ifR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ptud0ifc(&self) -> Ptud0ifcR {
        Ptud0ifcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ptdd0ifc(&self) -> Ptdd0ifcR {
        Ptdd0ifcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ptud1ifc(&self) -> Ptud1ifcR {
        Ptud1ifcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ptdd1ifc(&self) -> Ptdd1ifcR {
        Ptdd1ifcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ptud2ifc(&self) -> Ptud2ifcR {
        Ptud2ifcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ptdd2ifc(&self) -> Ptdd2ifcR {
        Ptdd2ifcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pwmzifc(&self) -> PwmzifcR {
        PwmzifcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pwmpifc(&self) -> PwmpifcR {
        PwmpifcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fltifc(&self) -> FltifcR {
        FltifcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fltcmp0ifc(&self) -> Fltcmp0ifcR {
        Fltcmp0ifcR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fltcmp1ifc(&self) -> Fltcmp1ifcR {
        Fltcmp1ifcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ostdfc(&self) -> OstdfcR {
        OstdfcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fltcmp2ifc(&self) -> Fltcmp2ifcR {
        Fltcmp2ifcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWMINTF")
            .field("rev0", &self.rev0())
            .field("fltcmp2ifc", &self.fltcmp2ifc())
            .field("ostdfc", &self.ostdfc())
            .field("rev1", &self.rev1())
            .field("fltcmp1ifc", &self.fltcmp1ifc())
            .field("fltcmp0ifc", &self.fltcmp0ifc())
            .field("fltifc", &self.fltifc())
            .field("pwmpifc", &self.pwmpifc())
            .field("pwmzifc", &self.pwmzifc())
            .field("ptdd2ifc", &self.ptdd2ifc())
            .field("ptud2ifc", &self.ptud2ifc())
            .field("ptdd1ifc", &self.ptdd1ifc())
            .field("ptud1ifc", &self.ptud1ifc())
            .field("ptdd0ifc", &self.ptdd0ifc())
            .field("ptud0ifc", &self.ptud0ifc())
            .field("rev2", &self.rev2())
            .field("fltcmp2if", &self.fltcmp2if())
            .field("ostdf", &self.ostdf())
            .field("rev3", &self.rev3())
            .field("fltcmp1if", &self.fltcmp1if())
            .field("fltcmp0if", &self.fltcmp0if())
            .field("fltif", &self.fltif())
            .field("pwmpif", &self.pwmpif())
            .field("pwmzif", &self.pwmzif())
            .field("ptdd2if", &self.ptdd2if())
            .field("ptud2if", &self.ptud2if())
            .field("ptdd1if", &self.ptdd1if())
            .field("ptud1if", &self.ptud1if())
            .field("ptdd0if", &self.ptdd0if())
            .field("ptud0if", &self.ptud0if())
            .finish()
    }
}
impl W {
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, PwmintfSpec> {
        Rev2W::new(self, 14)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn ptud0ifc(&mut self) -> Ptud0ifcW<'_, PwmintfSpec> {
        Ptud0ifcW::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn ptdd0ifc(&mut self) -> Ptdd0ifcW<'_, PwmintfSpec> {
        Ptdd0ifcW::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn ptud1ifc(&mut self) -> Ptud1ifcW<'_, PwmintfSpec> {
        Ptud1ifcW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn ptdd1ifc(&mut self) -> Ptdd1ifcW<'_, PwmintfSpec> {
        Ptdd1ifcW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn ptud2ifc(&mut self) -> Ptud2ifcW<'_, PwmintfSpec> {
        Ptud2ifcW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn ptdd2ifc(&mut self) -> Ptdd2ifcW<'_, PwmintfSpec> {
        Ptdd2ifcW::new(self, 21)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pwmzifc(&mut self) -> PwmzifcW<'_, PwmintfSpec> {
        PwmzifcW::new(self, 22)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pwmpifc(&mut self) -> PwmpifcW<'_, PwmintfSpec> {
        PwmpifcW::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn fltifc(&mut self) -> FltifcW<'_, PwmintfSpec> {
        FltifcW::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn fltcmp0ifc(&mut self) -> Fltcmp0ifcW<'_, PwmintfSpec> {
        Fltcmp0ifcW::new(self, 25)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn fltcmp1ifc(&mut self) -> Fltcmp1ifcW<'_, PwmintfSpec> {
        Fltcmp1ifcW::new(self, 26)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, PwmintfSpec> {
        Rev1W::new(self, 27)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn ostdfc(&mut self) -> OstdfcW<'_, PwmintfSpec> {
        OstdfcW::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn fltcmp2ifc(&mut self) -> Fltcmp2ifcW<'_, PwmintfSpec> {
        Fltcmp2ifcW::new(self, 29)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwmintfSpec> {
        Rev0W::new(self, 30)
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
