#[doc = "Register `ADCON3` reader"]
pub type R = crate::R<Adcon3Spec>;
#[doc = "Register `ADCON3` writer"]
pub type W = crate::W<Adcon3Spec>;
#[doc = "Field `ADSOC` reader - "]
pub type AdsocR = crate::BitReader;
#[doc = "Field `ADSOC` writer - "]
pub type AdsocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adsoc(&self) -> AdsocR {
        AdsocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADCON3")
            .field("rev0", &self.rev0())
            .field("adsoc", &self.adsoc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adsoc(&mut self) -> AdsocW<'_, Adcon3Spec> {
        AdsocW::new(self, 0)
    }
    #[doc = "Bits 1:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, Adcon3Spec> {
        Rev0W::new(self, 1)
    }
}
#[doc = "ADCON3\n\nYou can [`read`](crate::Reg::read) this register and get [`adcon3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcon3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Adcon3Spec;
impl crate::RegisterSpec for Adcon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adcon3::R`](R) reader structure"]
impl crate::Readable for Adcon3Spec {}
#[doc = "`write(|w| ..)` method takes [`adcon3::W`](W) writer structure"]
impl crate::Writable for Adcon3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADCON3 to value 0"]
impl crate::Resettable for Adcon3Spec {}
