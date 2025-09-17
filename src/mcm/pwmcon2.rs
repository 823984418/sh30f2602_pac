#[doc = "Register `PWMCON2` reader"]
pub type R = crate::R<Pwmcon2Spec>;
#[doc = "Register `PWMCON2` writer"]
pub type W = crate::W<Pwmcon2Spec>;
#[doc = "Field `CMP1` reader - "]
pub type Cmp1R = crate::FieldReader;
#[doc = "Field `CMP1` writer - "]
pub type Cmp1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP2` reader - "]
pub type Cmp2R = crate::FieldReader;
#[doc = "Field `CMP2` writer - "]
pub type Cmp2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP3` reader - "]
pub type Cmp3R = crate::FieldReader;
#[doc = "Field `CMP3` writer - "]
pub type Cmp3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CMP4` reader - "]
pub type Cmp4R = crate::FieldReader;
#[doc = "Field `CMP4` writer - "]
pub type Cmp4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OSYNC` reader - "]
pub type OsyncR = crate::BitReader;
#[doc = "Field `OSYNC` writer - "]
pub type OsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DILDEN` reader - "]
pub type DildenR = crate::BitReader;
#[doc = "Field `DILDEN` writer - "]
pub type DildenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CILDEN` reader - "]
pub type CildenR = crate::BitReader;
#[doc = "Field `CILDEN` writer - "]
pub type CildenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZDLDEN` reader - "]
pub type ZdldenR = crate::BitReader;
#[doc = "Field `ZDLDEN` writer - "]
pub type ZdldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDLDEN` reader - "]
pub type PdldenR = crate::BitReader;
#[doc = "Field `PDLDEN` writer - "]
pub type PdldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZCMLDEN` reader - "]
pub type ZcmldenR = crate::BitReader;
#[doc = "Field `ZCMLDEN` writer - "]
pub type ZcmldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCMLDEN` reader - "]
pub type PcmldenR = crate::BitReader;
#[doc = "Field `PCMLDEN` writer - "]
pub type PcmldenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cmp2(&self) -> Cmp2R {
        Cmp2R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cmp3(&self) -> Cmp3R {
        Cmp3R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cmp4(&self) -> Cmp4R {
        Cmp4R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn osync(&self) -> OsyncR {
        OsyncR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dilden(&self) -> DildenR {
        DildenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cilden(&self) -> CildenR {
        CildenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn zdlden(&self) -> ZdldenR {
        ZdldenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pdlden(&self) -> PdldenR {
        PdldenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn zcmlden(&self) -> ZcmldenR {
        ZcmldenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pcmlden(&self) -> PcmldenR {
        PcmldenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> Cmp1W<'_, Pwmcon2Spec> {
        Cmp1W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn cmp2(&mut self) -> Cmp2W<'_, Pwmcon2Spec> {
        Cmp2W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn cmp3(&mut self) -> Cmp3W<'_, Pwmcon2Spec> {
        Cmp3W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn cmp4(&mut self) -> Cmp4W<'_, Pwmcon2Spec> {
        Cmp4W::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn osync(&mut self) -> OsyncW<'_, Pwmcon2Spec> {
        OsyncW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dilden(&mut self) -> DildenW<'_, Pwmcon2Spec> {
        DildenW::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cilden(&mut self) -> CildenW<'_, Pwmcon2Spec> {
        CildenW::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn zdlden(&mut self) -> ZdldenW<'_, Pwmcon2Spec> {
        ZdldenW::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn pdlden(&mut self) -> PdldenW<'_, Pwmcon2Spec> {
        PdldenW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn zcmlden(&mut self) -> ZcmldenW<'_, Pwmcon2Spec> {
        ZcmldenW::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn pcmlden(&mut self) -> PcmldenW<'_, Pwmcon2Spec> {
        PcmldenW::new(self, 14)
    }
    #[doc = "Bits 15:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Pwmcon2Spec> {
        Rev0W::new(self, 15)
    }
}
#[doc = "PWMCON2\n\nYou can [`read`](crate::Reg::read) this register and get [`pwmcon2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwmcon2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pwmcon2Spec;
impl crate::RegisterSpec for Pwmcon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwmcon2::R`](R) reader structure"]
impl crate::Readable for Pwmcon2Spec {}
#[doc = "`write(|w| ..)` method takes [`pwmcon2::W`](W) writer structure"]
impl crate::Writable for Pwmcon2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWMCON2 to value 0"]
impl crate::Resettable for Pwmcon2Spec {}
