#[doc = "Register `ADDR5` reader"]
pub type R = crate::R<Addr5Spec>;
#[doc = "Field `ADDR5` reader - "]
pub type Addr5R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr5(&self) -> Addr5R {
        Addr5R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR5")
            .field("rev0", &self.rev0())
            .field("addr5", &self.addr5())
            .finish()
    }
}
#[doc = "ADDR5\n\nYou can [`read`](crate::Reg::read) this register and get [`addr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr5Spec;
impl crate::RegisterSpec for Addr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr5::R`](R) reader structure"]
impl crate::Readable for Addr5Spec {}
#[doc = "`reset()` method sets ADDR5 to value 0"]
impl crate::Resettable for Addr5Spec {}
