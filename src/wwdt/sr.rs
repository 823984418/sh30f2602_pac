#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SrSpec>;
#[doc = "Field `TCNT` reader - "]
pub type TcntR = crate::FieldReader;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WWDTIF` reader - "]
pub type WwdtifR = crate::BitReader;
#[doc = "Field `WWDTIF` writer - "]
pub type WwdtifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn tcnt(&self) -> TcntR {
        TcntR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wwdtif(&self) -> WwdtifR {
        WwdtifR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("rev0", &self.rev0())
            .field("wwdtif", &self.wwdtif())
            .field("rev1", &self.rev1())
            .field("tcnt", &self.tcnt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:14"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, SrSpec> {
        Rev1W::new(self, 8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn wwdtif(&mut self) -> WwdtifW<'_, SrSpec> {
        WwdtifW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, SrSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "SR\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR to value 0xff"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0xff;
}
