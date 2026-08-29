#[doc = "Register `ADDR9` reader"]
pub type R = crate::R<Addr9Spec>;
#[doc = "Field `ADDR9` reader - "]
pub type Addr9R = crate::FieldReader<u16>;
#[doc = "Field `rev0` reader - "]
pub type Rev0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn addr9(&self) -> Addr9R {
        Addr9R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn rev0(&self) -> Rev0R {
        Rev0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR9")
            .field("rev0", &self.rev0())
            .field("addr9", &self.addr9())
            .finish()
    }
}
#[doc = "ADDR9\n\nYou can [`read`](crate::Reg::read) this register and get [`addr9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Addr9Spec;
impl crate::RegisterSpec for Addr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr9::R`](R) reader structure"]
impl crate::Readable for Addr9Spec {}
#[doc = "`reset()` method sets ADDR9 to value 0"]
impl crate::Resettable for Addr9Spec {}
