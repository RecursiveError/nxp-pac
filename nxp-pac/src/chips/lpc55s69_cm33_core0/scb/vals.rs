#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Bfhfnmins {
    #[doc = "BusFault, HardFault, and NMI are Secure."]
    Secure = 0x0,
    #[doc = "BusFault and NMI are Non-secure and exceptions can target Non-secure HardFault."]
    NonSecure = 0x01,
}
impl Bfhfnmins {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Bfhfnmins {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Bfhfnmins {
    #[inline(always)]
    fn from(val: u8) -> Bfhfnmins {
        Bfhfnmins::from_bits(val)
    }
}
impl From<Bfhfnmins> for u8 {
    #[inline(always)]
    fn from(val: Bfhfnmins) -> u8 {
        Bfhfnmins::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Endianness {
    #[doc = "Little-endian."]
    LittleEndian = 0x0,
    #[doc = "Big-endian."]
    BigEndian = 0x01,
}
impl Endianness {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Endianness {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Endianness {
    #[inline(always)]
    fn from(val: u8) -> Endianness {
        Endianness::from_bits(val)
    }
}
impl From<Endianness> for u8 {
    #[inline(always)]
    fn from(val: Endianness) -> u8 {
        Endianness::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Pris {
    #[doc = "Priority ranges of Secure and Non-secure exceptions are identical."]
    SamePriority = 0x0,
    #[doc = "Non-secure exceptions are de-prioritized."]
    SecurePrioritized = 0x01,
}
impl Pris {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Pris {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Pris {
    #[inline(always)]
    fn from(val: u8) -> Pris {
        Pris::from_bits(val)
    }
}
impl From<Pris> for u8 {
    #[inline(always)]
    fn from(val: Pris) -> u8 {
        Pris::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sevonpend {
    #[doc = "Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded."]
    ExcludeDisabledInterrupts = 0x0,
    #[doc = "Enabled events and all interrupts, including disabled interrupts, can wakeup the processor."]
    IncludeDisabledInterrupts = 0x01,
}
impl Sevonpend {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sevonpend {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sevonpend {
    #[inline(always)]
    fn from(val: u8) -> Sevonpend {
        Sevonpend::from_bits(val)
    }
}
impl From<Sevonpend> for u8 {
    #[inline(always)]
    fn from(val: Sevonpend) -> u8 {
        Sevonpend::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepdeep {
    #[doc = "Sleep."]
    Sleep = 0x0,
    #[doc = "Deep sleep."]
    DeepSleep = 0x01,
}
impl Sleepdeep {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepdeep {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepdeep {
    #[inline(always)]
    fn from(val: u8) -> Sleepdeep {
        Sleepdeep::from_bits(val)
    }
}
impl From<Sleepdeep> for u8 {
    #[inline(always)]
    fn from(val: Sleepdeep) -> u8 {
        Sleepdeep::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sleepdeeps {
    #[doc = "The SLEEPDEEP bit is accessible from both Security states."]
    SecureAndNonSecure = 0x0,
    #[doc = "The SLEEPDEEP bit behaves as RAZ/WI when accessed from the Non-secure state."]
    SecureOnly = 0x01,
}
impl Sleepdeeps {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sleepdeeps {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sleepdeeps {
    #[inline(always)]
    fn from(val: u8) -> Sleepdeeps {
        Sleepdeeps::from_bits(val)
    }
}
impl From<Sleepdeeps> for u8 {
    #[inline(always)]
    fn from(val: Sleepdeeps) -> u8 {
        Sleepdeeps::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysresetreq {
    #[doc = "Do not request a system reset."]
    NoRequest = 0x0,
    #[doc = "Request a system reset."]
    RequestReset = 0x01,
}
impl Sysresetreq {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysresetreq {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysresetreq {
    #[inline(always)]
    fn from(val: u8) -> Sysresetreq {
        Sysresetreq::from_bits(val)
    }
}
impl From<Sysresetreq> for u8 {
    #[inline(always)]
    fn from(val: Sysresetreq) -> u8 {
        Sysresetreq::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Sysresetreqs {
    #[doc = "SYSRESETREQ functionality is available to both Security states."]
    SecureAndNonSecure = 0x0,
    #[doc = "SYSRESETREQ functionality is only available to Secure state."]
    SecureOnly = 0x01,
}
impl Sysresetreqs {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Sysresetreqs {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Sysresetreqs {
    #[inline(always)]
    fn from(val: u8) -> Sysresetreqs {
        Sysresetreqs::from_bits(val)
    }
}
impl From<Sysresetreqs> for u8 {
    #[inline(always)]
    fn from(val: Sysresetreqs) -> u8 {
        Sysresetreqs::to_bits(val)
    }
}
