#[doc = "Register `TDR` reader"]
pub type R = crate::R<TdrSpec>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TDR` reader - "]
pub type TdrR = crate::FieldReader;
#[doc = "Field `TDR` writer - "]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TB8` reader - "]
pub type Tb8R = crate::BitReader;
#[doc = "Field `TB8` writer - "]
pub type Tb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tdr(&self) -> TdrR {
        TdrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tb8(&self) -> Tb8R {
        Tb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TDR")
            .field("rev0", &self.rev0())
            .field("tb8", &self.tb8())
            .field("tdr", &self.tdr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TdrW<'_, TdrSpec> {
        TdrW::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn tb8(&mut self) -> Tb8W<'_, TdrSpec> {
        Tb8W::new(self, 8)
    }
    #[doc = "Bits 9:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, TdrSpec> {
        Rev0W::new(self, 9)
    }
}
#[doc = "TDR\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TdrSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDR to value 0"]
impl crate::Resettable for TdrSpec {}
