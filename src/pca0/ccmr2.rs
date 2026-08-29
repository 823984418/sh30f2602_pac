#[doc = "Register `CCMR2` reader"]
pub type R = crate::R<Ccmr2Spec>;
#[doc = "Register `CCMR2` writer"]
pub type W = crate::W<Ccmr2Spec>;
#[doc = "Field `CCIE` reader - "]
pub type CcieR = crate::BitReader;
#[doc = "Field `CCIE` writer - "]
pub type CcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCP` reader - "]
pub type TcpR = crate::BitReader;
#[doc = "Field `TCP` writer - "]
pub type TcpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEN` reader - "]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - "]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - "]
pub type FsR = crate::FieldReader;
#[doc = "Field `FS` writer - "]
pub type FsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SM` reader - "]
pub type SmR = crate::FieldReader;
#[doc = "Field `SM` writer - "]
pub type SmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CC` reader - "]
pub type CcR = crate::BitReader;
#[doc = "Field `CC` writer - "]
pub type CcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICF` reader - "]
pub type IcfR = crate::FieldReader;
#[doc = "Field `ICF` writer - "]
pub type IcfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CSSEL` reader - "]
pub type CsselR = crate::BitReader;
#[doc = "Field `CSSEL` writer - "]
pub type CsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccie(&self) -> CcieR {
        CcieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcp(&self) -> TcpR {
        TcpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cc(&self) -> CcR {
        CcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn icf(&self) -> IcfR {
        IcfR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cssel(&self) -> CsselR {
        CsselR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCMR2")
            .field("rev0", &self.rev0())
            .field("cssel", &self.cssel())
            .field("icf", &self.icf())
            .field("cc", &self.cc())
            .field("sm", &self.sm())
            .field("fs", &self.fs())
            .field("cen", &self.cen())
            .field("tcp", &self.tcp())
            .field("rev1", &self.rev1())
            .field("ccie", &self.ccie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ccie(&mut self) -> CcieW<'_, Ccmr2Spec> {
        CcieW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, Ccmr2Spec> {
        Rev1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TcpW<'_, Ccmr2Spec> {
        TcpW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, Ccmr2Spec> {
        CenW::new(self, 3)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, Ccmr2Spec> {
        FsW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn sm(&mut self) -> SmW<'_, Ccmr2Spec> {
        SmW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cc(&mut self) -> CcW<'_, Ccmr2Spec> {
        CcW::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn icf(&mut self) -> IcfW<'_, Ccmr2Spec> {
        IcfW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cssel(&mut self) -> CsselW<'_, Ccmr2Spec> {
        CsselW::new(self, 12)
    }
    #[doc = "Bits 13:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Ccmr2Spec> {
        Rev0W::new(self, 13)
    }
}
#[doc = "CCMR2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccmr2Spec;
impl crate::RegisterSpec for Ccmr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr2::R`](R) reader structure"]
impl crate::Readable for Ccmr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccmr2::W`](W) writer structure"]
impl crate::Writable for Ccmr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCMR2 to value 0"]
impl crate::Resettable for Ccmr2Spec {}
