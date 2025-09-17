#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "Field `IE0` reader - "]
pub type Ie0R = crate::BitReader;
#[doc = "Field `IE0` writer - "]
pub type Ie0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE1` reader - "]
pub type Ie1R = crate::BitReader;
#[doc = "Field `IE1` writer - "]
pub type Ie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE2` reader - "]
pub type Ie2R = crate::BitReader;
#[doc = "Field `IE2` writer - "]
pub type Ie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE3` reader - "]
pub type Ie3R = crate::BitReader;
#[doc = "Field `IE3` writer - "]
pub type Ie3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE4` reader - "]
pub type Ie4R = crate::BitReader;
#[doc = "Field `IE4` writer - "]
pub type Ie4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE5` reader - "]
pub type Ie5R = crate::BitReader;
#[doc = "Field `IE5` writer - "]
pub type Ie5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE6` reader - "]
pub type Ie6R = crate::BitReader;
#[doc = "Field `IE6` writer - "]
pub type Ie6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE7` reader - "]
pub type Ie7R = crate::BitReader;
#[doc = "Field `IE7` writer - "]
pub type Ie7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE8` reader - "]
pub type Ie8R = crate::BitReader;
#[doc = "Field `IE8` writer - "]
pub type Ie8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE9` reader - "]
pub type Ie9R = crate::BitReader;
#[doc = "Field `IE9` writer - "]
pub type Ie9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE10` reader - "]
pub type Ie10R = crate::BitReader;
#[doc = "Field `IE10` writer - "]
pub type Ie10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE11` reader - "]
pub type Ie11R = crate::BitReader;
#[doc = "Field `IE11` writer - "]
pub type Ie11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE12` reader - "]
pub type Ie12R = crate::BitReader;
#[doc = "Field `IE12` writer - "]
pub type Ie12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE13` reader - "]
pub type Ie13R = crate::BitReader;
#[doc = "Field `IE13` writer - "]
pub type Ie13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE14` reader - "]
pub type Ie14R = crate::BitReader;
#[doc = "Field `IE14` writer - "]
pub type Ie14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IE15` reader - "]
pub type Ie15R = crate::BitReader;
#[doc = "Field `IE15` writer - "]
pub type Ie15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ie0(&self) -> Ie0R {
        Ie0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ie1(&self) -> Ie1R {
        Ie1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ie2(&self) -> Ie2R {
        Ie2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ie3(&self) -> Ie3R {
        Ie3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ie4(&self) -> Ie4R {
        Ie4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ie5(&self) -> Ie5R {
        Ie5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ie6(&self) -> Ie6R {
        Ie6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ie7(&self) -> Ie7R {
        Ie7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ie8(&self) -> Ie8R {
        Ie8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ie9(&self) -> Ie9R {
        Ie9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ie10(&self) -> Ie10R {
        Ie10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ie11(&self) -> Ie11R {
        Ie11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ie12(&self) -> Ie12R {
        Ie12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ie13(&self) -> Ie13R {
        Ie13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie14(&self) -> Ie14R {
        Ie14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ie15(&self) -> Ie15R {
        Ie15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ie0(&mut self) -> Ie0W<'_, IeSpec> {
        Ie0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ie1(&mut self) -> Ie1W<'_, IeSpec> {
        Ie1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ie2(&mut self) -> Ie2W<'_, IeSpec> {
        Ie2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn ie3(&mut self) -> Ie3W<'_, IeSpec> {
        Ie3W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ie4(&mut self) -> Ie4W<'_, IeSpec> {
        Ie4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ie5(&mut self) -> Ie5W<'_, IeSpec> {
        Ie5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn ie6(&mut self) -> Ie6W<'_, IeSpec> {
        Ie6W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ie7(&mut self) -> Ie7W<'_, IeSpec> {
        Ie7W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ie8(&mut self) -> Ie8W<'_, IeSpec> {
        Ie8W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ie9(&mut self) -> Ie9W<'_, IeSpec> {
        Ie9W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ie10(&mut self) -> Ie10W<'_, IeSpec> {
        Ie10W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ie11(&mut self) -> Ie11W<'_, IeSpec> {
        Ie11W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ie12(&mut self) -> Ie12W<'_, IeSpec> {
        Ie12W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ie13(&mut self) -> Ie13W<'_, IeSpec> {
        Ie13W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ie14(&mut self) -> Ie14W<'_, IeSpec> {
        Ie14W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ie15(&mut self) -> Ie15W<'_, IeSpec> {
        Ie15W::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, IeSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "IE\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IE to value 0xffff"]
impl crate::Resettable for IeSpec {
    const RESET_VALUE: u32 = 0xffff;
}
