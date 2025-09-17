#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AfrlSpec>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AfrlSpec>;
#[doc = "Field `AFR0` reader - "]
pub type Afr0R = crate::FieldReader;
#[doc = "Field `AFR0` writer - "]
pub type Afr0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev7` reader - "]
pub type Rev7R = crate::BitReader;
#[doc = "Field `rev7` writer - "]
pub type Rev7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR1` reader - "]
pub type Afr1R = crate::FieldReader;
#[doc = "Field `AFR1` writer - "]
pub type Afr1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev6` reader - "]
pub type Rev6R = crate::BitReader;
#[doc = "Field `rev6` writer - "]
pub type Rev6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR2` reader - "]
pub type Afr2R = crate::FieldReader;
#[doc = "Field `AFR2` writer - "]
pub type Afr2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev5` reader - "]
pub type Rev5R = crate::BitReader;
#[doc = "Field `rev5` writer - "]
pub type Rev5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR3` reader - "]
pub type Afr3R = crate::FieldReader;
#[doc = "Field `AFR3` writer - "]
pub type Afr3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev4` reader - "]
pub type Rev4R = crate::BitReader;
#[doc = "Field `rev4` writer - "]
pub type Rev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR4` reader - "]
pub type Afr4R = crate::FieldReader;
#[doc = "Field `AFR4` writer - "]
pub type Afr4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR5` reader - "]
pub type Afr5R = crate::FieldReader;
#[doc = "Field `AFR5` writer - "]
pub type Afr5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR6` reader - "]
pub type Afr6R = crate::FieldReader;
#[doc = "Field `AFR6` writer - "]
pub type Afr6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFR7` reader - "]
pub type Afr7R = crate::FieldReader;
#[doc = "Field `AFR7` writer - "]
pub type Afr7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn afr0(&self) -> Afr0R {
        Afr0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev7(&self) -> Rev7R {
        Rev7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn afr1(&self) -> Afr1R {
        Afr1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rev6(&self) -> Rev6R {
        Rev6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn afr2(&self) -> Afr2R {
        Afr2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev5(&self) -> Rev5R {
        Rev5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn afr3(&self) -> Afr3R {
        Afr3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn afr4(&self) -> Afr4R {
        Afr4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn afr5(&self) -> Afr5R {
        Afr5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn afr6(&self) -> Afr6R {
        Afr6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn afr7(&self) -> Afr7R {
        Afr7R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn afr0(&mut self) -> Afr0W<'_, AfrlSpec> {
        Afr0W::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev7(&mut self) -> Rev7W<'_, AfrlSpec> {
        Rev7W::new(self, 3)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn afr1(&mut self) -> Afr1W<'_, AfrlSpec> {
        Afr1W::new(self, 4)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn rev6(&mut self) -> Rev6W<'_, AfrlSpec> {
        Rev6W::new(self, 7)
    }
    #[doc = "Bits 8:10"]
    #[inline(always)]
    pub fn afr2(&mut self) -> Afr2W<'_, AfrlSpec> {
        Afr2W::new(self, 8)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn rev5(&mut self) -> Rev5W<'_, AfrlSpec> {
        Rev5W::new(self, 11)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn afr3(&mut self) -> Afr3W<'_, AfrlSpec> {
        Afr3W::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn rev4(&mut self) -> Rev4W<'_, AfrlSpec> {
        Rev4W::new(self, 15)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn afr4(&mut self) -> Afr4W<'_, AfrlSpec> {
        Afr4W::new(self, 16)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, AfrlSpec> {
        Rev3W::new(self, 19)
    }
    #[doc = "Bits 20:22"]
    #[inline(always)]
    pub fn afr5(&mut self) -> Afr5W<'_, AfrlSpec> {
        Afr5W::new(self, 20)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, AfrlSpec> {
        Rev2W::new(self, 23)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn afr6(&mut self) -> Afr6W<'_, AfrlSpec> {
        Afr6W::new(self, 24)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, AfrlSpec> {
        Rev1W::new(self, 27)
    }
    #[doc = "Bits 28:30"]
    #[inline(always)]
    pub fn afr7(&mut self) -> Afr7W<'_, AfrlSpec> {
        Afr7W::new(self, 28)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AfrlSpec> {
        Rev0W::new(self, 31)
    }
}
#[doc = "AFRL\n\nYou can [`read`](crate::Reg::read) this register and get [`afrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AfrlSpec;
impl crate::RegisterSpec for AfrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AfrlSpec {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AfrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AfrlSpec {}
