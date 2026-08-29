#[doc = "Register `PSQ` reader"]
pub type R = crate::R<PsqSpec>;
#[doc = "Register `PSQ` writer"]
pub type W = crate::W<PsqSpec>;
#[doc = "Field `PSQL` reader - "]
pub type PsqlR = crate::FieldReader<u16>;
#[doc = "Field `PSQL` writer - "]
pub type PsqlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PSQH` reader - "]
pub type PsqhR = crate::FieldReader<u16>;
#[doc = "Field `PSQH` writer - "]
pub type PsqhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn psql(&self) -> PsqlR {
        PsqlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn psqh(&self) -> PsqhR {
        PsqhR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSQ")
            .field("psqh", &self.psqh())
            .field("psql", &self.psql())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn psql(&mut self) -> PsqlW<'_, PsqSpec> {
        PsqlW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn psqh(&mut self) -> PsqhW<'_, PsqSpec> {
        PsqhW::new(self, 16)
    }
}
#[doc = "PSQ\n\nYou can [`read`](crate::Reg::read) this register and get [`psq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PsqSpec;
impl crate::RegisterSpec for PsqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psq::R`](R) reader structure"]
impl crate::Readable for PsqSpec {}
#[doc = "`write(|w| ..)` method takes [`psq::W`](W) writer structure"]
impl crate::Writable for PsqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSQ to value 0"]
impl crate::Resettable for PsqSpec {}
