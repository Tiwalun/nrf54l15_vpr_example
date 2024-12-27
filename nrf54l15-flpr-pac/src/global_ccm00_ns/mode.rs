#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "The mode of operation to be used. The settings in this register apply when the CRYPT task is triggered.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: AES CCM packet encryption mode"]
    Encryption = 0,
    #[doc = "1: Deprecated enumerator - This mode will run CCM decryption in the speed of the DATARATE field."]
    Decryption = 1,
    #[doc = "2: AES CCM decryption mode."]
    FastDecryption = 2,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - The mode of operation to be used. The settings in this register apply when the CRYPT task is triggered."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Mode> {
        match self.bits {
            0 => Some(Mode::Encryption),
            1 => Some(Mode::Decryption),
            2 => Some(Mode::FastDecryption),
            _ => None,
        }
    }
    #[doc = "AES CCM packet encryption mode"]
    #[inline(always)]
    pub fn is_encryption(&self) -> bool {
        *self == Mode::Encryption
    }
    #[doc = "Deprecated enumerator - This mode will run CCM decryption in the speed of the DATARATE field."]
    #[inline(always)]
    pub fn is_decryption(&self) -> bool {
        *self == Mode::Decryption
    }
    #[doc = "AES CCM decryption mode."]
    #[inline(always)]
    pub fn is_fast_decryption(&self) -> bool {
        *self == Mode::FastDecryption
    }
}
#[doc = "Field `MODE` writer - The mode of operation to be used. The settings in this register apply when the CRYPT task is triggered."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "AES CCM packet encryption mode"]
    #[inline(always)]
    pub fn encryption(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Encryption)
    }
    #[doc = "Deprecated enumerator - This mode will run CCM decryption in the speed of the DATARATE field."]
    #[inline(always)]
    pub fn decryption(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Decryption)
    }
    #[doc = "AES CCM decryption mode."]
    #[inline(always)]
    pub fn fast_decryption(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::FastDecryption)
    }
}
#[doc = "Protocol and packet format selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Protocol {
    #[doc = "0: Bluetooth Low Energy packet format"]
    Ble = 0,
    #[doc = "1: 802.15.4 packet format"]
    Ieee802154 = 1,
}
impl From<Protocol> for u8 {
    #[inline(always)]
    fn from(variant: Protocol) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Protocol {
    type Ux = u8;
}
impl crate::IsEnum for Protocol {}
#[doc = "Field `PROTOCOL` reader - Protocol and packet format selection"]
pub type ProtocolR = crate::FieldReader<Protocol>;
impl ProtocolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Protocol> {
        match self.bits {
            0 => Some(Protocol::Ble),
            1 => Some(Protocol::Ieee802154),
            _ => None,
        }
    }
    #[doc = "Bluetooth Low Energy packet format"]
    #[inline(always)]
    pub fn is_ble(&self) -> bool {
        *self == Protocol::Ble
    }
    #[doc = "802.15.4 packet format"]
    #[inline(always)]
    pub fn is_ieee802154(&self) -> bool {
        *self == Protocol::Ieee802154
    }
}
#[doc = "Field `PROTOCOL` writer - Protocol and packet format selection"]
pub type ProtocolW<'a, REG> = crate::FieldWriter<'a, REG, 2, Protocol>;
impl<'a, REG> ProtocolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bluetooth Low Energy packet format"]
    #[inline(always)]
    pub fn ble(self) -> &'a mut crate::W<REG> {
        self.variant(Protocol::Ble)
    }
    #[doc = "802.15.4 packet format"]
    #[inline(always)]
    pub fn ieee802154(self) -> &'a mut crate::W<REG> {
        self.variant(Protocol::Ieee802154)
    }
}
#[doc = "Radio data rate that the CCM shall run synchronous with\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Datarate {
    #[doc = "0: 125 Kbps"]
    _125kbit = 0,
    #[doc = "1: 250 Kbps"]
    _250kbit = 1,
    #[doc = "2: 500 Kbps"]
    _500kbit = 2,
    #[doc = "3: 1 Mbps"]
    _1mbit = 3,
    #[doc = "4: 2 Mbps"]
    _2mbit = 4,
    #[doc = "5: 4 Mbps"]
    _4mbit = 5,
}
impl From<Datarate> for u8 {
    #[inline(always)]
    fn from(variant: Datarate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Datarate {
    type Ux = u8;
}
impl crate::IsEnum for Datarate {}
#[doc = "Field `DATARATE` reader - Radio data rate that the CCM shall run synchronous with"]
pub type DatarateR = crate::FieldReader<Datarate>;
impl DatarateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Datarate> {
        match self.bits {
            0 => Some(Datarate::_125kbit),
            1 => Some(Datarate::_250kbit),
            2 => Some(Datarate::_500kbit),
            3 => Some(Datarate::_1mbit),
            4 => Some(Datarate::_2mbit),
            5 => Some(Datarate::_4mbit),
            _ => None,
        }
    }
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn is_125kbit(&self) -> bool {
        *self == Datarate::_125kbit
    }
    #[doc = "250 Kbps"]
    #[inline(always)]
    pub fn is_250kbit(&self) -> bool {
        *self == Datarate::_250kbit
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn is_500kbit(&self) -> bool {
        *self == Datarate::_500kbit
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn is_1mbit(&self) -> bool {
        *self == Datarate::_1mbit
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn is_2mbit(&self) -> bool {
        *self == Datarate::_2mbit
    }
    #[doc = "4 Mbps"]
    #[inline(always)]
    pub fn is_4mbit(&self) -> bool {
        *self == Datarate::_4mbit
    }
}
#[doc = "Field `DATARATE` writer - Radio data rate that the CCM shall run synchronous with"]
pub type DatarateW<'a, REG> = crate::FieldWriter<'a, REG, 3, Datarate>;
impl<'a, REG> DatarateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "125 Kbps"]
    #[inline(always)]
    pub fn _125kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_125kbit)
    }
    #[doc = "250 Kbps"]
    #[inline(always)]
    pub fn _250kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_250kbit)
    }
    #[doc = "500 Kbps"]
    #[inline(always)]
    pub fn _500kbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_500kbit)
    }
    #[doc = "1 Mbps"]
    #[inline(always)]
    pub fn _1mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_1mbit)
    }
    #[doc = "2 Mbps"]
    #[inline(always)]
    pub fn _2mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_2mbit)
    }
    #[doc = "4 Mbps"]
    #[inline(always)]
    pub fn _4mbit(self) -> &'a mut crate::W<REG> {
        self.variant(Datarate::_4mbit)
    }
}
#[doc = "CCM MAC length (bytes)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Maclen {
    #[doc = "0: M = 0 This is a special case for CCM* where encryption is required but not authentication"]
    M0 = 0,
    #[doc = "1: M = 4"]
    M4 = 1,
    #[doc = "2: M = 6"]
    M6 = 2,
    #[doc = "3: M = 8"]
    M8 = 3,
    #[doc = "4: M = 10"]
    M10 = 4,
    #[doc = "5: M = 12"]
    M12 = 5,
    #[doc = "6: M = 14"]
    M14 = 6,
    #[doc = "7: M = 16"]
    M16 = 7,
}
impl From<Maclen> for u8 {
    #[inline(always)]
    fn from(variant: Maclen) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Maclen {
    type Ux = u8;
}
impl crate::IsEnum for Maclen {}
#[doc = "Field `MACLEN` reader - CCM MAC length (bytes)"]
pub type MaclenR = crate::FieldReader<Maclen>;
impl MaclenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Maclen {
        match self.bits {
            0 => Maclen::M0,
            1 => Maclen::M4,
            2 => Maclen::M6,
            3 => Maclen::M8,
            4 => Maclen::M10,
            5 => Maclen::M12,
            6 => Maclen::M14,
            7 => Maclen::M16,
            _ => unreachable!(),
        }
    }
    #[doc = "M = 0 This is a special case for CCM* where encryption is required but not authentication"]
    #[inline(always)]
    pub fn is_m0(&self) -> bool {
        *self == Maclen::M0
    }
    #[doc = "M = 4"]
    #[inline(always)]
    pub fn is_m4(&self) -> bool {
        *self == Maclen::M4
    }
    #[doc = "M = 6"]
    #[inline(always)]
    pub fn is_m6(&self) -> bool {
        *self == Maclen::M6
    }
    #[doc = "M = 8"]
    #[inline(always)]
    pub fn is_m8(&self) -> bool {
        *self == Maclen::M8
    }
    #[doc = "M = 10"]
    #[inline(always)]
    pub fn is_m10(&self) -> bool {
        *self == Maclen::M10
    }
    #[doc = "M = 12"]
    #[inline(always)]
    pub fn is_m12(&self) -> bool {
        *self == Maclen::M12
    }
    #[doc = "M = 14"]
    #[inline(always)]
    pub fn is_m14(&self) -> bool {
        *self == Maclen::M14
    }
    #[doc = "M = 16"]
    #[inline(always)]
    pub fn is_m16(&self) -> bool {
        *self == Maclen::M16
    }
}
#[doc = "Field `MACLEN` writer - CCM MAC length (bytes)"]
pub type MaclenW<'a, REG> = crate::FieldWriter<'a, REG, 3, Maclen, crate::Safe>;
impl<'a, REG> MaclenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "M = 0 This is a special case for CCM* where encryption is required but not authentication"]
    #[inline(always)]
    pub fn m0(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M0)
    }
    #[doc = "M = 4"]
    #[inline(always)]
    pub fn m4(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M4)
    }
    #[doc = "M = 6"]
    #[inline(always)]
    pub fn m6(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M6)
    }
    #[doc = "M = 8"]
    #[inline(always)]
    pub fn m8(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M8)
    }
    #[doc = "M = 10"]
    #[inline(always)]
    pub fn m10(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M10)
    }
    #[doc = "M = 12"]
    #[inline(always)]
    pub fn m12(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M12)
    }
    #[doc = "M = 14"]
    #[inline(always)]
    pub fn m14(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M14)
    }
    #[doc = "M = 16"]
    #[inline(always)]
    pub fn m16(self) -> &'a mut crate::W<REG> {
        self.variant(Maclen::M16)
    }
}
impl R {
    #[doc = "Bits 0:1 - The mode of operation to be used. The settings in this register apply when the CRYPT task is triggered."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - Protocol and packet format selection"]
    #[inline(always)]
    pub fn protocol(&self) -> ProtocolR {
        ProtocolR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    pub fn datarate(&self) -> DatarateR {
        DatarateR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:26 - CCM MAC length (bytes)"]
    #[inline(always)]
    pub fn maclen(&self) -> MaclenR {
        MaclenR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - The mode of operation to be used. The settings in this register apply when the CRYPT task is triggered."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<ModeSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Protocol and packet format selection"]
    #[inline(always)]
    pub fn protocol(&mut self) -> ProtocolW<ModeSpec> {
        ProtocolW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Radio data rate that the CCM shall run synchronous with"]
    #[inline(always)]
    pub fn datarate(&mut self) -> DatarateW<ModeSpec> {
        DatarateW::new(self, 16)
    }
    #[doc = "Bits 24:26 - CCM MAC length (bytes)"]
    #[inline(always)]
    pub fn maclen(&mut self) -> MaclenW<ModeSpec> {
        MaclenW::new(self, 24)
    }
}
#[doc = "Operation mode\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0x01;
}
