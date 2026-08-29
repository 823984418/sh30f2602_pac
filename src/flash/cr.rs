#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `rev1` reader - "]
pub type Rev1R = crate::FieldReader;
#[doc = "Field `rev1` writer - "]
pub type Rev1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `STRT` reader - "]
pub type StrtR = crate::BitReader;
#[doc = "Field `STRT` writer - "]
pub type StrtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - "]
pub type PsizeR = crate::FieldReader;
#[doc = "Field `PSIZE` writer - "]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INFLCK` reader - "]
pub type InflckR = crate::BitReader;
#[doc = "Field `INFLCK` writer - "]
pub type InflckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::BitReader;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E2LCK` reader - "]
pub type E2lckR = crate::BitReader;
#[doc = "Field `E2LCK` writer - "]
pub type E2lckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MNLCK` reader - "]
pub type MnlckR = crate::BitReader;
#[doc = "Field `MNLCK` writer - "]
pub type MnlckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD` reader - "]
pub type CmdR = crate::FieldReader<u16>;
#[doc = "Field `CMD` writer - "]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rev1(&self) -> Rev1R {
        Rev1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn strt(&self) -> StrtR {
        StrtR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn inflck(&self) -> InflckR {
        InflckR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn e2lck(&self) -> E2lckR {
        E2lckR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mnlck(&self) -> MnlckR {
        MnlckR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("cmd", &self.cmd())
            .field("mnlck", &self.mnlck())
            .field("e2lck", &self.e2lck())
            .field("rev0", &self.rev0())
            .field("inflck", &self.inflck())
            .field("psize", &self.psize())
            .field("strt", &self.strt())
            .field("rev1", &self.rev1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn rev1(&mut self) -> Rev1W<'_, CrSpec> {
        Rev1W::new(self, 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn strt(&mut self) -> StrtW<'_, CrSpec> {
        StrtW::new(self, 8)
    }
    #[doc = "Bits 9:11"]
    #[inline(always)]
    pub fn psize(&mut self) -> PsizeW<'_, CrSpec> {
        PsizeW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn inflck(&mut self) -> InflckW<'_, CrSpec> {
        InflckW::new(self, 12)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, CrSpec> {
        Rev0W::new(self, 13)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn e2lck(&mut self) -> E2lckW<'_, CrSpec> {
        E2lckW::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn mnlck(&mut self) -> MnlckW<'_, CrSpec> {
        MnlckW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, CrSpec> {
        CmdW::new(self, 16)
    }
}
#[doc = "CR\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0xd000"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0xd000;
}
