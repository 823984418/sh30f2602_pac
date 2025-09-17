#[doc = "Register `ADDR3` reader"]
pub type R = crate::R<Addr3Spec>;
#[doc = "Field `ADDR3` reader - "]
pub type Addr3R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr3(&self) -> Addr3R {
        Addr3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR3\n\nYou can [`read`](crate::Reg::read) this register and get [`addr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr3Spec;
impl crate::RegisterSpec for Addr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr3::R`](R) reader structure"]
impl crate::Readable for Addr3Spec {}
#[doc = "`reset()` method sets ADDR3 to value 0"]
impl crate::Resettable for Addr3Spec {}
