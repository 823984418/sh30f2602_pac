#[doc = "Register `ADDR2` reader"]
pub type R = crate::R<Addr2Spec>;
#[doc = "Field `ADDR2` reader - "]
pub type Addr2R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr2(&self) -> Addr2R {
        Addr2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR2\n\nYou can [`read`](crate::Reg::read) this register and get [`addr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr2Spec;
impl crate::RegisterSpec for Addr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr2::R`](R) reader structure"]
impl crate::Readable for Addr2Spec {}
#[doc = "`reset()` method sets ADDR2 to value 0"]
impl crate::Resettable for Addr2Spec {}
