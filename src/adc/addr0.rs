#[doc = "Register `ADDR0` reader"]
pub type R = crate::R<Addr0Spec>;
#[doc = "Field `ADDR0` reader - "]
pub type Addr0R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr0(&self) -> Addr0R {
        Addr0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR0\n\nYou can [`read`](crate::Reg::read) this register and get [`addr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr0Spec;
impl crate::RegisterSpec for Addr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr0::R`](R) reader structure"]
impl crate::Readable for Addr0Spec {}
#[doc = "`reset()` method sets ADDR0 to value 0"]
impl crate::Resettable for Addr0Spec {}
