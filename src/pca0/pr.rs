#[doc = "Register `PR` reader"]
pub type R = crate::R<PrSpec>;
#[doc = "Register `PR` writer"]
pub type W = crate::W<PrSpec>;
#[doc = "Field `PRL` reader - "]
pub type PrlR = crate::FieldReader<u16>;
#[doc = "Field `PRL` writer - "]
pub type PrlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PRH` reader - "]
pub type PrhR = crate::FieldReader<u16>;
#[doc = "Field `PRH` writer - "]
pub type PrhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn prl(&self) -> PrlR {
        PrlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn prh(&self) -> PrhR {
        PrhR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PR")
            .field("prh", &self.prh())
            .field("prl", &self.prl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn prl(&mut self) -> PrlW<'_, PrSpec> {
        PrlW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn prh(&mut self) -> PrhW<'_, PrSpec> {
        PrhW::new(self, 16)
    }
}
#[doc = "PR\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrSpec;
impl crate::RegisterSpec for PrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pr::R`](R) reader structure"]
impl crate::Readable for PrSpec {}
#[doc = "`write(|w| ..)` method takes [`pr::W`](W) writer structure"]
impl crate::Writable for PrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PR to value 0xffff_ffff"]
impl crate::Resettable for PrSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
