#[doc = "Register `ADDR7` reader"]
pub type R = crate::R<Addr7Spec>;
#[doc = "Field `ADDR7` reader - "]
pub type Addr7R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr7(&self) -> Addr7R {
        Addr7R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR7")
            .field("rev0", &self.rev0())
            .field("addr7", &self.addr7())
            .finish()
    }
}
#[doc = "ADDR7\n\nYou can [`read`](crate::Reg::read) this register and get [`addr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr7Spec;
impl crate::RegisterSpec for Addr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr7::R`](R) reader structure"]
impl crate::Readable for Addr7Spec {}
#[doc = "`reset()` method sets ADDR7 to value 0"]
impl crate::Resettable for Addr7Spec {}
