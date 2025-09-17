#[doc = "Register `SEQCHSEL1` reader"]
pub type R = crate::R<Seqchsel1Spec>;
#[doc = "Register `SEQCHSEL1` writer"]
pub type W = crate::W<Seqchsel1Spec>;
#[doc = "Field `SEQCH8` reader - "]
pub type Seqch8R = crate::FieldReader;
#[doc = "Field `SEQCH8` writer - "]
pub type Seqch8W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH9` reader - "]
pub type Seqch9R = crate::FieldReader;
#[doc = "Field `SEQCH9` writer - "]
pub type Seqch9W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH10` reader - "]
pub type Seqch10R = crate::FieldReader;
#[doc = "Field `SEQCH10` writer - "]
pub type Seqch10W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH11` reader - "]
pub type Seqch11R = crate::FieldReader;
#[doc = "Field `SEQCH11` writer - "]
pub type Seqch11W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH12` reader - "]
pub type Seqch12R = crate::FieldReader;
#[doc = "Field `SEQCH12` writer - "]
pub type Seqch12W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH13` reader - "]
pub type Seqch13R = crate::FieldReader;
#[doc = "Field `SEQCH13` writer - "]
pub type Seqch13W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH14` reader - "]
pub type Seqch14R = crate::FieldReader;
#[doc = "Field `SEQCH14` writer - "]
pub type Seqch14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEQCH15` reader - "]
pub type Seqch15R = crate::FieldReader;
#[doc = "Field `SEQCH15` writer - "]
pub type Seqch15W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn seqch8(&self) -> Seqch8R {
        Seqch8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn seqch9(&self) -> Seqch9R {
        Seqch9R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn seqch10(&self) -> Seqch10R {
        Seqch10R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn seqch11(&self) -> Seqch11R {
        Seqch11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn seqch12(&self) -> Seqch12R {
        Seqch12R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn seqch13(&self) -> Seqch13R {
        Seqch13R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn seqch14(&self) -> Seqch14R {
        Seqch14R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn seqch15(&self) -> Seqch15R {
        Seqch15R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn seqch8(&mut self) -> Seqch8W<'_, Seqchsel1Spec> {
        Seqch8W::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn seqch9(&mut self) -> Seqch9W<'_, Seqchsel1Spec> {
        Seqch9W::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn seqch10(&mut self) -> Seqch10W<'_, Seqchsel1Spec> {
        Seqch10W::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn seqch11(&mut self) -> Seqch11W<'_, Seqchsel1Spec> {
        Seqch11W::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn seqch12(&mut self) -> Seqch12W<'_, Seqchsel1Spec> {
        Seqch12W::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn seqch13(&mut self) -> Seqch13W<'_, Seqchsel1Spec> {
        Seqch13W::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn seqch14(&mut self) -> Seqch14W<'_, Seqchsel1Spec> {
        Seqch14W::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn seqch15(&mut self) -> Seqch15W<'_, Seqchsel1Spec> {
        Seqch15W::new(self, 28)
    }
}
#[doc = "SEQCHSEL1\n\nYou can [`read`](crate::Reg::read) this register and get [`seqchsel1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqchsel1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Seqchsel1Spec;
impl crate::RegisterSpec for Seqchsel1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqchsel1::R`](R) reader structure"]
impl crate::Readable for Seqchsel1Spec {}
#[doc = "`write(|w| ..)` method takes [`seqchsel1::W`](W) writer structure"]
impl crate::Writable for Seqchsel1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEQCHSEL1 to value 0"]
impl crate::Resettable for Seqchsel1Spec {}
