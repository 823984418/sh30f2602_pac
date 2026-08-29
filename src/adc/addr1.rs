#[doc = "Register `ADDR1` reader"]
pub type R = crate::R<Addr1Spec>;
#[doc = "Field `ADDR1` reader - "]
pub type Addr1R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr1(&self) -> Addr1R {
        Addr1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR1")
            .field("rev0", &self.rev0())
            .field("addr1", &self.addr1())
            .finish()
    }
}
#[doc = "ADDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`addr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr1Spec;
impl crate::RegisterSpec for Addr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr1::R`](R) reader structure"]
impl crate::Readable for Addr1Spec {}
#[doc = "`reset()` method sets ADDR1 to value 0"]
impl crate::Resettable for Addr1Spec {}
