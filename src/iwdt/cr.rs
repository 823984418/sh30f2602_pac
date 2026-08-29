#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `IWDTRLR` reader - "]
pub type IwdtrlrR = crate::FieldReader<u16>;
#[doc = "Field `IWDTRLR` writer - "]
pub type IwdtrlrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `IWDTPR` reader - "]
pub type IwdtprR = crate::FieldReader;
#[doc = "Field `IWDTPR` writer - "]
pub type IwdtprW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IWDTON` reader - "]
pub type IwdtonR = crate::BitReader;
#[doc = "Field `IWDTON` writer - "]
pub type IwdtonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn iwdtrlr(&self) -> IwdtrlrR {
        IwdtrlrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn iwdtpr(&self) -> IwdtprR {
        IwdtprR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn iwdton(&self) -> IwdtonR {
        IwdtonR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("lock", &self.lock())
            .field("iwdton", &self.iwdton())
            .field("iwdtpr", &self.iwdtpr())
            .field("iwdtrlr", &self.iwdtrlr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn iwdtrlr(&mut self) -> IwdtrlrW<'_, CrSpec> {
        IwdtrlrW::new(self, 0)
    }
    #[doc = "Bits 12:14"]
    #[inline(always)]
    pub fn iwdtpr(&mut self) -> IwdtprW<'_, CrSpec> {
        IwdtprW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn iwdton(&mut self) -> IwdtonW<'_, CrSpec> {
        IwdtonW::new(self, 15)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, CrSpec> {
        LockW::new(self, 16)
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
#[doc = "`reset()` method sets CR to value 0x3fff"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
