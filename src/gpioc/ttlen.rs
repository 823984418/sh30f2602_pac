#[doc = "Register `TTLEN` reader"]
pub type R = crate::R<TtlenSpec>;
#[doc = "Register `TTLEN` writer"]
pub type W = crate::W<TtlenSpec>;
#[doc = "Field `TTLEN0` reader - "]
pub type Ttlen0R = crate::BitReader;
#[doc = "Field `TTLEN0` writer - "]
pub type Ttlen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN1` reader - "]
pub type Ttlen1R = crate::BitReader;
#[doc = "Field `TTLEN1` writer - "]
pub type Ttlen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN2` reader - "]
pub type Ttlen2R = crate::BitReader;
#[doc = "Field `TTLEN2` writer - "]
pub type Ttlen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev4` reader - "]
pub type Rev4R = crate::BitReader;
#[doc = "Field `rev4` writer - "]
pub type Rev4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN4` reader - "]
pub type Ttlen4R = crate::BitReader;
#[doc = "Field `TTLEN4` writer - "]
pub type Ttlen4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN5` reader - "]
pub type Ttlen5R = crate::BitReader;
#[doc = "Field `TTLEN5` writer - "]
pub type Ttlen5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev3` reader - "]
pub type Rev3R = crate::BitReader;
#[doc = "Field `rev3` writer - "]
pub type Rev3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN7` reader - "]
pub type Ttlen7R = crate::BitReader;
#[doc = "Field `TTLEN7` writer - "]
pub type Ttlen7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev2` reader - "]
pub type Rev2R = crate::BitReader;
#[doc = "Field `rev2` writer - "]
pub type Rev2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN9` reader - "]
pub type Ttlen9R = crate::BitReader;
#[doc = "Field `TTLEN9` writer - "]
pub type Ttlen9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::BitReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN11` reader - "]
pub type Ttlen11R = crate::BitReader;
#[doc = "Field `TTLEN11` writer - "]
pub type Ttlen11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN12` reader - "]
pub type Ttlen12R = crate::BitReader;
#[doc = "Field `TTLEN12` writer - "]
pub type Ttlen12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN13` reader - "]
pub type Ttlen13R = crate::BitReader;
#[doc = "Field `TTLEN13` writer - "]
pub type Ttlen13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN14` reader - "]
pub type Ttlen14R = crate::BitReader;
#[doc = "Field `TTLEN14` writer - "]
pub type Ttlen14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTLEN15` reader - "]
pub type Ttlen15R = crate::BitReader;
#[doc = "Field `TTLEN15` writer - "]
pub type Ttlen15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ttlen0(&self) -> Ttlen0R {
        Ttlen0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ttlen1(&self) -> Ttlen1R {
        Ttlen1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ttlen2(&self) -> Ttlen2R {
        Ttlen2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev4(&self) -> Rev4R {
        Rev4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ttlen4(&self) -> Ttlen4R {
        Ttlen4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ttlen5(&self) -> Ttlen5R {
        Ttlen5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rev3(&self) -> Rev3R {
        Rev3R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ttlen7(&self) -> Ttlen7R {
        Ttlen7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rev2(&self) -> Rev2R {
        Rev2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ttlen9(&self) -> Ttlen9R {
        Ttlen9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ttlen11(&self) -> Ttlen11R {
        Ttlen11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ttlen12(&self) -> Ttlen12R {
        Ttlen12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ttlen13(&self) -> Ttlen13R {
        Ttlen13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ttlen14(&self) -> Ttlen14R {
        Ttlen14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ttlen15(&self) -> Ttlen15R {
        Ttlen15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TTLEN")
            .field("rev0", &self.rev0())
            .field("ttlen15", &self.ttlen15())
            .field("ttlen14", &self.ttlen14())
            .field("ttlen13", &self.ttlen13())
            .field("ttlen12", &self.ttlen12())
            .field("ttlen11", &self.ttlen11())
            .field("rev1", &self.rev1())
            .field("ttlen9", &self.ttlen9())
            .field("rev2", &self.rev2())
            .field("ttlen7", &self.ttlen7())
            .field("rev3", &self.rev3())
            .field("ttlen5", &self.ttlen5())
            .field("ttlen4", &self.ttlen4())
            .field("rev4", &self.rev4())
            .field("ttlen2", &self.ttlen2())
            .field("ttlen1", &self.ttlen1())
            .field("ttlen0", &self.ttlen0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn ttlen0(&mut self) -> Ttlen0W<'_, TtlenSpec> {
        Ttlen0W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn ttlen1(&mut self) -> Ttlen1W<'_, TtlenSpec> {
        Ttlen1W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn ttlen2(&mut self) -> Ttlen2W<'_, TtlenSpec> {
        Ttlen2W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rev4(&mut self) -> Rev4W<'_, TtlenSpec> {
        Rev4W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn ttlen4(&mut self) -> Ttlen4W<'_, TtlenSpec> {
        Ttlen4W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn ttlen5(&mut self) -> Ttlen5W<'_, TtlenSpec> {
        Ttlen5W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn rev3(&mut self) -> Rev3W<'_, TtlenSpec> {
        Rev3W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn ttlen7(&mut self) -> Ttlen7W<'_, TtlenSpec> {
        Ttlen7W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn rev2(&mut self) -> Rev2W<'_, TtlenSpec> {
        Rev2W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ttlen9(&mut self) -> Ttlen9W<'_, TtlenSpec> {
        Ttlen9W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, TtlenSpec> {
        Rev1W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn ttlen11(&mut self) -> Ttlen11W<'_, TtlenSpec> {
        Ttlen11W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn ttlen12(&mut self) -> Ttlen12W<'_, TtlenSpec> {
        Ttlen12W::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn ttlen13(&mut self) -> Ttlen13W<'_, TtlenSpec> {
        Ttlen13W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn ttlen14(&mut self) -> Ttlen14W<'_, TtlenSpec> {
        Ttlen14W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn ttlen15(&mut self) -> Ttlen15W<'_, TtlenSpec> {
        Ttlen15W::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, TtlenSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "TTLEN\n\nYou can [`read`](crate::Reg::read) this register and get [`ttlen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ttlen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TtlenSpec;
impl crate::RegisterSpec for TtlenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ttlen::R`](R) reader structure"]
impl crate::Readable for TtlenSpec {}
#[doc = "`write(|w| ..)` method takes [`ttlen::W`](W) writer structure"]
impl crate::Writable for TtlenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TTLEN to value 0"]
impl crate::Resettable for TtlenSpec {}
