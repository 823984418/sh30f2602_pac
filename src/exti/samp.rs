#[doc = "Register `SAMP` reader"]
pub type R = crate::R<SampSpec>;
#[doc = "Register `SAMP` writer"]
pub type W = crate::W<SampSpec>;
#[doc = "Field `SN0` reader - "]
pub type Sn0R = crate::FieldReader;
#[doc = "Field `SN0` writer - "]
pub type Sn0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS0` reader - "]
pub type Ps0R = crate::FieldReader;
#[doc = "Field `PS0` writer - "]
pub type Ps0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SN1` reader - "]
pub type Sn1R = crate::FieldReader;
#[doc = "Field `SN1` writer - "]
pub type Sn1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS1` reader - "]
pub type Ps1R = crate::FieldReader;
#[doc = "Field `PS1` writer - "]
pub type Ps1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SN2` reader - "]
pub type Sn2R = crate::FieldReader;
#[doc = "Field `SN2` writer - "]
pub type Sn2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS2` reader - "]
pub type Ps2R = crate::FieldReader;
#[doc = "Field `PS2` writer - "]
pub type Ps2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SN3` reader - "]
pub type Sn3R = crate::FieldReader;
#[doc = "Field `SN3` writer - "]
pub type Sn3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS3` reader - "]
pub type Ps3R = crate::FieldReader;
#[doc = "Field `PS3` writer - "]
pub type Ps3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SN4` reader - "]
pub type Sn4R = crate::FieldReader;
#[doc = "Field `SN4` writer - "]
pub type Sn4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS4` reader - "]
pub type Ps4R = crate::FieldReader;
#[doc = "Field `PS4` writer - "]
pub type Ps4W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SN5` reader - "]
pub type Sn5R = crate::FieldReader;
#[doc = "Field `SN5` writer - "]
pub type Sn5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS5` reader - "]
pub type Ps5R = crate::FieldReader;
#[doc = "Field `PS5` writer - "]
pub type Ps5W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SN6` reader - "]
pub type Sn6R = crate::FieldReader;
#[doc = "Field `SN6` writer - "]
pub type Sn6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS6` reader - "]
pub type Ps6R = crate::FieldReader;
#[doc = "Field `PS6` writer - "]
pub type Ps6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SN7` reader - "]
pub type Sn7R = crate::FieldReader;
#[doc = "Field `SN7` writer - "]
pub type Sn7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PS7` reader - "]
pub type Ps7R = crate::FieldReader;
#[doc = "Field `PS7` writer - "]
pub type Ps7W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sn0(&self) -> Sn0R {
        Sn0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ps0(&self) -> Ps0R {
        Ps0R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sn1(&self) -> Sn1R {
        Sn1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ps1(&self) -> Ps1R {
        Ps1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sn2(&self) -> Sn2R {
        Sn2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ps2(&self) -> Ps2R {
        Ps2R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sn3(&self) -> Sn3R {
        Sn3R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ps3(&self) -> Ps3R {
        Ps3R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sn4(&self) -> Sn4R {
        Sn4R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn ps4(&self) -> Ps4R {
        Ps4R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sn5(&self) -> Sn5R {
        Sn5R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn ps5(&self) -> Ps5R {
        Ps5R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn sn6(&self) -> Sn6R {
        Sn6R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ps6(&self) -> Ps6R {
        Ps6R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sn7(&self) -> Sn7R {
        Sn7R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn ps7(&self) -> Ps7R {
        Ps7R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAMP")
            .field("ps7", &self.ps7())
            .field("sn7", &self.sn7())
            .field("ps6", &self.ps6())
            .field("sn6", &self.sn6())
            .field("ps5", &self.ps5())
            .field("sn5", &self.sn5())
            .field("ps4", &self.ps4())
            .field("sn4", &self.sn4())
            .field("ps3", &self.ps3())
            .field("sn3", &self.sn3())
            .field("ps2", &self.ps2())
            .field("sn2", &self.sn2())
            .field("ps1", &self.ps1())
            .field("sn1", &self.sn1())
            .field("ps0", &self.ps0())
            .field("sn0", &self.sn0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn sn0(&mut self) -> Sn0W<'_, SampSpec> {
        Sn0W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn ps0(&mut self) -> Ps0W<'_, SampSpec> {
        Ps0W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn sn1(&mut self) -> Sn1W<'_, SampSpec> {
        Sn1W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ps1(&mut self) -> Ps1W<'_, SampSpec> {
        Ps1W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn sn2(&mut self) -> Sn2W<'_, SampSpec> {
        Sn2W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ps2(&mut self) -> Ps2W<'_, SampSpec> {
        Ps2W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn sn3(&mut self) -> Sn3W<'_, SampSpec> {
        Sn3W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn ps3(&mut self) -> Ps3W<'_, SampSpec> {
        Ps3W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn sn4(&mut self) -> Sn4W<'_, SampSpec> {
        Sn4W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn ps4(&mut self) -> Ps4W<'_, SampSpec> {
        Ps4W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn sn5(&mut self) -> Sn5W<'_, SampSpec> {
        Sn5W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn ps5(&mut self) -> Ps5W<'_, SampSpec> {
        Ps5W::new(self, 22)
    }
    #[doc = "Bits 24:25"]
    #[inline(always)]
    pub fn sn6(&mut self) -> Sn6W<'_, SampSpec> {
        Sn6W::new(self, 24)
    }
    #[doc = "Bits 26:27"]
    #[inline(always)]
    pub fn ps6(&mut self) -> Ps6W<'_, SampSpec> {
        Ps6W::new(self, 26)
    }
    #[doc = "Bits 28:29"]
    #[inline(always)]
    pub fn sn7(&mut self) -> Sn7W<'_, SampSpec> {
        Sn7W::new(self, 28)
    }
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn ps7(&mut self) -> Ps7W<'_, SampSpec> {
        Ps7W::new(self, 30)
    }
}
#[doc = "SAMP\n\nYou can [`read`](crate::Reg::read) this register and get [`samp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`samp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SampSpec;
impl crate::RegisterSpec for SampSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`samp::R`](R) reader structure"]
impl crate::Readable for SampSpec {}
#[doc = "`write(|w| ..)` method takes [`samp::W`](W) writer structure"]
impl crate::Writable for SampSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAMP to value 0"]
impl crate::Resettable for SampSpec {}
