#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `SADDR` reader - "]
pub type SaddrR = crate::FieldReader;
#[doc = "Field `SADDR` writer - "]
pub type SaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SMAR` reader - "]
pub type SmarR = crate::FieldReader;
#[doc = "Field `SMAR` writer - "]
pub type SmarW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saddr(&self) -> SaddrR {
        SaddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn smar(&self) -> SmarR {
        SmarR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR")
            .field("rev0", &self.rev0())
            .field("smar", &self.smar())
            .field("saddr", &self.saddr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saddr(&mut self) -> SaddrW<'_, AddrSpec> {
        SaddrW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn smar(&mut self) -> SmarW<'_, AddrSpec> {
        SmarW::new(self, 8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, AddrSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "ADDR\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {}
