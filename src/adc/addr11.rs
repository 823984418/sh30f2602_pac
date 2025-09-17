#[doc = "Register `ADDR11` reader"]
pub type R = crate::R<Addr11Spec>;
#[doc = "Field `ADDR11` reader - "]
pub type Addr11R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr11(&self) -> Addr11R {
        Addr11R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR11\n\nYou can [`read`](crate::Reg::read) this register and get [`addr11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr11Spec;
impl crate::RegisterSpec for Addr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr11::R`](R) reader structure"]
impl crate::Readable for Addr11Spec {}
#[doc = "`reset()` method sets ADDR11 to value 0"]
impl crate::Resettable for Addr11Spec {}
