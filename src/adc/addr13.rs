#[doc = "Register `ADDR13` reader"]
pub type R = crate::R<Addr13Spec>;
#[doc = "Field `ADDR13` reader - "]
pub type Addr13R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr13(&self) -> Addr13R {
        Addr13R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR13")
            .field("rev0", &self.rev0())
            .field("addr13", &self.addr13())
            .finish()
    }
}
#[doc = "ADDR13\n\nYou can [`read`](crate::Reg::read) this register and get [`addr13::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr13Spec;
impl crate::RegisterSpec for Addr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr13::R`](R) reader structure"]
impl crate::Readable for Addr13Spec {}
#[doc = "`reset()` method sets ADDR13 to value 0"]
impl crate::Resettable for Addr13Spec {}
