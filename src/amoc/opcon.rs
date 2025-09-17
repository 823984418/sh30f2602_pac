#[doc = "Register `OPCON` reader"]
pub type R = crate::R<OpconSpec>;
#[doc = "Register `OPCON` writer"]
pub type W = crate::W<OpconSpec>;
#[doc = "Field `OP0EN` reader - "]
pub type Op0enR = crate::BitReader;
#[doc = "Field `OP0EN` writer - "]
pub type Op0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP1EN` reader - "]
pub type Op1enR = crate::BitReader;
#[doc = "Field `OP1EN` writer - "]
pub type Op1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP2EN` reader - "]
pub type Op2enR = crate::BitReader;
#[doc = "Field `OP2EN` writer - "]
pub type Op2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP3EN` reader - "]
pub type Op3enR = crate::BitReader;
#[doc = "Field `OP3EN` writer - "]
pub type Op3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u32>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn op0en(&self) -> Op0enR {
        Op0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn op1en(&self) -> Op1enR {
        Op1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn op2en(&self) -> Op2enR {
        Op2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn op3en(&self) -> Op3enR {
        Op3enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn op0en(&mut self) -> Op0enW<'_, OpconSpec> {
        Op0enW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn op1en(&mut self) -> Op1enW<'_, OpconSpec> {
        Op1enW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn op2en(&mut self) -> Op2enW<'_, OpconSpec> {
        Op2enW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn op3en(&mut self) -> Op3enW<'_, OpconSpec> {
        Op3enW::new(self, 3)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, OpconSpec> {
        Rev0W::new(self, 4)
    }
}
#[doc = "OPCON\n\nYou can [`read`](crate::Reg::read) this register and get [`opcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OpconSpec;
impl crate::RegisterSpec for OpconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opcon::R`](R) reader structure"]
impl crate::Readable for OpconSpec {}
#[doc = "`write(|w| ..)` method takes [`opcon::W`](W) writer structure"]
impl crate::Writable for OpconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPCON to value 0"]
impl crate::Resettable for OpconSpec {}
