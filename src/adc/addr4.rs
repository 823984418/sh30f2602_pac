#[doc = "Register `ADDR4` reader"]
pub type R = crate::R<Addr4Spec>;
#[doc = "Field `ADDR4` reader - "]
pub type Addr4R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr4(&self) -> Addr4R {
        Addr4R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR4\n\nYou can [`read`](crate::Reg::read) this register and get [`addr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr4Spec;
impl crate::RegisterSpec for Addr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr4::R`](R) reader structure"]
impl crate::Readable for Addr4Spec {}
#[doc = "`reset()` method sets ADDR4 to value 0"]
impl crate::Resettable for Addr4Spec {}
