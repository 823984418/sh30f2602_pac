#[doc = "Register `TIMINTF` reader"]
pub type R = crate::R<TimintfSpec>;
#[doc = "Register `TIMINTF` writer"]
pub type W = crate::W<TimintfSpec>;
#[doc = "Field `TF` reader - "]
pub type TfR = crate::BitReader;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader<u16>;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TFC` reader - "]
pub type TfcR = crate::BitReader;
#[doc = "Field `TFC` writer - "]
pub type TfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tf(&self) -> TfR {
        TfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tfc(&self) -> TfcR {
        TfcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:15"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, TimintfSpec> {
        Rev1W::new(self, 1)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn tfc(&mut self) -> TfcW<'_, TimintfSpec> {
        TfcW::new(self, 16)
    }
    #[doc = "Bits 17:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, TimintfSpec> {
        Rev0W::new(self, 17)
    }
}
#[doc = "TIMINTF\n\nYou can [`read`](crate::Reg::read) this register and get [`timintf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timintf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimintfSpec;
impl crate::RegisterSpec for TimintfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timintf::R`](R) reader structure"]
impl crate::Readable for TimintfSpec {}
#[doc = "`write(|w| ..)` method takes [`timintf::W`](W) writer structure"]
impl crate::Writable for TimintfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMINTF to value 0"]
impl crate::Resettable for TimintfSpec {}
