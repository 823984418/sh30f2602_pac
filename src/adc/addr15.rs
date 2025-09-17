#[doc = "Register `ADDR15` reader"]
pub type R = crate::R<Addr15Spec>;
#[doc = "Field `ADDR15` reader - "]
pub type Addr15R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr15(&self) -> Addr15R {
        Addr15R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR15\n\nYou can [`read`](crate::Reg::read) this register and get [`addr15::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr15Spec;
impl crate::RegisterSpec for Addr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr15::R`](R) reader structure"]
impl crate::Readable for Addr15Spec {}
#[doc = "`reset()` method sets ADDR15 to value 0"]
impl crate::Resettable for Addr15Spec {}
