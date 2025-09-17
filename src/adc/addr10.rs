#[doc = "Register `ADDR10` reader"]
pub type R = crate::R<Addr10Spec>;
#[doc = "Field `ADDR10` reader - "]
pub type Addr10R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr10(&self) -> Addr10R {
        Addr10R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "ADDR10\n\nYou can [`read`](crate::Reg::read) this register and get [`addr10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr10Spec;
impl crate::RegisterSpec for Addr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr10::R`](R) reader structure"]
impl crate::Readable for Addr10Spec {}
#[doc = "`reset()` method sets ADDR10 to value 0"]
impl crate::Resettable for Addr10Spec {}
