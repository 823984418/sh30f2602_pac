#[doc = "Register `HLDOCR` reader"]
pub type R = crate::R<HldocrSpec>;
#[doc = "Register `HLDOCR` writer"]
pub type W = crate::W<HldocrSpec>;
#[doc = "Field `IDC` reader - "]
pub type IdcR = crate::FieldReader;
#[doc = "Field `IDC` writer - "]
pub type IdcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDCEN` reader - "]
pub type IdcenR = crate::BitReader;
#[doc = "Field `IDCEN` writer - "]
pub type IdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `LOCK` reader - "]
pub type LockR = crate::FieldReader<u16>;
#[doc = "Field `LOCK` writer - "]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn idc(&self) -> IdcR {
        IdcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idcen(&self) -> IdcenR {
        IdcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn idc(&mut self) -> IdcW<'_, HldocrSpec> {
        IdcW::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn idcen(&mut self) -> IdcenW<'_, HldocrSpec> {
        IdcenW::new(self, 2)
    }
    #[doc = "Bits 3:15"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, HldocrSpec> {
        Rev0W::new(self, 3)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<'_, HldocrSpec> {
        LockW::new(self, 16)
    }
}
#[doc = "HLDOCR\n\nYou can [`read`](crate::Reg::read) this register and get [`hldocr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hldocr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HldocrSpec;
impl crate::RegisterSpec for HldocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hldocr::R`](R) reader structure"]
impl crate::Readable for HldocrSpec {}
#[doc = "`write(|w| ..)` method takes [`hldocr::W`](W) writer structure"]
impl crate::Writable for HldocrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HLDOCR to value 0"]
impl crate::Resettable for HldocrSpec {}
