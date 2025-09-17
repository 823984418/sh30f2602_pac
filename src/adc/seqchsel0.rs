#[doc = "Register `SEQCHSEL0` reader"]
pub type R = crate::R<Seqchsel0Spec>;
#[doc = "Register `SEQCHSEL0` writer"]
pub type W = crate::W<Seqchsel0Spec>;
#[doc = "Field `SEQCH0` reader - "]
pub type Seqch0R = crate::FieldReader;
#[doc = "Field `SEQCH0` writer - "]
pub type Seqch0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH1` reader - "]
pub type Seqch1R = crate::FieldReader;
#[doc = "Field `SEQCH1` writer - "]
pub type Seqch1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH2` reader - "]
pub type Seqch2R = crate::FieldReader;
#[doc = "Field `SEQCH2` writer - "]
pub type Seqch2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH3` reader - "]
pub type Seqch3R = crate::FieldReader;
#[doc = "Field `SEQCH3` writer - "]
pub type Seqch3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH4` reader - "]
pub type Seqch4R = crate::FieldReader;
#[doc = "Field `SEQCH4` writer - "]
pub type Seqch4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH5` reader - "]
pub type Seqch5R = crate::FieldReader;
#[doc = "Field `SEQCH5` writer - "]
pub type Seqch5W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH6` reader - "]
pub type Seqch6R = crate::FieldReader;
#[doc = "Field `SEQCH6` writer - "]
pub type Seqch6W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH7` reader - "]
pub type Seqch7R = crate::FieldReader;
#[doc = "Field `SEQCH7` writer - "]
pub type Seqch7W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn seqch0(&self) -> Seqch0R {
        Seqch0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn seqch1(&self) -> Seqch1R {
        Seqch1R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn seqch2(&self) -> Seqch2R {
        Seqch2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn seqch3(&self) -> Seqch3R {
        Seqch3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn seqch4(&self) -> Seqch4R {
        Seqch4R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn seqch5(&self) -> Seqch5R {
        Seqch5R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn seqch6(&self) -> Seqch6R {
        Seqch6R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn seqch7(&self) -> Seqch7R {
        Seqch7R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn seqch0(&mut self) -> Seqch0W<'_, Seqchsel0Spec> {
        Seqch0W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn seqch1(&mut self) -> Seqch1W<'_, Seqchsel0Spec> {
        Seqch1W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn seqch2(&mut self) -> Seqch2W<'_, Seqchsel0Spec> {
        Seqch2W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn seqch3(&mut self) -> Seqch3W<'_, Seqchsel0Spec> {
        Seqch3W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn seqch4(&mut self) -> Seqch4W<'_, Seqchsel0Spec> {
        Seqch4W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn seqch5(&mut self) -> Seqch5W<'_, Seqchsel0Spec> {
        Seqch5W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn seqch6(&mut self) -> Seqch6W<'_, Seqchsel0Spec> {
        Seqch6W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn seqch7(&mut self) -> Seqch7W<'_, Seqchsel0Spec> {
        Seqch7W::new(self, 28)
    }
}
#[doc = "SEQCHSEL0\n\nYou can [`read`](crate::Reg::read) this register and get [`seqchsel0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqchsel0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqchsel0Spec;
impl crate::RegisterSpec for Seqchsel0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqchsel0::R`](R) reader structure"]
impl crate::Readable for Seqchsel0Spec {}
#[doc = "`write(|w| ..)` method takes [`seqchsel0::W`](W) writer structure"]
impl crate::Writable for Seqchsel0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQCHSEL0 to value 0"]
impl crate::Resettable for Seqchsel0Spec {}
