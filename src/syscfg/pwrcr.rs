#[doc = "Register `PWRCR` reader"]
pub type R = crate::R<PwrcrSpec>;
#[doc = "Register `PWRCR` writer"]
pub type W = crate::W<PwrcrSpec>;
#[doc = "Field `VBOD` reader - "]
pub type VbodR = crate::FieldReader;
#[doc = "Field `VBOD` writer - "]
pub type VbodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BODMD` reader - "]
pub type BodmdR = crate::FieldReader;
#[doc = "Field `BODMD` writer - "]
pub type BodmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BODIE` reader - "]
pub type BodieR = crate::BitReader;
#[doc = "Field `BODIE` writer - "]
pub type BodieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODEN` reader - "]
pub type BodenR = crate::BitReader;
#[doc = "Field `BODEN` writer - "]
pub type BodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LVRLV` reader - "]
pub type LvrlvR = crate::FieldReader;
#[doc = "Field `LVRLV` writer - "]
pub type LvrlvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LVREN` reader - "]
pub type LvrenR = crate::BitReader;
#[doc = "Field `LVREN` writer - "]
pub type LvrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn vbod(&self) -> VbodR {
        VbodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bodmd(&self) -> BodmdR {
        BodmdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bodie(&self) -> BodieR {
        BodieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn boden(&self) -> BodenR {
        BodenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lvrlv(&self) -> LvrlvR {
        LvrlvR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lvren(&self) -> LvrenR {
        LvrenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWRCR")
            .field("rev0", &self.rev0())
            .field("lvren", &self.lvren())
            .field("lvrlv", &self.lvrlv())
            .field("boden", &self.boden())
            .field("bodie", &self.bodie())
            .field("bodmd", &self.bodmd())
            .field("vbod", &self.vbod())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn vbod(&mut self) -> VbodW<'_, PwrcrSpec> {
        VbodW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn bodmd(&mut self) -> BodmdW<'_, PwrcrSpec> {
        BodmdW::new(self, 4)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn bodie(&mut self) -> BodieW<'_, PwrcrSpec> {
        BodieW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn boden(&mut self) -> BodenW<'_, PwrcrSpec> {
        BodenW::new(self, 7)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn lvrlv(&mut self) -> LvrlvW<'_, PwrcrSpec> {
        LvrlvW::new(self, 8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn lvren(&mut self) -> LvrenW<'_, PwrcrSpec> {
        LvrenW::new(self, 10)
    }
    #[doc = "Bits 11:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, PwrcrSpec> {
        Rev0W::new(self, 11)
    }
}
#[doc = "PWRCR\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcrSpec;
impl crate::RegisterSpec for PwrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcr::R`](R) reader structure"]
impl crate::Readable for PwrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure"]
impl crate::Writable for PwrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PwrcrSpec {}
