#[doc = "Register `TCNT` reader"]
pub type R = crate::R<TcntSpec>;
#[doc = "Register `TCNT` writer"]
pub type W = crate::W<TcntSpec>;
#[doc = "Field `TCNTL` reader - "]
pub type TcntlR = crate::FieldReader<u16>;
#[doc = "Field `TCNTL` writer - "]
pub type TcntlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TCNTH` reader - "]
pub type TcnthR = crate::FieldReader<u16>;
#[doc = "Field `TCNTH` writer - "]
pub type TcnthW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tcntl(&self) -> TcntlR {
        TcntlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tcnth(&self) -> TcnthR {
        TcnthR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCNT")
            .field("tcnth", &self.tcnth())
            .field("tcntl", &self.tcntl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn tcntl(&mut self) -> TcntlW<'_, TcntSpec> {
        TcntlW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tcnth(&mut self) -> TcnthW<'_, TcntSpec> {
        TcnthW::new(self, 16)
    }
}
#[doc = "TCNT\n\nYou can [`read`](crate::Reg::read) this register and get [`tcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcntSpec;
impl crate::RegisterSpec for TcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcnt::R`](R) reader structure"]
impl crate::Readable for TcntSpec {}
#[doc = "`write(|w| ..)` method takes [`tcnt::W`](W) writer structure"]
impl crate::Writable for TcntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCNT to value 0"]
impl crate::Resettable for TcntSpec {}
