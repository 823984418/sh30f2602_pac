#[doc = "Register `SAFR` reader"]
pub type R = crate::R<SafrSpec>;
#[doc = "Register `SAFR` writer"]
pub type W = crate::W<SafrSpec>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::FieldReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWJCFG` reader - "]
pub type SwjcfgR = crate::BitReader;
#[doc = "Field `SWJCFG` writer - "]
pub type SwjcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IEN_EXTI0` reader - "]
pub type IenExti0R = crate::BitReader;
#[doc = "Field `IEN_EXTI0` writer - "]
pub type IenExti0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEN_BOD` reader - "]
pub type IenBodR = crate::BitReader;
#[doc = "Field `IEN_BOD` writer - "]
pub type IenBodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IEN_CSM` reader - "]
pub type IenCsmR = crate::BitReader;
#[doc = "Field `IEN_CSM` writer - "]
pub type IenCsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swjcfg(&self) -> SwjcfgR {
        SwjcfgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ien_exti0(&self) -> IenExti0R {
        IenExti0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ien_bod(&self) -> IenBodR {
        IenBodR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ien_csm(&self) -> IenCsmR {
        IenCsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAFR")
            .field("lock", &self.lock())
            .field("rev0", &self.rev0())
            .field("ien_csm", &self.ien_csm())
            .field("ien_bod", &self.ien_bod())
            .field("ien_exti0", &self.ien_exti0())
            .field("rev1", &self.rev1())
            .field("swjcfg", &self.swjcfg())
            .field("rev2", &self.rev2())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, SafrSpec> {
        Rev2W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn swjcfg(&mut self) -> SwjcfgW<'_, SafrSpec> {
        SwjcfgW::new(self, 2)
    }
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, SafrSpec> {
        Rev1W::new(self, 3)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ien_exti0(&mut self) -> IenExti0W<'_, SafrSpec> {
        IenExti0W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ien_bod(&mut self) -> IenBodW<'_, SafrSpec> {
        IenBodW::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ien_csm(&mut self) -> IenCsmW<'_, SafrSpec> {
        IenCsmW::new(self, 7)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, SafrSpec> {
        Rev0W::new(self, 8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, SafrSpec> {
        LockW::new(self, 16)
    }
}
#[doc = "SAFR\n\nYou can [`read`](crate::Reg::read) this register and get [`safr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`safr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SafrSpec;
impl crate::RegisterSpec for SafrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`safr::R`](R) reader structure"]
impl crate::Readable for SafrSpec {}
#[doc = "`write(|w| ..)` method takes [`safr::W`](W) writer structure"]
impl crate::Writable for SafrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAFR to value 0"]
impl crate::Resettable for SafrSpec {}
