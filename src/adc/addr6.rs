#[doc = "Register `ADDR6` reader"]
pub type R = crate::R<Addr6Spec>;
#[doc = "Field `ADDR6` reader - "]
pub type Addr6R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr6(&self) -> Addr6R {
        Addr6R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR6\n\nYou can [`read`](crate::Reg::read) this register and get [`addr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr6Spec;
impl crate::RegisterSpec for Addr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr6::R`](R) reader structure"]
impl crate::Readable for Addr6Spec {}
#[doc = "`reset()` method sets ADDR6 to value 0"]
impl crate::Resettable for Addr6Spec {}
