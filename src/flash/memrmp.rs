#[doc = "Register `MEMRMP` reader"]
pub type R = crate::R<MemrmpSpec>;
#[doc = "Register `MEMRMP` writer"]
pub type W = crate::W<MemrmpSpec>;
#[doc = "Field `MEMMODE` reader - "]
pub type MemmodeR = crate::FieldReader<u16>;
#[doc = "Field `MEMMODE` writer - "]
pub type MemmodeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` writer - "]
pub type Rev0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn memmode(&self) -> MemmodeR {
        MemmodeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn memmode(&mut self) -> MemmodeW<'_, MemrmpSpec> {
        MemmodeW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&mut self) -> Rev0W<'_, MemrmpSpec> {
        Rev0W::new(self, 16)
    }
}
#[doc = "MEMRMP\n\nYou can [`read`](crate::Reg::read) this register and get [`memrmp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`memrmp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MemrmpSpec;
impl crate::RegisterSpec for MemrmpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`memrmp::R`](R) reader structure"]
impl crate::Readable for MemrmpSpec {}
#[doc = "`write(|w| ..)` method takes [`memrmp::W`](W) writer structure"]
impl crate::Writable for MemrmpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEMRMP to value 0"]
impl crate::Resettable for MemrmpSpec {}
