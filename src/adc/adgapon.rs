#[doc = "Register `ADGAPON` reader"]
pub type R = crate::R<AdgaponSpec>;
#[doc = "Register `ADGAPON` writer"]
pub type W = crate::W<AdgaponSpec>;
#[doc = "Field `GAPEN0` reader - "]
pub type Gapen0R = crate::BitReader;
#[doc = "Field `GAPEN0` writer - "]
pub type Gapen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN1` reader - "]
pub type Gapen1R = crate::BitReader;
#[doc = "Field `GAPEN1` writer - "]
pub type Gapen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN2` reader - "]
pub type Gapen2R = crate::BitReader;
#[doc = "Field `GAPEN2` writer - "]
pub type Gapen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN3` reader - "]
pub type Gapen3R = crate::BitReader;
#[doc = "Field `GAPEN3` writer - "]
pub type Gapen3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN4` reader - "]
pub type Gapen4R = crate::BitReader;
#[doc = "Field `GAPEN4` writer - "]
pub type Gapen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN5` reader - "]
pub type Gapen5R = crate::BitReader;
#[doc = "Field `GAPEN5` writer - "]
pub type Gapen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN6` reader - "]
pub type Gapen6R = crate::BitReader;
#[doc = "Field `GAPEN6` writer - "]
pub type Gapen6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN7` reader - "]
pub type Gapen7R = crate::BitReader;
#[doc = "Field `GAPEN7` writer - "]
pub type Gapen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN8` reader - "]
pub type Gapen8R = crate::BitReader;
#[doc = "Field `GAPEN8` writer - "]
pub type Gapen8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN9` reader - "]
pub type Gapen9R = crate::BitReader;
#[doc = "Field `GAPEN9` writer - "]
pub type Gapen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN10` reader - "]
pub type Gapen10R = crate::BitReader;
#[doc = "Field `GAPEN10` writer - "]
pub type Gapen10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN11` reader - "]
pub type Gapen11R = crate::BitReader;
#[doc = "Field `GAPEN11` writer - "]
pub type Gapen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN12` reader - "]
pub type Gapen12R = crate::BitReader;
#[doc = "Field `GAPEN12` writer - "]
pub type Gapen12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN13` reader - "]
pub type Gapen13R = crate::BitReader;
#[doc = "Field `GAPEN13` writer - "]
pub type Gapen13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN14` reader - "]
pub type Gapen14R = crate::BitReader;
#[doc = "Field `GAPEN14` writer - "]
pub type Gapen14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GAPEN15` reader - "]
pub type Gapen15R = crate::BitReader;
#[doc = "Field `GAPEN15` writer - "]
pub type Gapen15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gapen0(&self) -> Gapen0R {
        Gapen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gapen1(&self) -> Gapen1R {
        Gapen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gapen2(&self) -> Gapen2R {
        Gapen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gapen3(&self) -> Gapen3R {
        Gapen3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gapen4(&self) -> Gapen4R {
        Gapen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gapen5(&self) -> Gapen5R {
        Gapen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gapen6(&self) -> Gapen6R {
        Gapen6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gapen7(&self) -> Gapen7R {
        Gapen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gapen8(&self) -> Gapen8R {
        Gapen8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gapen9(&self) -> Gapen9R {
        Gapen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gapen10(&self) -> Gapen10R {
        Gapen10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gapen11(&self) -> Gapen11R {
        Gapen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gapen12(&self) -> Gapen12R {
        Gapen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gapen13(&self) -> Gapen13R {
        Gapen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gapen14(&self) -> Gapen14R {
        Gapen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gapen15(&self) -> Gapen15R {
        Gapen15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADGAPON")
            .field("rev0", &self.rev0())
            .field("gapen15", &self.gapen15())
            .field("gapen14", &self.gapen14())
            .field("gapen13", &self.gapen13())
            .field("gapen12", &self.gapen12())
            .field("gapen11", &self.gapen11())
            .field("gapen10", &self.gapen10())
            .field("gapen9", &self.gapen9())
            .field("gapen8", &self.gapen8())
            .field("gapen7", &self.gapen7())
            .field("gapen6", &self.gapen6())
            .field("gapen5", &self.gapen5())
            .field("gapen4", &self.gapen4())
            .field("gapen3", &self.gapen3())
            .field("gapen2", &self.gapen2())
            .field("gapen1", &self.gapen1())
            .field("gapen0", &self.gapen0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn gapen0(&mut self) -> Gapen0W<'_, AdgaponSpec> {
        Gapen0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn gapen1(&mut self) -> Gapen1W<'_, AdgaponSpec> {
        Gapen1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn gapen2(&mut self) -> Gapen2W<'_, AdgaponSpec> {
        Gapen2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn gapen3(&mut self) -> Gapen3W<'_, AdgaponSpec> {
        Gapen3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn gapen4(&mut self) -> Gapen4W<'_, AdgaponSpec> {
        Gapen4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn gapen5(&mut self) -> Gapen5W<'_, AdgaponSpec> {
        Gapen5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn gapen6(&mut self) -> Gapen6W<'_, AdgaponSpec> {
        Gapen6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn gapen7(&mut self) -> Gapen7W<'_, AdgaponSpec> {
        Gapen7W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn gapen8(&mut self) -> Gapen8W<'_, AdgaponSpec> {
        Gapen8W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn gapen9(&mut self) -> Gapen9W<'_, AdgaponSpec> {
        Gapen9W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn gapen10(&mut self) -> Gapen10W<'_, AdgaponSpec> {
        Gapen10W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn gapen11(&mut self) -> Gapen11W<'_, AdgaponSpec> {
        Gapen11W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn gapen12(&mut self) -> Gapen12W<'_, AdgaponSpec> {
        Gapen12W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn gapen13(&mut self) -> Gapen13W<'_, AdgaponSpec> {
        Gapen13W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn gapen14(&mut self) -> Gapen14W<'_, AdgaponSpec> {
        Gapen14W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn gapen15(&mut self) -> Gapen15W<'_, AdgaponSpec> {
        Gapen15W::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AdgaponSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "ADGAPON\n\nYou can [`read`](crate::Reg::read) this register and get [`adgapon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgapon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdgaponSpec;
impl crate::RegisterSpec for AdgaponSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adgapon::R`](R) reader structure"]
impl crate::Readable for AdgaponSpec {}
#[doc = "`write(|w| ..)` method takes [`adgapon::W`](W) writer structure"]
impl crate::Writable for AdgaponSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADGAPON to value 0"]
impl crate::Resettable for AdgaponSpec {}
