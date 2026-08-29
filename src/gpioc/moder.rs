#[doc = "Register `MODER` reader"]
pub type R = crate::R<ModerSpec>;
#[doc = "Register `MODER` writer"]
pub type W = crate::W<ModerSpec>;
#[doc = "Field `MODER0` reader - "]
pub type Moder0R = crate::BitReader;
#[doc = "Field `MODER0` writer - "]
pub type Moder0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER1` reader - "]
pub type Moder1R = crate::BitReader;
#[doc = "Field `MODER1` writer - "]
pub type Moder1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER2` reader - "]
pub type Moder2R = crate::BitReader;
#[doc = "Field `MODER2` writer - "]
pub type Moder2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER3` reader - "]
pub type Moder3R = crate::BitReader;
#[doc = "Field `MODER3` writer - "]
pub type Moder3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER4` reader - "]
pub type Moder4R = crate::BitReader;
#[doc = "Field `MODER4` writer - "]
pub type Moder4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER5` reader - "]
pub type Moder5R = crate::BitReader;
#[doc = "Field `MODER5` writer - "]
pub type Moder5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER6` reader - "]
pub type Moder6R = crate::BitReader;
#[doc = "Field `MODER6` writer - "]
pub type Moder6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER7` reader - "]
pub type Moder7R = crate::BitReader;
#[doc = "Field `MODER7` writer - "]
pub type Moder7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER8` reader - "]
pub type Moder8R = crate::BitReader;
#[doc = "Field `MODER8` writer - "]
pub type Moder8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER9` reader - "]
pub type Moder9R = crate::BitReader;
#[doc = "Field `MODER9` writer - "]
pub type Moder9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER10` reader - "]
pub type Moder10R = crate::BitReader;
#[doc = "Field `MODER10` writer - "]
pub type Moder10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER11` reader - "]
pub type Moder11R = crate::BitReader;
#[doc = "Field `MODER11` writer - "]
pub type Moder11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER12` reader - "]
pub type Moder12R = crate::BitReader;
#[doc = "Field `MODER12` writer - "]
pub type Moder12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER13` reader - "]
pub type Moder13R = crate::BitReader;
#[doc = "Field `MODER13` writer - "]
pub type Moder13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER14` reader - "]
pub type Moder14R = crate::BitReader;
#[doc = "Field `MODER14` writer - "]
pub type Moder14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODER15` reader - "]
pub type Moder15R = crate::BitReader;
#[doc = "Field `MODER15` writer - "]
pub type Moder15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn moder0(&self) -> Moder0R {
        Moder0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn moder1(&self) -> Moder1R {
        Moder1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn moder2(&self) -> Moder2R {
        Moder2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn moder3(&self) -> Moder3R {
        Moder3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn moder4(&self) -> Moder4R {
        Moder4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn moder5(&self) -> Moder5R {
        Moder5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn moder6(&self) -> Moder6R {
        Moder6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn moder7(&self) -> Moder7R {
        Moder7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn moder8(&self) -> Moder8R {
        Moder8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn moder9(&self) -> Moder9R {
        Moder9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn moder10(&self) -> Moder10R {
        Moder10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn moder11(&self) -> Moder11R {
        Moder11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn moder12(&self) -> Moder12R {
        Moder12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn moder13(&self) -> Moder13R {
        Moder13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn moder14(&self) -> Moder14R {
        Moder14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn moder15(&self) -> Moder15R {
        Moder15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODER")
            .field("rev0", &self.rev0())
            .field("moder15", &self.moder15())
            .field("moder14", &self.moder14())
            .field("moder13", &self.moder13())
            .field("moder12", &self.moder12())
            .field("moder11", &self.moder11())
            .field("moder10", &self.moder10())
            .field("moder9", &self.moder9())
            .field("moder8", &self.moder8())
            .field("moder7", &self.moder7())
            .field("moder6", &self.moder6())
            .field("moder5", &self.moder5())
            .field("moder4", &self.moder4())
            .field("moder3", &self.moder3())
            .field("moder2", &self.moder2())
            .field("moder1", &self.moder1())
            .field("moder0", &self.moder0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn moder0(&mut self) -> Moder0W<'_, ModerSpec> {
        Moder0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn moder1(&mut self) -> Moder1W<'_, ModerSpec> {
        Moder1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn moder2(&mut self) -> Moder2W<'_, ModerSpec> {
        Moder2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn moder3(&mut self) -> Moder3W<'_, ModerSpec> {
        Moder3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn moder4(&mut self) -> Moder4W<'_, ModerSpec> {
        Moder4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn moder5(&mut self) -> Moder5W<'_, ModerSpec> {
        Moder5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn moder6(&mut self) -> Moder6W<'_, ModerSpec> {
        Moder6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn moder7(&mut self) -> Moder7W<'_, ModerSpec> {
        Moder7W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn moder8(&mut self) -> Moder8W<'_, ModerSpec> {
        Moder8W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn moder9(&mut self) -> Moder9W<'_, ModerSpec> {
        Moder9W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn moder10(&mut self) -> Moder10W<'_, ModerSpec> {
        Moder10W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn moder11(&mut self) -> Moder11W<'_, ModerSpec> {
        Moder11W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn moder12(&mut self) -> Moder12W<'_, ModerSpec> {
        Moder12W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn moder13(&mut self) -> Moder13W<'_, ModerSpec> {
        Moder13W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn moder14(&mut self) -> Moder14W<'_, ModerSpec> {
        Moder14W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn moder15(&mut self) -> Moder15W<'_, ModerSpec> {
        Moder15W::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, ModerSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "MODER\n\nYou can [`read`](crate::Reg::read) this register and get [`moder::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moder::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModerSpec;
impl crate::RegisterSpec for ModerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`moder::R`](R) reader structure"]
impl crate::Readable for ModerSpec {}
#[doc = "`write(|w| ..)` method takes [`moder::W`](W) writer structure"]
impl crate::Writable for ModerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODER to value 0"]
impl crate::Resettable for ModerSpec {}
