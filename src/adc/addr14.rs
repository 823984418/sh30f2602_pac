#[doc = "Register `ADDR14` reader"]
pub type R = crate::R<Addr14Spec>;
#[doc = "Field `ADDR14` reader - "]
pub type Addr14R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr14(&self) -> Addr14R {
        Addr14R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR14\n\nYou can [`read`](crate::Reg::read) this register and get [`addr14::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr14Spec;
impl crate::RegisterSpec for Addr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr14::R`](R) reader structure"]
impl crate::Readable for Addr14Spec {}
#[doc = "`reset()` method sets ADDR14 to value 0"]
impl crate::Resettable for Addr14Spec {}
