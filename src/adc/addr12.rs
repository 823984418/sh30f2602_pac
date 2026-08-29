#[doc = "Register `ADDR12` reader"]
pub type R = crate::R<Addr12Spec>;
#[doc = "Field `ADDR12` reader - "]
pub type Addr12R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr12(&self) -> Addr12R {
        Addr12R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR12")
            .field("rev0", &self.rev0())
            .field("addr12", &self.addr12())
            .finish()
    }
}
#[doc = "ADDR12\n\nYou can [`read`](crate::Reg::read) this register and get [`addr12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr12Spec;
impl crate::RegisterSpec for Addr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr12::R`](R) reader structure"]
impl crate::Readable for Addr12Spec {}
#[doc = "`reset()` method sets ADDR12 to value 0"]
impl crate::Resettable for Addr12Spec {}
