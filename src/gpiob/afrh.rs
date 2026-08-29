#[doc = "Register `AFRH` reader"]
pub type R = crate::R<AfrhSpec>;
#[doc = "Register `AFRH` writer"]
pub type W = crate::W<AfrhSpec>;
#[doc = "Field `AFR8` reader - "]
pub type Afr8R = crate::FieldReader;
#[doc = "Field `AFR8` writer - "]
pub type Afr8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev7` reader - "]
pub type Rev7R = crate::BitReader;
#[doc = "Field `rev7` writer - "]
pub type Rev7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR9` reader - "]
pub type Afr9R = crate::FieldReader;
#[doc = "Field `AFR9` writer - "]
pub type Afr9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev6` reader - "]
pub type Rev6R = crate::BitReader;
#[doc = "Field `rev6` writer - "]
pub type Rev6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR10` reader - "]
pub type Afr10R = crate::FieldReader;
#[doc = "Field `AFR10` writer - "]
pub type Afr10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev5` reader - "]
pub type Rev5R = crate::BitReader;
#[doc = "Field `rev5` writer - "]
pub type Rev5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR11` reader - "]
pub type Afr11R = crate::FieldReader;
#[doc = "Field `AFR11` writer - "]
pub type Afr11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev4` reader - "]
pub type Rev4R = crate::BitReader;
#[doc = "Field `rev4` writer - "]
pub type Rev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR12` reader - "]
pub type Afr12R = crate::FieldReader;
#[doc = "Field `AFR12` writer - "]
pub type Afr12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR13` reader - "]
pub type Afr13R = crate::FieldReader;
#[doc = "Field `AFR13` writer - "]
pub type Afr13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR14` reader - "]
pub type Afr14R = crate::FieldReader;
#[doc = "Field `AFR14` writer - "]
pub type Afr14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR15` reader - "]
pub type Afr15R = crate::FieldReader;
#[doc = "Field `AFR15` writer - "]
pub type Afr15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn afr8(&self) -> Afr8R {
        Afr8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev7(&self) -> Rev7R {
        Rev7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn afr9(&self) -> Afr9R {
        Afr9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rev6(&self) -> Rev6R {
        Rev6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn afr10(&self) -> Afr10R {
        Afr10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev5(&self) -> Rev5R {
        Rev5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn afr11(&self) -> Afr11R {
        Afr11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn afr12(&self) -> Afr12R {
        Afr12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn afr13(&self) -> Afr13R {
        Afr13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn afr14(&self) -> Afr14R {
        Afr14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn afr15(&self) -> Afr15R {
        Afr15R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AFRH")
            .field("rev0", &self.rev0())
            .field("afr15", &self.afr15())
            .field("rev1", &self.rev1())
            .field("afr14", &self.afr14())
            .field("rev2", &self.rev2())
            .field("afr13", &self.afr13())
            .field("rev3", &self.rev3())
            .field("afr12", &self.afr12())
            .field("rev4", &self.rev4())
            .field("afr11", &self.afr11())
            .field("rev5", &self.rev5())
            .field("afr10", &self.afr10())
            .field("rev6", &self.rev6())
            .field("afr9", &self.afr9())
            .field("rev7", &self.rev7())
            .field("afr8", &self.afr8())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn afr8(&mut self) -> Afr8W<'_, AfrhSpec> {
        Afr8W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev7(&mut self) -> Rev7W<'_, AfrhSpec> {
        Rev7W::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn afr9(&mut self) -> Afr9W<'_, AfrhSpec> {
        Afr9W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rev6(&mut self) -> Rev6W<'_, AfrhSpec> {
        Rev6W::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn afr10(&mut self) -> Afr10W<'_, AfrhSpec> {
        Afr10W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev5(&mut self) -> Rev5W<'_, AfrhSpec> {
        Rev5W::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn afr11(&mut self) -> Afr11W<'_, AfrhSpec> {
        Afr11W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev4(&mut self) -> Rev4W<'_, AfrhSpec> {
        Rev4W::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn afr12(&mut self) -> Afr12W<'_, AfrhSpec> {
        Afr12W::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, AfrhSpec> {
        Rev3W::new(self, 19)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn afr13(&mut self) -> Afr13W<'_, AfrhSpec> {
        Afr13W::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, AfrhSpec> {
        Rev2W::new(self, 23)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn afr14(&mut self) -> Afr14W<'_, AfrhSpec> {
        Afr14W::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, AfrhSpec> {
        Rev1W::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn afr15(&mut self) -> Afr15W<'_, AfrhSpec> {
        Afr15W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AfrhSpec> {
        Rev0W::new(self, 31)
    }
}
#[doc = "AFRH\n\nYou can [`read`](crate::Reg::read) this register and get [`afrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrhSpec;
impl crate::RegisterSpec for AfrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrh::R`](R) reader structure"]
impl crate::Readable for AfrhSpec {}
#[doc = "`write(|w| ..)` method takes [`afrh::W`](W) writer structure"]
impl crate::Writable for AfrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRH to value 0"]
impl crate::Resettable for AfrhSpec {}
