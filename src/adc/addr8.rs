#[doc = "Register `ADDR8` reader"]
pub type R = crate::R<Addr8Spec>;
#[doc = "Field `ADDR8` reader - "]
pub type Addr8R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr8(&self) -> Addr8R {
        Addr8R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR8\n\nYou can [`read`](crate::Reg::read) this register and get [`addr8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr8Spec;
impl crate::RegisterSpec for Addr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr8::R`](R) reader structure"]
impl crate::Readable for Addr8Spec {}
#[doc = "`reset()` method sets ADDR8 to value 0"]
impl crate::Resettable for Addr8Spec {}
