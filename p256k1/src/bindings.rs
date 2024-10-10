#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!((bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len());
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
pub const ECMULT_GEN_PREC_BITS: u32 = 4;
pub const ECMULT_WINDOW_SIZE: u32 = 15;
pub const SECP256K1_FLAGS_TYPE_MASK: u32 = 255;
pub const SECP256K1_FLAGS_TYPE_CONTEXT: u32 = 1;
pub const SECP256K1_FLAGS_TYPE_COMPRESSION: u32 = 2;
pub const SECP256K1_FLAGS_BIT_CONTEXT_VERIFY: u32 = 256;
pub const SECP256K1_FLAGS_BIT_CONTEXT_SIGN: u32 = 512;
pub const SECP256K1_FLAGS_BIT_CONTEXT_DECLASSIFY: u32 = 1024;
pub const SECP256K1_FLAGS_BIT_COMPRESSION: u32 = 256;
pub const SECP256K1_CONTEXT_NONE: u32 = 1;
pub const SECP256K1_CONTEXT_VERIFY: u32 = 257;
pub const SECP256K1_CONTEXT_SIGN: u32 = 513;
pub const SECP256K1_CONTEXT_DECLASSIFY: u32 = 1025;
pub const SECP256K1_EC_COMPRESSED: u32 = 258;
pub const SECP256K1_EC_UNCOMPRESSED: u32 = 2;
pub const SECP256K1_TAG_PUBKEY_EVEN: u32 = 2;
pub const SECP256K1_TAG_PUBKEY_ODD: u32 = 3;
pub const SECP256K1_TAG_PUBKEY_UNCOMPRESSED: u32 = 4;
pub const SECP256K1_TAG_PUBKEY_HYBRID_EVEN: u32 = 6;
pub const SECP256K1_TAG_PUBKEY_HYBRID_ODD: u32 = 7;
pub const __API_TO_BE_DEPRECATED: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_IOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_MACCATALYST: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_WATCHOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_TVOS: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_DRIVERKIT: u32 = 100000;
pub const __API_TO_BE_DEPRECATED_VISIONOS: u32 = 100000;
pub const __MAC_10_0: u32 = 1000;
pub const __MAC_10_1: u32 = 1010;
pub const __MAC_10_2: u32 = 1020;
pub const __MAC_10_3: u32 = 1030;
pub const __MAC_10_4: u32 = 1040;
pub const __MAC_10_5: u32 = 1050;
pub const __MAC_10_6: u32 = 1060;
pub const __MAC_10_7: u32 = 1070;
pub const __MAC_10_8: u32 = 1080;
pub const __MAC_10_9: u32 = 1090;
pub const __MAC_10_10: u32 = 101000;
pub const __MAC_10_10_2: u32 = 101002;
pub const __MAC_10_10_3: u32 = 101003;
pub const __MAC_10_11: u32 = 101100;
pub const __MAC_10_11_2: u32 = 101102;
pub const __MAC_10_11_3: u32 = 101103;
pub const __MAC_10_11_4: u32 = 101104;
pub const __MAC_10_12: u32 = 101200;
pub const __MAC_10_12_1: u32 = 101201;
pub const __MAC_10_12_2: u32 = 101202;
pub const __MAC_10_12_4: u32 = 101204;
pub const __MAC_10_13: u32 = 101300;
pub const __MAC_10_13_1: u32 = 101301;
pub const __MAC_10_13_2: u32 = 101302;
pub const __MAC_10_13_4: u32 = 101304;
pub const __MAC_10_14: u32 = 101400;
pub const __MAC_10_14_1: u32 = 101401;
pub const __MAC_10_14_4: u32 = 101404;
pub const __MAC_10_14_5: u32 = 101405;
pub const __MAC_10_14_6: u32 = 101406;
pub const __MAC_10_15: u32 = 101500;
pub const __MAC_10_15_1: u32 = 101501;
pub const __MAC_10_15_4: u32 = 101504;
pub const __MAC_10_16: u32 = 101600;
pub const __MAC_11_0: u32 = 110000;
pub const __MAC_11_1: u32 = 110100;
pub const __MAC_11_3: u32 = 110300;
pub const __MAC_11_4: u32 = 110400;
pub const __MAC_11_5: u32 = 110500;
pub const __MAC_11_6: u32 = 110600;
pub const __MAC_12_0: u32 = 120000;
pub const __MAC_12_1: u32 = 120100;
pub const __MAC_12_2: u32 = 120200;
pub const __MAC_12_3: u32 = 120300;
pub const __MAC_12_4: u32 = 120400;
pub const __MAC_12_5: u32 = 120500;
pub const __MAC_12_6: u32 = 120600;
pub const __MAC_12_7: u32 = 120700;
pub const __MAC_13_0: u32 = 130000;
pub const __MAC_13_1: u32 = 130100;
pub const __MAC_13_2: u32 = 130200;
pub const __MAC_13_3: u32 = 130300;
pub const __MAC_13_4: u32 = 130400;
pub const __MAC_13_5: u32 = 130500;
pub const __MAC_13_6: u32 = 130600;
pub const __MAC_14_0: u32 = 140000;
pub const __MAC_14_1: u32 = 140100;
pub const __MAC_14_2: u32 = 140200;
pub const __MAC_14_3: u32 = 140300;
pub const __MAC_14_4: u32 = 140400;
pub const __MAC_14_5: u32 = 140500;
pub const __MAC_15_0: u32 = 150000;
pub const __IPHONE_2_0: u32 = 20000;
pub const __IPHONE_2_1: u32 = 20100;
pub const __IPHONE_2_2: u32 = 20200;
pub const __IPHONE_3_0: u32 = 30000;
pub const __IPHONE_3_1: u32 = 30100;
pub const __IPHONE_3_2: u32 = 30200;
pub const __IPHONE_4_0: u32 = 40000;
pub const __IPHONE_4_1: u32 = 40100;
pub const __IPHONE_4_2: u32 = 40200;
pub const __IPHONE_4_3: u32 = 40300;
pub const __IPHONE_5_0: u32 = 50000;
pub const __IPHONE_5_1: u32 = 50100;
pub const __IPHONE_6_0: u32 = 60000;
pub const __IPHONE_6_1: u32 = 60100;
pub const __IPHONE_7_0: u32 = 70000;
pub const __IPHONE_7_1: u32 = 70100;
pub const __IPHONE_8_0: u32 = 80000;
pub const __IPHONE_8_1: u32 = 80100;
pub const __IPHONE_8_2: u32 = 80200;
pub const __IPHONE_8_3: u32 = 80300;
pub const __IPHONE_8_4: u32 = 80400;
pub const __IPHONE_9_0: u32 = 90000;
pub const __IPHONE_9_1: u32 = 90100;
pub const __IPHONE_9_2: u32 = 90200;
pub const __IPHONE_9_3: u32 = 90300;
pub const __IPHONE_10_0: u32 = 100000;
pub const __IPHONE_10_1: u32 = 100100;
pub const __IPHONE_10_2: u32 = 100200;
pub const __IPHONE_10_3: u32 = 100300;
pub const __IPHONE_11_0: u32 = 110000;
pub const __IPHONE_11_1: u32 = 110100;
pub const __IPHONE_11_2: u32 = 110200;
pub const __IPHONE_11_3: u32 = 110300;
pub const __IPHONE_11_4: u32 = 110400;
pub const __IPHONE_12_0: u32 = 120000;
pub const __IPHONE_12_1: u32 = 120100;
pub const __IPHONE_12_2: u32 = 120200;
pub const __IPHONE_12_3: u32 = 120300;
pub const __IPHONE_12_4: u32 = 120400;
pub const __IPHONE_13_0: u32 = 130000;
pub const __IPHONE_13_1: u32 = 130100;
pub const __IPHONE_13_2: u32 = 130200;
pub const __IPHONE_13_3: u32 = 130300;
pub const __IPHONE_13_4: u32 = 130400;
pub const __IPHONE_13_5: u32 = 130500;
pub const __IPHONE_13_6: u32 = 130600;
pub const __IPHONE_13_7: u32 = 130700;
pub const __IPHONE_14_0: u32 = 140000;
pub const __IPHONE_14_1: u32 = 140100;
pub const __IPHONE_14_2: u32 = 140200;
pub const __IPHONE_14_3: u32 = 140300;
pub const __IPHONE_14_5: u32 = 140500;
pub const __IPHONE_14_4: u32 = 140400;
pub const __IPHONE_14_6: u32 = 140600;
pub const __IPHONE_14_7: u32 = 140700;
pub const __IPHONE_14_8: u32 = 140800;
pub const __IPHONE_15_0: u32 = 150000;
pub const __IPHONE_15_1: u32 = 150100;
pub const __IPHONE_15_2: u32 = 150200;
pub const __IPHONE_15_3: u32 = 150300;
pub const __IPHONE_15_4: u32 = 150400;
pub const __IPHONE_15_5: u32 = 150500;
pub const __IPHONE_15_6: u32 = 150600;
pub const __IPHONE_15_7: u32 = 150700;
pub const __IPHONE_15_8: u32 = 150800;
pub const __IPHONE_16_0: u32 = 160000;
pub const __IPHONE_16_1: u32 = 160100;
pub const __IPHONE_16_2: u32 = 160200;
pub const __IPHONE_16_3: u32 = 160300;
pub const __IPHONE_16_4: u32 = 160400;
pub const __IPHONE_16_5: u32 = 160500;
pub const __IPHONE_16_6: u32 = 160600;
pub const __IPHONE_16_7: u32 = 160700;
pub const __IPHONE_17_0: u32 = 170000;
pub const __IPHONE_17_1: u32 = 170100;
pub const __IPHONE_17_2: u32 = 170200;
pub const __IPHONE_17_3: u32 = 170300;
pub const __IPHONE_17_4: u32 = 170400;
pub const __IPHONE_17_5: u32 = 170500;
pub const __IPHONE_18_0: u32 = 180000;
pub const __WATCHOS_1_0: u32 = 10000;
pub const __WATCHOS_2_0: u32 = 20000;
pub const __WATCHOS_2_1: u32 = 20100;
pub const __WATCHOS_2_2: u32 = 20200;
pub const __WATCHOS_3_0: u32 = 30000;
pub const __WATCHOS_3_1: u32 = 30100;
pub const __WATCHOS_3_1_1: u32 = 30101;
pub const __WATCHOS_3_2: u32 = 30200;
pub const __WATCHOS_4_0: u32 = 40000;
pub const __WATCHOS_4_1: u32 = 40100;
pub const __WATCHOS_4_2: u32 = 40200;
pub const __WATCHOS_4_3: u32 = 40300;
pub const __WATCHOS_5_0: u32 = 50000;
pub const __WATCHOS_5_1: u32 = 50100;
pub const __WATCHOS_5_2: u32 = 50200;
pub const __WATCHOS_5_3: u32 = 50300;
pub const __WATCHOS_6_0: u32 = 60000;
pub const __WATCHOS_6_1: u32 = 60100;
pub const __WATCHOS_6_2: u32 = 60200;
pub const __WATCHOS_7_0: u32 = 70000;
pub const __WATCHOS_7_1: u32 = 70100;
pub const __WATCHOS_7_2: u32 = 70200;
pub const __WATCHOS_7_3: u32 = 70300;
pub const __WATCHOS_7_4: u32 = 70400;
pub const __WATCHOS_7_5: u32 = 70500;
pub const __WATCHOS_7_6: u32 = 70600;
pub const __WATCHOS_8_0: u32 = 80000;
pub const __WATCHOS_8_1: u32 = 80100;
pub const __WATCHOS_8_3: u32 = 80300;
pub const __WATCHOS_8_4: u32 = 80400;
pub const __WATCHOS_8_5: u32 = 80500;
pub const __WATCHOS_8_6: u32 = 80600;
pub const __WATCHOS_8_7: u32 = 80700;
pub const __WATCHOS_8_8: u32 = 80800;
pub const __WATCHOS_9_0: u32 = 90000;
pub const __WATCHOS_9_1: u32 = 90100;
pub const __WATCHOS_9_2: u32 = 90200;
pub const __WATCHOS_9_3: u32 = 90300;
pub const __WATCHOS_9_4: u32 = 90400;
pub const __WATCHOS_9_5: u32 = 90500;
pub const __WATCHOS_9_6: u32 = 90600;
pub const __WATCHOS_10_0: u32 = 100000;
pub const __WATCHOS_10_1: u32 = 100100;
pub const __WATCHOS_10_2: u32 = 100200;
pub const __WATCHOS_10_3: u32 = 100300;
pub const __WATCHOS_10_4: u32 = 100400;
pub const __WATCHOS_10_5: u32 = 100500;
pub const __WATCHOS_11_0: u32 = 110000;
pub const __TVOS_9_0: u32 = 90000;
pub const __TVOS_9_1: u32 = 90100;
pub const __TVOS_9_2: u32 = 90200;
pub const __TVOS_10_0: u32 = 100000;
pub const __TVOS_10_0_1: u32 = 100001;
pub const __TVOS_10_1: u32 = 100100;
pub const __TVOS_10_2: u32 = 100200;
pub const __TVOS_11_0: u32 = 110000;
pub const __TVOS_11_1: u32 = 110100;
pub const __TVOS_11_2: u32 = 110200;
pub const __TVOS_11_3: u32 = 110300;
pub const __TVOS_11_4: u32 = 110400;
pub const __TVOS_12_0: u32 = 120000;
pub const __TVOS_12_1: u32 = 120100;
pub const __TVOS_12_2: u32 = 120200;
pub const __TVOS_12_3: u32 = 120300;
pub const __TVOS_12_4: u32 = 120400;
pub const __TVOS_13_0: u32 = 130000;
pub const __TVOS_13_2: u32 = 130200;
pub const __TVOS_13_3: u32 = 130300;
pub const __TVOS_13_4: u32 = 130400;
pub const __TVOS_14_0: u32 = 140000;
pub const __TVOS_14_1: u32 = 140100;
pub const __TVOS_14_2: u32 = 140200;
pub const __TVOS_14_3: u32 = 140300;
pub const __TVOS_14_5: u32 = 140500;
pub const __TVOS_14_6: u32 = 140600;
pub const __TVOS_14_7: u32 = 140700;
pub const __TVOS_15_0: u32 = 150000;
pub const __TVOS_15_1: u32 = 150100;
pub const __TVOS_15_2: u32 = 150200;
pub const __TVOS_15_3: u32 = 150300;
pub const __TVOS_15_4: u32 = 150400;
pub const __TVOS_15_5: u32 = 150500;
pub const __TVOS_15_6: u32 = 150600;
pub const __TVOS_16_0: u32 = 160000;
pub const __TVOS_16_1: u32 = 160100;
pub const __TVOS_16_2: u32 = 160200;
pub const __TVOS_16_3: u32 = 160300;
pub const __TVOS_16_4: u32 = 160400;
pub const __TVOS_16_5: u32 = 160500;
pub const __TVOS_16_6: u32 = 160600;
pub const __TVOS_17_0: u32 = 170000;
pub const __TVOS_17_1: u32 = 170100;
pub const __TVOS_17_2: u32 = 170200;
pub const __TVOS_17_3: u32 = 170300;
pub const __TVOS_17_4: u32 = 170400;
pub const __TVOS_17_5: u32 = 170500;
pub const __TVOS_18_0: u32 = 180000;
pub const __BRIDGEOS_2_0: u32 = 20000;
pub const __BRIDGEOS_3_0: u32 = 30000;
pub const __BRIDGEOS_3_1: u32 = 30100;
pub const __BRIDGEOS_3_4: u32 = 30400;
pub const __BRIDGEOS_4_0: u32 = 40000;
pub const __BRIDGEOS_4_1: u32 = 40100;
pub const __BRIDGEOS_5_0: u32 = 50000;
pub const __BRIDGEOS_5_1: u32 = 50100;
pub const __BRIDGEOS_5_3: u32 = 50300;
pub const __BRIDGEOS_6_0: u32 = 60000;
pub const __BRIDGEOS_6_2: u32 = 60200;
pub const __BRIDGEOS_6_4: u32 = 60400;
pub const __BRIDGEOS_6_5: u32 = 60500;
pub const __BRIDGEOS_6_6: u32 = 60600;
pub const __BRIDGEOS_7_0: u32 = 70000;
pub const __BRIDGEOS_7_1: u32 = 70100;
pub const __BRIDGEOS_7_2: u32 = 70200;
pub const __BRIDGEOS_7_3: u32 = 70300;
pub const __BRIDGEOS_7_4: u32 = 70400;
pub const __BRIDGEOS_7_6: u32 = 70600;
pub const __BRIDGEOS_8_0: u32 = 80000;
pub const __BRIDGEOS_8_1: u32 = 80100;
pub const __BRIDGEOS_8_2: u32 = 80200;
pub const __BRIDGEOS_8_3: u32 = 80300;
pub const __BRIDGEOS_8_4: u32 = 80400;
pub const __BRIDGEOS_8_5: u32 = 80500;
pub const __BRIDGEOS_9_0: u32 = 90000;
pub const __DRIVERKIT_19_0: u32 = 190000;
pub const __DRIVERKIT_20_0: u32 = 200000;
pub const __DRIVERKIT_21_0: u32 = 210000;
pub const __DRIVERKIT_22_0: u32 = 220000;
pub const __DRIVERKIT_22_4: u32 = 220400;
pub const __DRIVERKIT_22_5: u32 = 220500;
pub const __DRIVERKIT_22_6: u32 = 220600;
pub const __DRIVERKIT_23_0: u32 = 230000;
pub const __DRIVERKIT_23_1: u32 = 230100;
pub const __DRIVERKIT_23_2: u32 = 230200;
pub const __DRIVERKIT_23_3: u32 = 230300;
pub const __DRIVERKIT_23_4: u32 = 230400;
pub const __DRIVERKIT_23_5: u32 = 230500;
pub const __DRIVERKIT_24_0: u32 = 240000;
pub const __VISIONOS_1_0: u32 = 10000;
pub const __VISIONOS_1_1: u32 = 10100;
pub const __VISIONOS_1_2: u32 = 10200;
pub const __VISIONOS_2_0: u32 = 20000;
pub const MAC_OS_X_VERSION_10_0: u32 = 1000;
pub const MAC_OS_X_VERSION_10_1: u32 = 1010;
pub const MAC_OS_X_VERSION_10_2: u32 = 1020;
pub const MAC_OS_X_VERSION_10_3: u32 = 1030;
pub const MAC_OS_X_VERSION_10_4: u32 = 1040;
pub const MAC_OS_X_VERSION_10_5: u32 = 1050;
pub const MAC_OS_X_VERSION_10_6: u32 = 1060;
pub const MAC_OS_X_VERSION_10_7: u32 = 1070;
pub const MAC_OS_X_VERSION_10_8: u32 = 1080;
pub const MAC_OS_X_VERSION_10_9: u32 = 1090;
pub const MAC_OS_X_VERSION_10_10: u32 = 101000;
pub const MAC_OS_X_VERSION_10_10_2: u32 = 101002;
pub const MAC_OS_X_VERSION_10_10_3: u32 = 101003;
pub const MAC_OS_X_VERSION_10_11: u32 = 101100;
pub const MAC_OS_X_VERSION_10_11_2: u32 = 101102;
pub const MAC_OS_X_VERSION_10_11_3: u32 = 101103;
pub const MAC_OS_X_VERSION_10_11_4: u32 = 101104;
pub const MAC_OS_X_VERSION_10_12: u32 = 101200;
pub const MAC_OS_X_VERSION_10_12_1: u32 = 101201;
pub const MAC_OS_X_VERSION_10_12_2: u32 = 101202;
pub const MAC_OS_X_VERSION_10_12_4: u32 = 101204;
pub const MAC_OS_X_VERSION_10_13: u32 = 101300;
pub const MAC_OS_X_VERSION_10_13_1: u32 = 101301;
pub const MAC_OS_X_VERSION_10_13_2: u32 = 101302;
pub const MAC_OS_X_VERSION_10_13_4: u32 = 101304;
pub const MAC_OS_X_VERSION_10_14: u32 = 101400;
pub const MAC_OS_X_VERSION_10_14_1: u32 = 101401;
pub const MAC_OS_X_VERSION_10_14_4: u32 = 101404;
pub const MAC_OS_X_VERSION_10_14_5: u32 = 101405;
pub const MAC_OS_X_VERSION_10_14_6: u32 = 101406;
pub const MAC_OS_X_VERSION_10_15: u32 = 101500;
pub const MAC_OS_X_VERSION_10_15_1: u32 = 101501;
pub const MAC_OS_X_VERSION_10_15_4: u32 = 101504;
pub const MAC_OS_X_VERSION_10_16: u32 = 101600;
pub const MAC_OS_VERSION_11_0: u32 = 110000;
pub const MAC_OS_VERSION_11_1: u32 = 110100;
pub const MAC_OS_VERSION_11_3: u32 = 110300;
pub const MAC_OS_VERSION_11_4: u32 = 110400;
pub const MAC_OS_VERSION_11_5: u32 = 110500;
pub const MAC_OS_VERSION_11_6: u32 = 110600;
pub const MAC_OS_VERSION_12_0: u32 = 120000;
pub const MAC_OS_VERSION_12_1: u32 = 120100;
pub const MAC_OS_VERSION_12_2: u32 = 120200;
pub const MAC_OS_VERSION_12_3: u32 = 120300;
pub const MAC_OS_VERSION_12_4: u32 = 120400;
pub const MAC_OS_VERSION_12_5: u32 = 120500;
pub const MAC_OS_VERSION_12_6: u32 = 120600;
pub const MAC_OS_VERSION_12_7: u32 = 120700;
pub const MAC_OS_VERSION_13_0: u32 = 130000;
pub const MAC_OS_VERSION_13_1: u32 = 130100;
pub const MAC_OS_VERSION_13_2: u32 = 130200;
pub const MAC_OS_VERSION_13_3: u32 = 130300;
pub const MAC_OS_VERSION_13_4: u32 = 130400;
pub const MAC_OS_VERSION_13_5: u32 = 130500;
pub const MAC_OS_VERSION_13_6: u32 = 130600;
pub const MAC_OS_VERSION_14_0: u32 = 140000;
pub const MAC_OS_VERSION_14_1: u32 = 140100;
pub const MAC_OS_VERSION_14_2: u32 = 140200;
pub const MAC_OS_VERSION_14_3: u32 = 140300;
pub const MAC_OS_VERSION_14_4: u32 = 140400;
pub const MAC_OS_VERSION_14_5: u32 = 140500;
pub const MAC_OS_VERSION_15_0: u32 = 150000;
pub const __MAC_OS_X_VERSION_MAX_ALLOWED: u32 = 150000;
pub const __ENABLE_LEGACY_MAC_AVAILABILITY: u32 = 1;
pub const __has_safe_buffers: u32 = 1;
pub const __DARWIN_ONLY_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const __DARWIN_ONLY_VERS_1050: u32 = 1;
pub const __DARWIN_UNIX03: u32 = 1;
pub const __DARWIN_64_BIT_INO_T: u32 = 1;
pub const __DARWIN_VERS_1050: u32 = 1;
pub const __DARWIN_NON_CANCELABLE: u32 = 0;
pub const __DARWIN_SUF_EXTSN: &[u8; 14usize] = b"$DARWIN_EXTSN\0";
pub const __DARWIN_C_ANSI: u32 = 4096;
pub const __DARWIN_C_FULL: u32 = 900000;
pub const __DARWIN_C_LEVEL: u32 = 900000;
pub const __STDC_WANT_LIB_EXT1__: u32 = 1;
pub const __DARWIN_NO_LONG_LONG: u32 = 0;
pub const _DARWIN_FEATURE_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_64_BIT_INODE: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_VERS_1050: u32 = 1;
pub const _DARWIN_FEATURE_ONLY_UNIX_CONFORMANCE: u32 = 1;
pub const _DARWIN_FEATURE_UNIX_CONFORMANCE: u32 = 3;
pub const __has_ptrcheck: u32 = 0;
pub const USE_CLANG_TYPES: u32 = 0;
pub const __PTHREAD_SIZE__: u32 = 8176;
pub const __PTHREAD_ATTR_SIZE__: u32 = 56;
pub const __PTHREAD_MUTEXATTR_SIZE__: u32 = 8;
pub const __PTHREAD_MUTEX_SIZE__: u32 = 56;
pub const __PTHREAD_CONDATTR_SIZE__: u32 = 8;
pub const __PTHREAD_COND_SIZE__: u32 = 40;
pub const __PTHREAD_ONCE_SIZE__: u32 = 8;
pub const __PTHREAD_RWLOCK_SIZE__: u32 = 192;
pub const __PTHREAD_RWLOCKATTR_SIZE__: u32 = 16;
pub const __DARWIN_WCHAR_MIN: i32 = -2147483648;
pub const _FORTIFY_SOURCE: u32 = 2;
pub const __DARWIN_NSIG: u32 = 32;
pub const NSIG: u32 = 32;
pub const _ARM_SIGNAL_: u32 = 1;
pub const SIGHUP: u32 = 1;
pub const SIGINT: u32 = 2;
pub const SIGQUIT: u32 = 3;
pub const SIGILL: u32 = 4;
pub const SIGTRAP: u32 = 5;
pub const SIGABRT: u32 = 6;
pub const SIGIOT: u32 = 6;
pub const SIGEMT: u32 = 7;
pub const SIGFPE: u32 = 8;
pub const SIGKILL: u32 = 9;
pub const SIGBUS: u32 = 10;
pub const SIGSEGV: u32 = 11;
pub const SIGSYS: u32 = 12;
pub const SIGPIPE: u32 = 13;
pub const SIGALRM: u32 = 14;
pub const SIGTERM: u32 = 15;
pub const SIGURG: u32 = 16;
pub const SIGSTOP: u32 = 17;
pub const SIGTSTP: u32 = 18;
pub const SIGCONT: u32 = 19;
pub const SIGCHLD: u32 = 20;
pub const SIGTTIN: u32 = 21;
pub const SIGTTOU: u32 = 22;
pub const SIGIO: u32 = 23;
pub const SIGXCPU: u32 = 24;
pub const SIGXFSZ: u32 = 25;
pub const SIGVTALRM: u32 = 26;
pub const SIGPROF: u32 = 27;
pub const SIGWINCH: u32 = 28;
pub const SIGINFO: u32 = 29;
pub const SIGUSR1: u32 = 30;
pub const SIGUSR2: u32 = 31;
pub const __DARWIN_OPAQUE_ARM_THREAD_STATE64: u32 = 0;
pub const USE_CLANG_STDDEF: u32 = 0;
pub const SIGEV_NONE: u32 = 0;
pub const SIGEV_SIGNAL: u32 = 1;
pub const SIGEV_THREAD: u32 = 3;
pub const ILL_NOOP: u32 = 0;
pub const ILL_ILLOPC: u32 = 1;
pub const ILL_ILLTRP: u32 = 2;
pub const ILL_PRVOPC: u32 = 3;
pub const ILL_ILLOPN: u32 = 4;
pub const ILL_ILLADR: u32 = 5;
pub const ILL_PRVREG: u32 = 6;
pub const ILL_COPROC: u32 = 7;
pub const ILL_BADSTK: u32 = 8;
pub const FPE_NOOP: u32 = 0;
pub const FPE_FLTDIV: u32 = 1;
pub const FPE_FLTOVF: u32 = 2;
pub const FPE_FLTUND: u32 = 3;
pub const FPE_FLTRES: u32 = 4;
pub const FPE_FLTINV: u32 = 5;
pub const FPE_FLTSUB: u32 = 6;
pub const FPE_INTDIV: u32 = 7;
pub const FPE_INTOVF: u32 = 8;
pub const SEGV_NOOP: u32 = 0;
pub const SEGV_MAPERR: u32 = 1;
pub const SEGV_ACCERR: u32 = 2;
pub const BUS_NOOP: u32 = 0;
pub const BUS_ADRALN: u32 = 1;
pub const BUS_ADRERR: u32 = 2;
pub const BUS_OBJERR: u32 = 3;
pub const TRAP_BRKPT: u32 = 1;
pub const TRAP_TRACE: u32 = 2;
pub const CLD_NOOP: u32 = 0;
pub const CLD_EXITED: u32 = 1;
pub const CLD_KILLED: u32 = 2;
pub const CLD_DUMPED: u32 = 3;
pub const CLD_TRAPPED: u32 = 4;
pub const CLD_STOPPED: u32 = 5;
pub const CLD_CONTINUED: u32 = 6;
pub const POLL_IN: u32 = 1;
pub const POLL_OUT: u32 = 2;
pub const POLL_MSG: u32 = 3;
pub const POLL_ERR: u32 = 4;
pub const POLL_PRI: u32 = 5;
pub const POLL_HUP: u32 = 6;
pub const SA_ONSTACK: u32 = 1;
pub const SA_RESTART: u32 = 2;
pub const SA_RESETHAND: u32 = 4;
pub const SA_NOCLDSTOP: u32 = 8;
pub const SA_NODEFER: u32 = 16;
pub const SA_NOCLDWAIT: u32 = 32;
pub const SA_SIGINFO: u32 = 64;
pub const SA_USERTRAMP: u32 = 256;
pub const SA_64REGSET: u32 = 512;
pub const SA_USERSPACE_MASK: u32 = 127;
pub const SIG_BLOCK: u32 = 1;
pub const SIG_UNBLOCK: u32 = 2;
pub const SIG_SETMASK: u32 = 3;
pub const SI_USER: u32 = 65537;
pub const SI_QUEUE: u32 = 65538;
pub const SI_TIMER: u32 = 65539;
pub const SI_ASYNCIO: u32 = 65540;
pub const SI_MESGQ: u32 = 65541;
pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 4;
pub const MINSIGSTKSZ: u32 = 32768;
pub const SIGSTKSZ: u32 = 131072;
pub const SV_ONSTACK: u32 = 1;
pub const SV_INTERRUPT: u32 = 2;
pub const SV_RESETHAND: u32 = 4;
pub const SV_NODEFER: u32 = 16;
pub const SV_NOCLDSTOP: u32 = 8;
pub const SV_SIGINFO: u32 = 64;
pub const __WORDSIZE: u32 = 64;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const INT64_MAX: u64 = 9223372036854775807;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT64_MIN: i64 = -9223372036854775808;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const UINT64_MAX: i32 = -1;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST64_MIN: i64 = -9223372036854775808;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const INT_LEAST64_MAX: u64 = 9223372036854775807;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const UINT_LEAST64_MAX: i32 = -1;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i32 = -32768;
pub const INT_FAST32_MIN: i32 = -2147483648;
pub const INT_FAST64_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u32 = 32767;
pub const INT_FAST32_MAX: u32 = 2147483647;
pub const INT_FAST64_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: u32 = 65535;
pub const UINT_FAST32_MAX: u32 = 4294967295;
pub const UINT_FAST64_MAX: i32 = -1;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const UINTPTR_MAX: i32 = -1;
pub const SIZE_MAX: i32 = -1;
pub const RSIZE_MAX: i32 = -1;
pub const WINT_MIN: i32 = -2147483648;
pub const WINT_MAX: u32 = 2147483647;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const PRIO_PROCESS: u32 = 0;
pub const PRIO_PGRP: u32 = 1;
pub const PRIO_USER: u32 = 2;
pub const PRIO_DARWIN_THREAD: u32 = 3;
pub const PRIO_DARWIN_PROCESS: u32 = 4;
pub const PRIO_MIN: i32 = -20;
pub const PRIO_MAX: u32 = 20;
pub const PRIO_DARWIN_BG: u32 = 4096;
pub const PRIO_DARWIN_NONUI: u32 = 4097;
pub const RUSAGE_SELF: u32 = 0;
pub const RUSAGE_CHILDREN: i32 = -1;
pub const RUSAGE_INFO_V0: u32 = 0;
pub const RUSAGE_INFO_V1: u32 = 1;
pub const RUSAGE_INFO_V2: u32 = 2;
pub const RUSAGE_INFO_V3: u32 = 3;
pub const RUSAGE_INFO_V4: u32 = 4;
pub const RUSAGE_INFO_V5: u32 = 5;
pub const RUSAGE_INFO_V6: u32 = 6;
pub const RUSAGE_INFO_CURRENT: u32 = 6;
pub const RU_PROC_RUNS_RESLIDE: u32 = 1;
pub const RLIMIT_CPU: u32 = 0;
pub const RLIMIT_FSIZE: u32 = 1;
pub const RLIMIT_DATA: u32 = 2;
pub const RLIMIT_STACK: u32 = 3;
pub const RLIMIT_CORE: u32 = 4;
pub const RLIMIT_AS: u32 = 5;
pub const RLIMIT_RSS: u32 = 5;
pub const RLIMIT_MEMLOCK: u32 = 6;
pub const RLIMIT_NPROC: u32 = 7;
pub const RLIMIT_NOFILE: u32 = 8;
pub const RLIM_NLIMITS: u32 = 9;
pub const _RLIMIT_POSIX_FLAG: u32 = 4096;
pub const RLIMIT_WAKEUPS_MONITOR: u32 = 1;
pub const RLIMIT_CPU_USAGE_MONITOR: u32 = 2;
pub const RLIMIT_THREAD_CPULIMITS: u32 = 3;
pub const RLIMIT_FOOTPRINT_INTERVAL: u32 = 4;
pub const WAKEMON_ENABLE: u32 = 1;
pub const WAKEMON_DISABLE: u32 = 2;
pub const WAKEMON_GET_PARAMS: u32 = 4;
pub const WAKEMON_SET_DEFAULTS: u32 = 8;
pub const WAKEMON_MAKE_FATAL: u32 = 16;
pub const CPUMON_MAKE_FATAL: u32 = 4096;
pub const FOOTPRINT_INTERVAL_RESET: u32 = 1;
pub const IOPOL_TYPE_DISK: u32 = 0;
pub const IOPOL_TYPE_VFS_ATIME_UPDATES: u32 = 2;
pub const IOPOL_TYPE_VFS_MATERIALIZE_DATALESS_FILES: u32 = 3;
pub const IOPOL_TYPE_VFS_STATFS_NO_DATA_VOLUME: u32 = 4;
pub const IOPOL_TYPE_VFS_TRIGGER_RESOLVE: u32 = 5;
pub const IOPOL_TYPE_VFS_IGNORE_CONTENT_PROTECTION: u32 = 6;
pub const IOPOL_TYPE_VFS_IGNORE_PERMISSIONS: u32 = 7;
pub const IOPOL_TYPE_VFS_SKIP_MTIME_UPDATE: u32 = 8;
pub const IOPOL_TYPE_VFS_ALLOW_LOW_SPACE_WRITES: u32 = 9;
pub const IOPOL_TYPE_VFS_DISALLOW_RW_FOR_O_EVTONLY: u32 = 10;
pub const IOPOL_SCOPE_PROCESS: u32 = 0;
pub const IOPOL_SCOPE_THREAD: u32 = 1;
pub const IOPOL_SCOPE_DARWIN_BG: u32 = 2;
pub const IOPOL_DEFAULT: u32 = 0;
pub const IOPOL_IMPORTANT: u32 = 1;
pub const IOPOL_PASSIVE: u32 = 2;
pub const IOPOL_THROTTLE: u32 = 3;
pub const IOPOL_UTILITY: u32 = 4;
pub const IOPOL_STANDARD: u32 = 5;
pub const IOPOL_APPLICATION: u32 = 5;
pub const IOPOL_NORMAL: u32 = 1;
pub const IOPOL_ATIME_UPDATES_DEFAULT: u32 = 0;
pub const IOPOL_ATIME_UPDATES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_DEFAULT: u32 = 0;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_OFF: u32 = 1;
pub const IOPOL_MATERIALIZE_DATALESS_FILES_ON: u32 = 2;
pub const IOPOL_VFS_STATFS_NO_DATA_VOLUME_DEFAULT: u32 = 0;
pub const IOPOL_VFS_STATFS_FORCE_NO_DATA_VOLUME: u32 = 1;
pub const IOPOL_VFS_TRIGGER_RESOLVE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_TRIGGER_RESOLVE_OFF: u32 = 1;
pub const IOPOL_VFS_CONTENT_PROTECTION_DEFAULT: u32 = 0;
pub const IOPOL_VFS_CONTENT_PROTECTION_IGNORE: u32 = 1;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_OFF: u32 = 0;
pub const IOPOL_VFS_IGNORE_PERMISSIONS_ON: u32 = 1;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_OFF: u32 = 0;
pub const IOPOL_VFS_SKIP_MTIME_UPDATE_ON: u32 = 1;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_OFF: u32 = 0;
pub const IOPOL_VFS_ALLOW_LOW_SPACE_WRITES_ON: u32 = 1;
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_DEFAULT: u32 = 0;
pub const IOPOL_VFS_DISALLOW_RW_FOR_O_EVTONLY_ON: u32 = 1;
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_DEFAULT: u32 = 0;
pub const IOPOL_VFS_NOCACHE_WRITE_FS_BLKSIZE_ON: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WCOREFLAG: u32 = 128;
pub const _WSTOPPED: u32 = 127;
pub const WEXITED: u32 = 4;
pub const WSTOPPED: u32 = 8;
pub const WCONTINUED: u32 = 16;
pub const WNOWAIT: u32 = 32;
pub const WAIT_ANY: i32 = -1;
pub const WAIT_MYPGRP: u32 = 0;
pub const _QUAD_HIGHWORD: u32 = 1;
pub const _QUAD_LOWWORD: u32 = 0;
pub const __DARWIN_LITTLE_ENDIAN: u32 = 1234;
pub const __DARWIN_BIG_ENDIAN: u32 = 4321;
pub const __DARWIN_PDP_ENDIAN: u32 = 3412;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const __DARWIN_BYTE_ORDER: u32 = 1234;
pub const BYTE_ORDER: u32 = 1234;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const RAND_MAX: u32 = 2147483647;
pub const USE_CLANG_STDARG: u32 = 0;
pub const RENAME_SECLUDE: u32 = 1;
pub const RENAME_SWAP: u32 = 2;
pub const RENAME_EXCL: u32 = 4;
pub const RENAME_RESERVED1: u32 = 8;
pub const RENAME_NOFOLLOW_ANY: u32 = 16;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const SEEK_HOLE: u32 = 3;
pub const SEEK_DATA: u32 = 4;
pub const __SLBF: u32 = 1;
pub const __SNBF: u32 = 2;
pub const __SRD: u32 = 4;
pub const __SWR: u32 = 8;
pub const __SRW: u32 = 16;
pub const __SEOF: u32 = 32;
pub const __SERR: u32 = 64;
pub const __SMBF: u32 = 128;
pub const __SAPP: u32 = 256;
pub const __SSTR: u32 = 512;
pub const __SOPT: u32 = 1024;
pub const __SNPT: u32 = 2048;
pub const __SOFF: u32 = 4096;
pub const __SMOD: u32 = 8192;
pub const __SALC: u32 = 16384;
pub const __SIGN: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 1024;
pub const EOF: i32 = -1;
pub const FOPEN_MAX: u32 = 20;
pub const FILENAME_MAX: u32 = 1024;
pub const P_tmpdir: &[u8; 10usize] = b"/var/tmp/\0";
pub const L_tmpnam: u32 = 1024;
pub const TMP_MAX: u32 = 308915776;
pub const L_ctermid: u32 = 1024;
pub const _USE_FORTIFY_LEVEL: u32 = 2;
pub const __DARWIN_CLK_TCK: u32 = 100;
pub const USE_CLANG_LIMITS: u32 = 0;
pub const MB_LEN_MAX: u32 = 6;
pub const CLK_TCK: u32 = 100;
pub const CHAR_BIT: u32 = 8;
pub const SCHAR_MAX: u32 = 127;
pub const SCHAR_MIN: i32 = -128;
pub const UCHAR_MAX: u32 = 255;
pub const CHAR_MAX: u32 = 127;
pub const CHAR_MIN: i32 = -128;
pub const USHRT_MAX: u32 = 65535;
pub const SHRT_MAX: u32 = 32767;
pub const SHRT_MIN: i32 = -32768;
pub const UINT_MAX: u32 = 4294967295;
pub const INT_MAX: u32 = 2147483647;
pub const INT_MIN: i32 = -2147483648;
pub const ULONG_MAX: i32 = -1;
pub const LONG_MAX: u64 = 9223372036854775807;
pub const LONG_MIN: i64 = -9223372036854775808;
pub const ULLONG_MAX: i32 = -1;
pub const LLONG_MAX: u64 = 9223372036854775807;
pub const LLONG_MIN: i64 = -9223372036854775808;
pub const LONG_BIT: u32 = 64;
pub const SSIZE_MAX: u64 = 9223372036854775807;
pub const WORD_BIT: u32 = 32;
pub const SIZE_T_MAX: i32 = -1;
pub const UQUAD_MAX: i32 = -1;
pub const QUAD_MAX: u64 = 9223372036854775807;
pub const QUAD_MIN: i64 = -9223372036854775808;
pub const ARG_MAX: u32 = 1048576;
pub const CHILD_MAX: u32 = 266;
pub const GID_MAX: u32 = 2147483647;
pub const LINK_MAX: u32 = 32767;
pub const MAX_CANON: u32 = 1024;
pub const MAX_INPUT: u32 = 1024;
pub const NAME_MAX: u32 = 255;
pub const NGROUPS_MAX: u32 = 16;
pub const UID_MAX: u32 = 2147483647;
pub const OPEN_MAX: u32 = 10240;
pub const PATH_MAX: u32 = 1024;
pub const PIPE_BUF: u32 = 512;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const CHARCLASS_NAME_MAX: u32 = 14;
pub const COLL_WEIGHTS_MAX: u32 = 2;
pub const EQUIV_CLASS_MAX: u32 = 2;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 255;
pub const NZERO: u32 = 20;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_EQUIV_CLASS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_KEYS_MAX: u32 = 512;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_SS_REPL_MAX: u32 = 4;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TRACE_EVENT_NAME_MAX: u32 = 30;
pub const _POSIX_TRACE_NAME_MAX: u32 = 8;
pub const _POSIX_TRACE_SYS_MAX: u32 = 8;
pub const _POSIX_TRACE_USER_EVENT_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const OFF_MIN: i64 = -9223372036854775808;
pub const OFF_MAX: u64 = 9223372036854775807;
pub const PASS_MAX: u32 = 128;
pub const NL_ARGMAX: u32 = 9;
pub const NL_LANGMAX: u32 = 14;
pub const NL_MSGMAX: u32 = 32767;
pub const NL_NMAX: u32 = 1;
pub const NL_SETMAX: u32 = 255;
pub const NL_TEXTMAX: u32 = 2048;
pub const _XOPEN_IOV_MAX: u32 = 16;
pub const IOV_MAX: u32 = 1024;
pub const _XOPEN_NAME_MAX: u32 = 255;
pub const _XOPEN_PATH_MAX: u32 = 1024;
pub const I64FORMAT: &[u8; 4usize] = b"lld\0";
pub const I64uFORMAT: &[u8; 4usize] = b"llu\0";
pub const SECP256K1_WIDEMUL_INT128: u32 = 1;
pub const SECP256K1_INT128_NATIVE: u32 = 1;
pub const SECP256K1_GE_X_MAGNITUDE_MAX: u32 = 4;
pub const SECP256K1_GE_Y_MAGNITUDE_MAX: u32 = 3;
pub const SECP256K1_GEJ_X_MAGNITUDE_MAX: u32 = 4;
pub const SECP256K1_GEJ_Y_MAGNITUDE_MAX: u32 = 4;
pub const SECP256K1_GEJ_Z_MAGNITUDE_MAX: u32 = 1;
pub const SECP256K1_CHECKMEM_ENABLED: u32 = 0;
pub const JACOBI64_ITERATIONS: u32 = 25;
pub const SECP256K1_N_C_2: u32 = 1;
pub const SECP256K1_B: u32 = 7;
pub const __HAS_FIXED_CHK_PROTOTYPES: u32 = 1;
pub const WINDOW_G: u32 = 15;
pub const WINDOW_A: u32 = 5;
pub const WNAF_BITS: u32 = 128;
pub const PIPPENGER_SCRATCH_OBJECTS: u32 = 6;
pub const STRAUSS_SCRATCH_OBJECTS: u32 = 5;
pub const PIPPENGER_MAX_BUCKET_WINDOW: u32 = 12;
pub const ECMULT_PIPPENGER_THRESHOLD: u32 = 88;
pub const ECMULT_MAX_POINTS_PER_BATCH: u32 = 5000000;
pub const ECMULT_CONST_GROUP_SIZE: u32 = 5;
pub const ECMULT_CONST_TABLE_SIZE: u32 = 16;
pub const ECMULT_CONST_GROUPS: u32 = 26;
pub const ECMULT_CONST_BITS: u32 = 130;
pub const COMB_RANGE: u32 = 256;
pub const COMB_BLOCKS: u32 = 11;
pub const COMB_TEETH: u32 = 6;
pub const COMB_POINTS: u32 = 32;
pub type wchar_t = ::std::os::raw::c_int;
pub type max_align_t = f64;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_context_struct {
    _unused: [u8; 0],
}
#[doc = " Opaque data structure that holds context information\n\n  The primary purpose of context objects is to store randomization data for\n  enhanced protection against side-channel leakage. This protection is only\n  effective if the context is randomized after its creation. See\n  secp256k1_context_create for creation of contexts and\n  secp256k1_context_randomize for randomization.\n\n  A secondary purpose of context objects is to store pointers to callback\n  functions that the library will call when certain error states arise. See\n  secp256k1_context_set_error_callback as well as\n  secp256k1_context_set_illegal_callback for details. Future library versions\n  may use context objects for additional purposes.\n\n  A constructed context can safely be used from multiple threads\n  simultaneously, but API calls that take a non-const pointer to a context\n  need exclusive access to it. In particular this is the case for\n  secp256k1_context_destroy, secp256k1_context_preallocated_destroy,\n  and secp256k1_context_randomize.\n\n  Regarding randomization, either do it once at creation time (in which case\n  you do not need any locking for the other calls), or use a read-write lock."]
pub type secp256k1_context = secp256k1_context_struct;
#[doc = " Opaque data structure that holds rewritable \"scratch space\"\n\n  The purpose of this structure is to replace dynamic memory allocations,\n  because we target architectures where this may not be available. It is\n  essentially a resizable (within specified parameters) block of bytes,\n  which is initially created either by memory allocation or TODO as a pointer\n  into some fixed rewritable space.\n\n  Unlike the context object, this cannot safely be shared between threads\n  without additional synchronization logic."]
pub type secp256k1_scratch_space = secp256k1_scratch_space_struct;
#[doc = " Opaque data structure that holds a parsed and valid public key.\n\n  The exact representation of data inside is implementation defined and not\n  guaranteed to be portable between different platforms or versions. It is\n  however guaranteed to be 64 bytes in size, and can be safely copied/moved.\n  If you need to convert to a format suitable for storage or transmission,\n  use secp256k1_ec_pubkey_serialize and secp256k1_ec_pubkey_parse. To\n  compare keys, use secp256k1_ec_pubkey_cmp."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_pubkey {
    pub data: [::std::os::raw::c_uchar; 64usize],
}
#[test]
fn bindgen_test_layout_secp256k1_pubkey() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_pubkey> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_pubkey>(),
        64usize,
        concat!("Size of: ", stringify!(secp256k1_pubkey))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_pubkey>(),
        1usize,
        concat!("Alignment of ", stringify!(secp256k1_pubkey))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_pubkey),
            "::",
            stringify!(data)
        )
    );
}
#[doc = " Opaque data structured that holds a parsed ECDSA signature.\n\n  The exact representation of data inside is implementation defined and not\n  guaranteed to be portable between different platforms or versions. It is\n  however guaranteed to be 64 bytes in size, and can be safely copied/moved.\n  If you need to convert to a format suitable for storage, transmission, or\n  comparison, use the secp256k1_ecdsa_signature_serialize_* and\n  secp256k1_ecdsa_signature_parse_* functions."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_ecdsa_signature {
    pub data: [::std::os::raw::c_uchar; 64usize],
}
#[test]
fn bindgen_test_layout_secp256k1_ecdsa_signature() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_ecdsa_signature> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_ecdsa_signature>(),
        64usize,
        concat!("Size of: ", stringify!(secp256k1_ecdsa_signature))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_ecdsa_signature>(),
        1usize,
        concat!("Alignment of ", stringify!(secp256k1_ecdsa_signature))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ecdsa_signature),
            "::",
            stringify!(data)
        )
    );
}
#[doc = " A pointer to a function to deterministically generate a nonce.\n\n Returns: 1 if a nonce was successfully generated. 0 will cause signing to fail.\n Out:     nonce32:   pointer to a 32-byte array to be filled by the function.\n In:      msg32:     the 32-byte message hash being verified (will not be NULL)\n          key32:     pointer to a 32-byte secret key (will not be NULL)\n          algo16:    pointer to a 16-byte array describing the signature\n                     algorithm (will be NULL for ECDSA for compatibility).\n          data:      Arbitrary data pointer that is passed through.\n          attempt:   how many iterations we have tried to find a nonce.\n                     This will almost always be 0, but different attempt values\n                     are required to result in a different nonce.\n\n Except for test cases, this function should compute some cryptographic hash of\n the message, the algorithm, the key and the attempt."]
pub type secp256k1_nonce_function = ::std::option::Option<
    unsafe extern "C" fn(
        nonce32: *mut ::std::os::raw::c_uchar,
        msg32: *const ::std::os::raw::c_uchar,
        key32: *const ::std::os::raw::c_uchar,
        algo16: *const ::std::os::raw::c_uchar,
        data: *mut ::std::os::raw::c_void,
        attempt: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub static mut s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_static:
        *const secp256k1_context;
}
extern "C" {
    pub static mut s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_no_precomp:
        *const secp256k1_context;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_selftest();
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_create(
        flags: ::std::os::raw::c_uint,
    ) -> *mut secp256k1_context;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_clone(
        ctx: *const secp256k1_context,
    ) -> *mut secp256k1_context;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_destroy(
        ctx: *mut secp256k1_context,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_set_illegal_callback(
        ctx: *mut secp256k1_context,
        fun: ::std::option::Option<
            unsafe extern "C" fn(
                message: *const ::std::os::raw::c_char,
                data: *mut ::std::os::raw::c_void,
            ),
        >,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_set_error_callback(
        ctx: *mut secp256k1_context,
        fun: ::std::option::Option<
            unsafe extern "C" fn(
                message: *const ::std::os::raw::c_char,
                data: *mut ::std::os::raw::c_void,
            ),
        >,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scratch_space_create(
        ctx: *const secp256k1_context,
        size: usize,
    ) -> *mut secp256k1_scratch_space;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scratch_space_destroy(
        ctx: *const secp256k1_context,
        scratch: *mut secp256k1_scratch_space,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_parse(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        input: *const ::std::os::raw::c_uchar,
        inputlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_serialize(
        ctx: *const secp256k1_context,
        output: *mut ::std::os::raw::c_uchar,
        outputlen: *mut usize,
        pubkey: *const secp256k1_pubkey,
        flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_cmp(
        ctx: *const secp256k1_context,
        pubkey1: *const secp256k1_pubkey,
        pubkey2: *const secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_sort(
        ctx: *const secp256k1_context,
        pubkeys: *mut *const secp256k1_pubkey,
        n_pubkeys: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecdsa_signature_parse_compact(
        ctx: *const secp256k1_context,
        sig: *mut secp256k1_ecdsa_signature,
        input64: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecdsa_signature_parse_der(
        ctx: *const secp256k1_context,
        sig: *mut secp256k1_ecdsa_signature,
        input: *const ::std::os::raw::c_uchar,
        inputlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecdsa_signature_serialize_der(
        ctx: *const secp256k1_context,
        output: *mut ::std::os::raw::c_uchar,
        outputlen: *mut usize,
        sig: *const secp256k1_ecdsa_signature,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecdsa_signature_serialize_compact(
        ctx: *const secp256k1_context,
        output64: *mut ::std::os::raw::c_uchar,
        sig: *const secp256k1_ecdsa_signature,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecdsa_verify(
        ctx: *const secp256k1_context,
        sig: *const secp256k1_ecdsa_signature,
        msghash32: *const ::std::os::raw::c_uchar,
        pubkey: *const secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecdsa_signature_normalize(
        ctx: *const secp256k1_context,
        sigout: *mut secp256k1_ecdsa_signature,
        sigin: *const secp256k1_ecdsa_signature,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_nonce_function_rfc6979:
        secp256k1_nonce_function;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_nonce_function_default:
        secp256k1_nonce_function;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecdsa_sign(
        ctx: *const secp256k1_context,
        sig: *mut secp256k1_ecdsa_signature,
        msghash32: *const ::std::os::raw::c_uchar,
        seckey: *const ::std::os::raw::c_uchar,
        noncefp: secp256k1_nonce_function,
        ndata: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_seckey_verify(
        ctx: *const secp256k1_context,
        seckey: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_create(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        seckey: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_seckey_negate(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_privkey_negate(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_negate(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_seckey_tweak_add(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_privkey_tweak_add(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_tweak_add(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_seckey_tweak_mul(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_privkey_tweak_mul(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_tweak_mul(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_context_randomize(
        ctx: *mut secp256k1_context,
        seed32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ec_pubkey_combine(
        ctx: *const secp256k1_context,
        out: *mut secp256k1_pubkey,
        ins: *const *const secp256k1_pubkey,
        n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_tagged_sha256(
        ctx: *const secp256k1_context,
        hash32: *mut ::std::os::raw::c_uchar,
        tag: *const ::std::os::raw::c_uchar,
        taglen: usize,
        msg: *const ::std::os::raw::c_uchar,
        msglen: usize,
    ) -> ::std::os::raw::c_int;
}
#[doc = " Opaque data structure that holds a parsed and valid \"x-only\" public key.\n  An x-only pubkey encodes a point whose Y coordinate is even. It is\n  serialized using only its X coordinate (32 bytes). See BIP-340 for more\n  information about x-only pubkeys.\n\n  The exact representation of data inside is implementation defined and not\n  guaranteed to be portable between different platforms or versions. It is\n  however guaranteed to be 64 bytes in size, and can be safely copied/moved.\n  If you need to convert to a format suitable for storage, transmission, use\n  use secp256k1_xonly_pubkey_serialize and secp256k1_xonly_pubkey_parse. To\n  compare keys, use secp256k1_xonly_pubkey_cmp."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_xonly_pubkey {
    pub data: [::std::os::raw::c_uchar; 64usize],
}
#[test]
fn bindgen_test_layout_secp256k1_xonly_pubkey() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_xonly_pubkey> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_xonly_pubkey>(),
        64usize,
        concat!("Size of: ", stringify!(secp256k1_xonly_pubkey))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_xonly_pubkey>(),
        1usize,
        concat!("Alignment of ", stringify!(secp256k1_xonly_pubkey))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_xonly_pubkey),
            "::",
            stringify!(data)
        )
    );
}
#[doc = " Opaque data structure that holds a keypair consisting of a secret and a\n  public key.\n\n  The exact representation of data inside is implementation defined and not\n  guaranteed to be portable between different platforms or versions. It is\n  however guaranteed to be 96 bytes in size, and can be safely copied/moved."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_keypair {
    pub data: [::std::os::raw::c_uchar; 96usize],
}
#[test]
fn bindgen_test_layout_secp256k1_keypair() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_keypair> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_keypair>(),
        96usize,
        concat!("Size of: ", stringify!(secp256k1_keypair))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_keypair>(),
        1usize,
        concat!("Alignment of ", stringify!(secp256k1_keypair))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_keypair),
            "::",
            stringify!(data)
        )
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_xonly_pubkey_parse(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_xonly_pubkey,
        input32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_xonly_pubkey_serialize(
        ctx: *const secp256k1_context,
        output32: *mut ::std::os::raw::c_uchar,
        pubkey: *const secp256k1_xonly_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_xonly_pubkey_cmp(
        ctx: *const secp256k1_context,
        pk1: *const secp256k1_xonly_pubkey,
        pk2: *const secp256k1_xonly_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_xonly_pubkey_from_pubkey(
        ctx: *const secp256k1_context,
        xonly_pubkey: *mut secp256k1_xonly_pubkey,
        pk_parity: *mut ::std::os::raw::c_int,
        pubkey: *const secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_xonly_pubkey_tweak_add(
        ctx: *const secp256k1_context,
        output_pubkey: *mut secp256k1_pubkey,
        internal_pubkey: *const secp256k1_xonly_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_xonly_pubkey_tweak_add_check(
        ctx: *const secp256k1_context,
        tweaked_pubkey32: *const ::std::os::raw::c_uchar,
        tweaked_pk_parity: ::std::os::raw::c_int,
        internal_pubkey: *const secp256k1_xonly_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_keypair_create(
        ctx: *const secp256k1_context,
        keypair: *mut secp256k1_keypair,
        seckey: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_keypair_sec(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        keypair: *const secp256k1_keypair,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_keypair_pub(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        keypair: *const secp256k1_keypair,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_keypair_xonly_pub(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_xonly_pubkey,
        pk_parity: *mut ::std::os::raw::c_int,
        keypair: *const secp256k1_keypair,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_keypair_xonly_tweak_add(
        ctx: *const secp256k1_context,
        keypair: *mut secp256k1_keypair,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
#[doc = " A pointer to a function to deterministically generate a nonce.\n\n  Same as secp256k1_nonce function with the exception of accepting an\n  additional pubkey argument and not requiring an attempt argument. The pubkey\n  argument can protect signature schemes with key-prefixed challenge hash\n  inputs against reusing the nonce when signing with the wrong precomputed\n  pubkey.\n\n  Returns: 1 if a nonce was successfully generated. 0 will cause signing to\n           return an error.\n  Out:  nonce32: pointer to a 32-byte array to be filled by the function\n  In:       msg: the message being verified. Is NULL if and only if msglen\n                 is 0.\n         msglen: the length of the message\n          key32: pointer to a 32-byte secret key (will not be NULL)\n     xonly_pk32: the 32-byte serialized xonly pubkey corresponding to key32\n                 (will not be NULL)\n           algo: pointer to an array describing the signature\n                 algorithm (will not be NULL)\n        algolen: the length of the algo array\n           data: arbitrary data pointer that is passed through\n\n  Except for test cases, this function should compute some cryptographic hash of\n  the message, the key, the pubkey, the algorithm description, and data."]
pub type secp256k1_nonce_function_hardened = ::std::option::Option<
    unsafe extern "C" fn(
        nonce32: *mut ::std::os::raw::c_uchar,
        msg: *const ::std::os::raw::c_uchar,
        msglen: usize,
        key32: *const ::std::os::raw::c_uchar,
        xonly_pk32: *const ::std::os::raw::c_uchar,
        algo: *const ::std::os::raw::c_uchar,
        algolen: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_nonce_function_bip340:
        secp256k1_nonce_function_hardened;
}
#[doc = " Data structure that contains additional arguments for schnorrsig_sign_custom.\n\n  A schnorrsig_extraparams structure object can be initialized correctly by\n  setting it to SECP256K1_SCHNORRSIG_EXTRAPARAMS_INIT.\n\n  Members:\n      magic: set to SECP256K1_SCHNORRSIG_EXTRAPARAMS_MAGIC at initialization\n             and has no other function than making sure the object is\n             initialized.\n    noncefp: pointer to a nonce generation function. If NULL,\n             secp256k1_nonce_function_bip340 is used\n      ndata: pointer to arbitrary data used by the nonce generation function\n             (can be NULL). If it is non-NULL and\n             secp256k1_nonce_function_bip340 is used, then ndata must be a\n             pointer to 32-byte auxiliary randomness as per BIP-340."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_schnorrsig_extraparams {
    pub magic: [::std::os::raw::c_uchar; 4usize],
    pub noncefp: secp256k1_nonce_function_hardened,
    pub ndata: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_secp256k1_schnorrsig_extraparams() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_schnorrsig_extraparams> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_schnorrsig_extraparams>(),
        24usize,
        concat!("Size of: ", stringify!(secp256k1_schnorrsig_extraparams))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_schnorrsig_extraparams>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(secp256k1_schnorrsig_extraparams)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).magic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_schnorrsig_extraparams),
            "::",
            stringify!(magic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).noncefp) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_schnorrsig_extraparams),
            "::",
            stringify!(noncefp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ndata) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_schnorrsig_extraparams),
            "::",
            stringify!(ndata)
        )
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_schnorrsig_sign32(
        ctx: *const secp256k1_context,
        sig64: *mut ::std::os::raw::c_uchar,
        msg32: *const ::std::os::raw::c_uchar,
        keypair: *const secp256k1_keypair,
        aux_rand32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_schnorrsig_sign(
        ctx: *const secp256k1_context,
        sig64: *mut ::std::os::raw::c_uchar,
        msg32: *const ::std::os::raw::c_uchar,
        keypair: *const secp256k1_keypair,
        aux_rand32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_schnorrsig_sign_custom(
        ctx: *const secp256k1_context,
        sig64: *mut ::std::os::raw::c_uchar,
        msg: *const ::std::os::raw::c_uchar,
        msglen: usize,
        keypair: *const secp256k1_keypair,
        extraparams: *mut secp256k1_schnorrsig_extraparams,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_schnorrsig_verify(
        ctx: *const secp256k1_context,
        sig64: *const ::std::os::raw::c_uchar,
        msg: *const ::std::os::raw::c_uchar,
        msglen: usize,
        pubkey: *const secp256k1_xonly_pubkey,
    ) -> ::std::os::raw::c_int;
}
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_longlong;
pub type __uint64_t = ::std::os::raw::c_ulonglong;
pub type __darwin_intptr_t = ::std::os::raw::c_long;
pub type __darwin_natural_t = ::std::os::raw::c_uint;
pub type __darwin_ct_rune_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t {
    pub __mbstate8: [::std::os::raw::c_char; 128usize],
    pub _mbstateL: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    const UNINIT: ::std::mem::MaybeUninit<__mbstate_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        128usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__mbstate8) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__mbstate8)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._mbstateL) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(_mbstateL)
        )
    );
}
pub type __darwin_mbstate_t = __mbstate_t;
pub type __darwin_ptrdiff_t = ::std::os::raw::c_long;
pub type __darwin_size_t = ::std::os::raw::c_ulong;
pub type __darwin_va_list = __builtin_va_list;
pub type __darwin_wchar_t = ::std::os::raw::c_int;
pub type __darwin_rune_t = __darwin_wchar_t;
pub type __darwin_wint_t = ::std::os::raw::c_int;
pub type __darwin_clock_t = ::std::os::raw::c_ulong;
pub type __darwin_socklen_t = __uint32_t;
pub type __darwin_ssize_t = ::std::os::raw::c_long;
pub type __darwin_time_t = ::std::os::raw::c_long;
pub type __darwin_blkcnt_t = __int64_t;
pub type __darwin_blksize_t = __int32_t;
pub type __darwin_dev_t = __int32_t;
pub type __darwin_fsblkcnt_t = ::std::os::raw::c_uint;
pub type __darwin_fsfilcnt_t = ::std::os::raw::c_uint;
pub type __darwin_gid_t = __uint32_t;
pub type __darwin_id_t = __uint32_t;
pub type __darwin_ino64_t = __uint64_t;
pub type __darwin_ino_t = __darwin_ino64_t;
pub type __darwin_mach_port_name_t = __darwin_natural_t;
pub type __darwin_mach_port_t = __darwin_mach_port_name_t;
pub type __darwin_mode_t = __uint16_t;
pub type __darwin_off_t = __int64_t;
pub type __darwin_pid_t = __int32_t;
pub type __darwin_sigset_t = __uint32_t;
pub type __darwin_suseconds_t = __int32_t;
pub type __darwin_uid_t = __uint32_t;
pub type __darwin_useconds_t = __uint32_t;
pub type __darwin_uuid_t = [::std::os::raw::c_uchar; 16usize];
pub type __darwin_uuid_string_t = [::std::os::raw::c_char; 37usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_pthread_handler_rec {
    pub __routine: ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub __arg: *mut ::std::os::raw::c_void,
    pub __next: *mut __darwin_pthread_handler_rec,
}
#[test]
fn bindgen_test_layout___darwin_pthread_handler_rec() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_pthread_handler_rec> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_pthread_handler_rec>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_pthread_handler_rec>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_pthread_handler_rec))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__routine) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__routine)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__arg) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__arg)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__next) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_pthread_handler_rec),
            "::",
            stringify!(__next)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_attr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_attr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_attr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_attr_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_attr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_attr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_cond_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 40usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_cond_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_cond_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_cond_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_cond_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_condattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_condattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_condattr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_condattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_condattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_condattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_condattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutex_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 56usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutex_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_mutex_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutex_t>(),
        64usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutex_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutex_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_mutexattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_mutexattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_mutexattr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_mutexattr_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_mutexattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_mutexattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_once_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 8usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_once_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_once_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_once_t>(),
        16usize,
        concat!("Size of: ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_once_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_once_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_once_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlock_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 192usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlock_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_rwlock_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlock_t>(),
        200usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlock_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_rwlockattr_t {
    pub __sig: ::std::os::raw::c_long,
    pub __opaque: [::std::os::raw::c_char; 16usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_rwlockattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_rwlockattr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_rwlockattr_t>(),
        24usize,
        concat!("Size of: ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_rwlockattr_t),
            "::",
            stringify!(__opaque)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _opaque_pthread_t {
    pub __sig: ::std::os::raw::c_long,
    pub __cleanup_stack: *mut __darwin_pthread_handler_rec,
    pub __opaque: [::std::os::raw::c_char; 8176usize],
}
#[test]
fn bindgen_test_layout__opaque_pthread_t() {
    const UNINIT: ::std::mem::MaybeUninit<_opaque_pthread_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_opaque_pthread_t>(),
        8192usize,
        concat!("Size of: ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_opaque_pthread_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_opaque_pthread_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sig) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__sig)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__cleanup_stack) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__cleanup_stack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__opaque) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_opaque_pthread_t),
            "::",
            stringify!(__opaque)
        )
    );
}
pub type __darwin_pthread_attr_t = _opaque_pthread_attr_t;
pub type __darwin_pthread_cond_t = _opaque_pthread_cond_t;
pub type __darwin_pthread_condattr_t = _opaque_pthread_condattr_t;
pub type __darwin_pthread_key_t = ::std::os::raw::c_ulong;
pub type __darwin_pthread_mutex_t = _opaque_pthread_mutex_t;
pub type __darwin_pthread_mutexattr_t = _opaque_pthread_mutexattr_t;
pub type __darwin_pthread_once_t = _opaque_pthread_once_t;
pub type __darwin_pthread_rwlock_t = _opaque_pthread_rwlock_t;
pub type __darwin_pthread_rwlockattr_t = _opaque_pthread_rwlockattr_t;
pub type __darwin_pthread_t = *mut _opaque_pthread_t;
pub type __darwin_nl_item = ::std::os::raw::c_int;
pub type __darwin_wctrans_t = ::std::os::raw::c_int;
pub type __darwin_wctype_t = __uint32_t;
pub const idtype_t_P_ALL: idtype_t = 0;
pub const idtype_t_P_PID: idtype_t = 1;
pub const idtype_t_P_PGID: idtype_t = 2;
pub type idtype_t = ::std::os::raw::c_uint;
pub type pid_t = __darwin_pid_t;
pub type id_t = __darwin_id_t;
pub type sig_atomic_t = ::std::os::raw::c_int;
pub type u_int8_t = ::std::os::raw::c_uchar;
pub type u_int16_t = ::std::os::raw::c_ushort;
pub type u_int32_t = ::std::os::raw::c_uint;
pub type u_int64_t = ::std::os::raw::c_ulonglong;
pub type register_t = i64;
pub type user_addr_t = u_int64_t;
pub type user_size_t = u_int64_t;
pub type user_ssize_t = i64;
pub type user_long_t = i64;
pub type user_ulong_t = u_int64_t;
pub type user_time_t = i64;
pub type user_off_t = i64;
pub type syscall_arg_t = u_int64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state {
    pub __exception: __uint32_t,
    pub __fsr: __uint32_t,
    pub __far: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_exception_state() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_exception_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_exception_state>(),
        12usize,
        concat!("Size of: ", stringify!(__darwin_arm_exception_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_exception_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_arm_exception_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__exception) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state),
            "::",
            stringify!(__exception)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fsr) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state),
            "::",
            stringify!(__fsr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__far) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state),
            "::",
            stringify!(__far)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state64 {
    pub __far: __uint64_t,
    pub __esr: __uint32_t,
    pub __exception: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_exception_state64() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_exception_state64> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_exception_state64>(),
        16usize,
        concat!("Size of: ", stringify!(__darwin_arm_exception_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_exception_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_exception_state64))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__far) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state64),
            "::",
            stringify!(__far)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__esr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state64),
            "::",
            stringify!(__esr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__exception) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state64),
            "::",
            stringify!(__exception)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_exception_state64_v2 {
    pub __far: __uint64_t,
    pub __esr: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_exception_state64_v2() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_exception_state64_v2> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_exception_state64_v2>(),
        16usize,
        concat!("Size of: ", stringify!(__darwin_arm_exception_state64_v2))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_exception_state64_v2>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(__darwin_arm_exception_state64_v2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__far) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state64_v2),
            "::",
            stringify!(__far)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__esr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_exception_state64_v2),
            "::",
            stringify!(__esr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state {
    pub __r: [__uint32_t; 13usize],
    pub __sp: __uint32_t,
    pub __lr: __uint32_t,
    pub __pc: __uint32_t,
    pub __cpsr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_thread_state() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_thread_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_thread_state>(),
        68usize,
        concat!("Size of: ", stringify!(__darwin_arm_thread_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_thread_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_arm_thread_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__r) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state),
            "::",
            stringify!(__r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sp) as usize - ptr as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state),
            "::",
            stringify!(__sp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__lr) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state),
            "::",
            stringify!(__lr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pc) as usize - ptr as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state),
            "::",
            stringify!(__pc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__cpsr) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state),
            "::",
            stringify!(__cpsr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_thread_state64 {
    pub __x: [__uint64_t; 29usize],
    pub __fp: __uint64_t,
    pub __lr: __uint64_t,
    pub __sp: __uint64_t,
    pub __pc: __uint64_t,
    pub __cpsr: __uint32_t,
    pub __pad: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_thread_state64() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_thread_state64> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_thread_state64>(),
        272usize,
        concat!("Size of: ", stringify!(__darwin_arm_thread_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_thread_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_thread_state64))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state64),
            "::",
            stringify!(__x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fp) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state64),
            "::",
            stringify!(__fp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__lr) as usize - ptr as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state64),
            "::",
            stringify!(__lr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sp) as usize - ptr as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state64),
            "::",
            stringify!(__sp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pc) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state64),
            "::",
            stringify!(__pc)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__cpsr) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state64),
            "::",
            stringify!(__cpsr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad) as usize - ptr as usize },
        268usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_thread_state64),
            "::",
            stringify!(__pad)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_vfp_state {
    pub __r: [__uint32_t; 64usize],
    pub __fpscr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_vfp_state() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_vfp_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_vfp_state>(),
        260usize,
        concat!("Size of: ", stringify!(__darwin_arm_vfp_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_vfp_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_arm_vfp_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__r) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_vfp_state),
            "::",
            stringify!(__r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fpscr) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_vfp_state),
            "::",
            stringify!(__fpscr)
        )
    );
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state64 {
    pub __v: [__uint128_t; 32usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_neon_state64() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_neon_state64> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_neon_state64>(),
        528usize,
        concat!("Size of: ", stringify!(__darwin_arm_neon_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_neon_state64>(),
        16usize,
        concat!("Alignment of ", stringify!(__darwin_arm_neon_state64))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__v) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_neon_state64),
            "::",
            stringify!(__v)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fpsr) as usize - ptr as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_neon_state64),
            "::",
            stringify!(__fpsr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fpcr) as usize - ptr as usize },
        516usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_neon_state64),
            "::",
            stringify!(__fpcr)
        )
    );
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_neon_state {
    pub __v: [__uint128_t; 16usize],
    pub __fpsr: __uint32_t,
    pub __fpcr: __uint32_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_neon_state() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_neon_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_neon_state>(),
        272usize,
        concat!("Size of: ", stringify!(__darwin_arm_neon_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_neon_state>(),
        16usize,
        concat!("Alignment of ", stringify!(__darwin_arm_neon_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__v) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_neon_state),
            "::",
            stringify!(__v)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fpsr) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_neon_state),
            "::",
            stringify!(__fpsr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fpcr) as usize - ptr as usize },
        260usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_neon_state),
            "::",
            stringify!(__fpcr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_pagein_state {
    pub __pagein_error: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___arm_pagein_state() {
    const UNINIT: ::std::mem::MaybeUninit<__arm_pagein_state> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__arm_pagein_state>(),
        4usize,
        concat!("Size of: ", stringify!(__arm_pagein_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__arm_pagein_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__arm_pagein_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pagein_error) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__arm_pagein_state),
            "::",
            stringify!(__pagein_error)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __arm_legacy_debug_state {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
}
#[test]
fn bindgen_test_layout___arm_legacy_debug_state() {
    const UNINIT: ::std::mem::MaybeUninit<__arm_legacy_debug_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__arm_legacy_debug_state>(),
        256usize,
        concat!("Size of: ", stringify!(__arm_legacy_debug_state))
    );
    assert_eq!(
        ::std::mem::align_of::<__arm_legacy_debug_state>(),
        4usize,
        concat!("Alignment of ", stringify!(__arm_legacy_debug_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__bvr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__arm_legacy_debug_state),
            "::",
            stringify!(__bvr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__bcr) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__arm_legacy_debug_state),
            "::",
            stringify!(__bcr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wvr) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__arm_legacy_debug_state),
            "::",
            stringify!(__wvr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wcr) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(__arm_legacy_debug_state),
            "::",
            stringify!(__wcr)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state32 {
    pub __bvr: [__uint32_t; 16usize],
    pub __bcr: [__uint32_t; 16usize],
    pub __wvr: [__uint32_t; 16usize],
    pub __wcr: [__uint32_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_debug_state32() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_debug_state32> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_debug_state32>(),
        264usize,
        concat!("Size of: ", stringify!(__darwin_arm_debug_state32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_debug_state32>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_debug_state32))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__bvr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state32),
            "::",
            stringify!(__bvr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__bcr) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state32),
            "::",
            stringify!(__bcr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wvr) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state32),
            "::",
            stringify!(__wvr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wcr) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state32),
            "::",
            stringify!(__wcr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__mdscr_el1) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state32),
            "::",
            stringify!(__mdscr_el1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_debug_state64 {
    pub __bvr: [__uint64_t; 16usize],
    pub __bcr: [__uint64_t; 16usize],
    pub __wvr: [__uint64_t; 16usize],
    pub __wcr: [__uint64_t; 16usize],
    pub __mdscr_el1: __uint64_t,
}
#[test]
fn bindgen_test_layout___darwin_arm_debug_state64() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_debug_state64> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_debug_state64>(),
        520usize,
        concat!("Size of: ", stringify!(__darwin_arm_debug_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_debug_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_debug_state64))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__bvr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state64),
            "::",
            stringify!(__bvr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__bcr) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state64),
            "::",
            stringify!(__bcr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wvr) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state64),
            "::",
            stringify!(__wvr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wcr) as usize - ptr as usize },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state64),
            "::",
            stringify!(__wcr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__mdscr_el1) as usize - ptr as usize },
        512usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_debug_state64),
            "::",
            stringify!(__mdscr_el1)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_arm_cpmu_state64 {
    pub __ctrs: [__uint64_t; 16usize],
}
#[test]
fn bindgen_test_layout___darwin_arm_cpmu_state64() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_arm_cpmu_state64> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_arm_cpmu_state64>(),
        128usize,
        concat!("Size of: ", stringify!(__darwin_arm_cpmu_state64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_arm_cpmu_state64>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_arm_cpmu_state64))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__ctrs) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_arm_cpmu_state64),
            "::",
            stringify!(__ctrs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext32 {
    pub __es: __darwin_arm_exception_state,
    pub __ss: __darwin_arm_thread_state,
    pub __fs: __darwin_arm_vfp_state,
}
#[test]
fn bindgen_test_layout___darwin_mcontext32() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_mcontext32> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext32>(),
        340usize,
        concat!("Size of: ", stringify!(__darwin_mcontext32))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext32>(),
        4usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext32))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__es) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext32),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__ss) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext32),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fs) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext32),
            "::",
            stringify!(__fs)
        )
    );
}
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_mcontext64 {
    pub __es: __darwin_arm_exception_state64,
    pub __ss: __darwin_arm_thread_state64,
    pub __ns: __darwin_arm_neon_state64,
}
#[test]
fn bindgen_test_layout___darwin_mcontext64() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_mcontext64> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_mcontext64>(),
        816usize,
        concat!("Size of: ", stringify!(__darwin_mcontext64))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_mcontext64>(),
        16usize,
        concat!("Alignment of ", stringify!(__darwin_mcontext64))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__es) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext64),
            "::",
            stringify!(__es)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__ss) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext64),
            "::",
            stringify!(__ss)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__ns) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_mcontext64),
            "::",
            stringify!(__ns)
        )
    );
}
pub type mcontext_t = *mut __darwin_mcontext64;
pub type pthread_attr_t = __darwin_pthread_attr_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_sigaltstack {
    pub ss_sp: *mut ::std::os::raw::c_void,
    pub ss_size: __darwin_size_t,
    pub ss_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___darwin_sigaltstack() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_sigaltstack> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_sigaltstack>(),
        24usize,
        concat!("Size of: ", stringify!(__darwin_sigaltstack))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_sigaltstack>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_sigaltstack))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ss_sp) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_sigaltstack),
            "::",
            stringify!(ss_sp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ss_size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_sigaltstack),
            "::",
            stringify!(ss_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ss_flags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_sigaltstack),
            "::",
            stringify!(ss_flags)
        )
    );
}
pub type stack_t = __darwin_sigaltstack;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __darwin_ucontext {
    pub uc_onstack: ::std::os::raw::c_int,
    pub uc_sigmask: __darwin_sigset_t,
    pub uc_stack: __darwin_sigaltstack,
    pub uc_link: *mut __darwin_ucontext,
    pub uc_mcsize: __darwin_size_t,
    pub uc_mcontext: *mut __darwin_mcontext64,
}
#[test]
fn bindgen_test_layout___darwin_ucontext() {
    const UNINIT: ::std::mem::MaybeUninit<__darwin_ucontext> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__darwin_ucontext>(),
        56usize,
        concat!("Size of: ", stringify!(__darwin_ucontext))
    );
    assert_eq!(
        ::std::mem::align_of::<__darwin_ucontext>(),
        8usize,
        concat!("Alignment of ", stringify!(__darwin_ucontext))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uc_onstack) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_onstack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uc_sigmask) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_sigmask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uc_stack) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_stack)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uc_link) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_link)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uc_mcsize) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_mcsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).uc_mcontext) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__darwin_ucontext),
            "::",
            stringify!(uc_mcontext)
        )
    );
}
pub type ucontext_t = __darwin_ucontext;
pub type sigset_t = __darwin_sigset_t;
pub type uid_t = __darwin_uid_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union sigval {
    pub sival_int: ::std::os::raw::c_int,
    pub sival_ptr: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_sigval() {
    const UNINIT: ::std::mem::MaybeUninit<sigval> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<sigval>(),
        8usize,
        concat!("Size of: ", stringify!(sigval))
    );
    assert_eq!(
        ::std::mem::align_of::<sigval>(),
        8usize,
        concat!("Alignment of ", stringify!(sigval))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sival_int) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigval),
            "::",
            stringify!(sival_int)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sival_ptr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigval),
            "::",
            stringify!(sival_ptr)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigevent {
    pub sigev_notify: ::std::os::raw::c_int,
    pub sigev_signo: ::std::os::raw::c_int,
    pub sigev_value: sigval,
    pub sigev_notify_function: ::std::option::Option<unsafe extern "C" fn(arg1: sigval)>,
    pub sigev_notify_attributes: *mut pthread_attr_t,
}
#[test]
fn bindgen_test_layout_sigevent() {
    const UNINIT: ::std::mem::MaybeUninit<sigevent> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<sigevent>(),
        32usize,
        concat!("Size of: ", stringify!(sigevent))
    );
    assert_eq!(
        ::std::mem::align_of::<sigevent>(),
        8usize,
        concat!("Alignment of ", stringify!(sigevent))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sigev_notify) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_notify)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sigev_signo) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_signo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sigev_value) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sigev_notify_function) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_notify_function)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sigev_notify_attributes) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(sigevent),
            "::",
            stringify!(sigev_notify_attributes)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __siginfo {
    pub si_signo: ::std::os::raw::c_int,
    pub si_errno: ::std::os::raw::c_int,
    pub si_code: ::std::os::raw::c_int,
    pub si_pid: pid_t,
    pub si_uid: uid_t,
    pub si_status: ::std::os::raw::c_int,
    pub si_addr: *mut ::std::os::raw::c_void,
    pub si_value: sigval,
    pub si_band: ::std::os::raw::c_long,
    pub __pad: [::std::os::raw::c_ulong; 7usize],
}
#[test]
fn bindgen_test_layout___siginfo() {
    const UNINIT: ::std::mem::MaybeUninit<__siginfo> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__siginfo>(),
        104usize,
        concat!("Size of: ", stringify!(__siginfo))
    );
    assert_eq!(
        ::std::mem::align_of::<__siginfo>(),
        8usize,
        concat!("Alignment of ", stringify!(__siginfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_signo) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_signo)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_errno) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_errno)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_code) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_code)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_pid) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_pid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_uid) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_uid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_status) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_addr) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_addr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_value) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_value)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).si_band) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(si_band)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__siginfo),
            "::",
            stringify!(__pad)
        )
    );
}
pub type siginfo_t = __siginfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __sigaction_u {
    pub __sa_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub __sa_sigaction: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: *mut __siginfo,
            arg3: *mut ::std::os::raw::c_void,
        ),
    >,
}
#[test]
fn bindgen_test_layout___sigaction_u() {
    const UNINIT: ::std::mem::MaybeUninit<__sigaction_u> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sigaction_u>(),
        8usize,
        concat!("Size of: ", stringify!(__sigaction_u))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigaction_u>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigaction_u))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sa_handler) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction_u),
            "::",
            stringify!(__sa_handler)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sa_sigaction) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction_u),
            "::",
            stringify!(__sa_sigaction)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_tramp: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
            arg4: *mut siginfo_t,
            arg5: *mut ::std::os::raw::c_void,
        ),
    >,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___sigaction() {
    const UNINIT: ::std::mem::MaybeUninit<__sigaction> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sigaction>(),
        24usize,
        concat!("Size of: ", stringify!(__sigaction))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigaction>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigaction))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sigaction_u) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(__sigaction_u)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sa_tramp) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(sa_tramp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sa_mask) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(sa_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sa_flags) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigaction),
            "::",
            stringify!(sa_flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    pub __sigaction_u: __sigaction_u,
    pub sa_mask: sigset_t,
    pub sa_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigaction() {
    const UNINIT: ::std::mem::MaybeUninit<sigaction> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<sigaction>(),
        16usize,
        concat!("Size of: ", stringify!(sigaction))
    );
    assert_eq!(
        ::std::mem::align_of::<sigaction>(),
        8usize,
        concat!("Alignment of ", stringify!(sigaction))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__sigaction_u) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(__sigaction_u)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sa_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sa_flags) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sigaction),
            "::",
            stringify!(sa_flags)
        )
    );
}
pub type sig_t = ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigvec {
    pub sv_handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    pub sv_mask: ::std::os::raw::c_int,
    pub sv_flags: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigvec() {
    const UNINIT: ::std::mem::MaybeUninit<sigvec> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<sigvec>(),
        16usize,
        concat!("Size of: ", stringify!(sigvec))
    );
    assert_eq!(
        ::std::mem::align_of::<sigvec>(),
        8usize,
        concat!("Alignment of ", stringify!(sigvec))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sv_handler) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigvec),
            "::",
            stringify!(sv_handler)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sv_mask) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigvec),
            "::",
            stringify!(sv_mask)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).sv_flags) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(sigvec),
            "::",
            stringify!(sv_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sigstack {
    pub ss_sp: *mut ::std::os::raw::c_char,
    pub ss_onstack: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_sigstack() {
    const UNINIT: ::std::mem::MaybeUninit<sigstack> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<sigstack>(),
        16usize,
        concat!("Size of: ", stringify!(sigstack))
    );
    assert_eq!(
        ::std::mem::align_of::<sigstack>(),
        8usize,
        concat!("Alignment of ", stringify!(sigstack))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ss_sp) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(sigstack),
            "::",
            stringify!(ss_sp)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ss_onstack) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(sigstack),
            "::",
            stringify!(ss_onstack)
        )
    );
}
extern "C" {
    pub fn signal(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
    ) -> ::std::option::Option<
        unsafe extern "C" fn(
            arg1: ::std::os::raw::c_int,
            arg2: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        ),
    >;
}
pub type int_least8_t = i8;
pub type int_least16_t = i16;
pub type int_least32_t = i32;
pub type int_least64_t = i64;
pub type uint_least8_t = u8;
pub type uint_least16_t = u16;
pub type uint_least32_t = u32;
pub type uint_least64_t = u64;
pub type int_fast8_t = i8;
pub type int_fast16_t = i16;
pub type int_fast32_t = i32;
pub type int_fast64_t = i64;
pub type uint_fast8_t = u8;
pub type uint_fast16_t = u16;
pub type uint_fast32_t = u32;
pub type uint_fast64_t = u64;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __darwin_time_t,
    pub tv_usec: __darwin_suseconds_t,
}
#[test]
fn bindgen_test_layout_timeval() {
    const UNINIT: ::std::mem::MaybeUninit<timeval> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<timeval>(),
        16usize,
        concat!("Size of: ", stringify!(timeval))
    );
    assert_eq!(
        ::std::mem::align_of::<timeval>(),
        8usize,
        concat!("Alignment of ", stringify!(timeval))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_usec) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timeval),
            "::",
            stringify!(tv_usec)
        )
    );
}
pub type rlim_t = __uint64_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub ru_maxrss: ::std::os::raw::c_long,
    pub ru_ixrss: ::std::os::raw::c_long,
    pub ru_idrss: ::std::os::raw::c_long,
    pub ru_isrss: ::std::os::raw::c_long,
    pub ru_minflt: ::std::os::raw::c_long,
    pub ru_majflt: ::std::os::raw::c_long,
    pub ru_nswap: ::std::os::raw::c_long,
    pub ru_inblock: ::std::os::raw::c_long,
    pub ru_oublock: ::std::os::raw::c_long,
    pub ru_msgsnd: ::std::os::raw::c_long,
    pub ru_msgrcv: ::std::os::raw::c_long,
    pub ru_nsignals: ::std::os::raw::c_long,
    pub ru_nvcsw: ::std::os::raw::c_long,
    pub ru_nivcsw: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_rusage() {
    const UNINIT: ::std::mem::MaybeUninit<rusage> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage>(),
        144usize,
        concat!("Size of: ", stringify!(rusage))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_utime) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_utime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_stime) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_stime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_maxrss) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_maxrss)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_ixrss) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_ixrss)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_idrss) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_idrss)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_isrss) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_isrss)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_minflt) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_minflt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_majflt) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_majflt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_nswap) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nswap)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_inblock) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_inblock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_oublock) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_oublock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_msgsnd) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_msgsnd)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_msgrcv) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_msgrcv)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_nsignals) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nsignals)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_nvcsw) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nvcsw)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ru_nivcsw) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage),
            "::",
            stringify!(ru_nivcsw)
        )
    );
}
pub type rusage_info_t = *mut ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v0 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v0() {
    const UNINIT: ::std::mem::MaybeUninit<rusage_info_v0> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v0>(),
        96usize,
        concat!("Size of: ", stringify!(rusage_info_v0))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v0>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v0))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v0),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v1 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v1() {
    const UNINIT: ::std::mem::MaybeUninit<rusage_info_v1> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v1>(),
        144usize,
        concat!("Size of: ", stringify!(rusage_info_v1))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v1>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v1),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v2 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v2() {
    const UNINIT: ::std::mem::MaybeUninit<rusage_info_v2> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v2>(),
        160usize,
        concat!("Size of: ", stringify!(rusage_info_v2))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v2>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v2))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v2),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v3 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v3() {
    const UNINIT: ::std::mem::MaybeUninit<rusage_info_v3> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v3>(),
        232usize,
        concat!("Size of: ", stringify!(rusage_info_v3))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v3>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v3))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_default) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_default)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_maintenance) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_maintenance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_background) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_background)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_utility) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_utility)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_legacy) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_legacy)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_initiated) as usize - ptr as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_user_initiated)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_interactive) as usize - ptr as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_cpu_time_qos_user_interactive)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_billed_system_time) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_billed_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_serviced_system_time) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v3),
            "::",
            stringify!(ri_serviced_system_time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v4 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v4() {
    const UNINIT: ::std::mem::MaybeUninit<rusage_info_v4> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v4>(),
        296usize,
        concat!("Size of: ", stringify!(rusage_info_v4))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v4>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v4))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_default) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_default)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_maintenance) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_maintenance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_background) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_background)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_utility) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_utility)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_legacy) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_legacy)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_initiated) as usize - ptr as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_user_initiated)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_interactive) as usize - ptr as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cpu_time_qos_user_interactive)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_billed_system_time) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_billed_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_serviced_system_time) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_serviced_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_logical_writes) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_logical_writes)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_lifetime_max_phys_footprint) as usize - ptr as usize
        },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_lifetime_max_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_instructions) as usize - ptr as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_instructions)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cycles) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_cycles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_billed_energy) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_billed_energy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_serviced_energy) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_serviced_energy)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_interval_max_phys_footprint) as usize - ptr as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_interval_max_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_runnable_time) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v4),
            "::",
            stringify!(ri_runnable_time)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v5 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
}
#[test]
fn bindgen_test_layout_rusage_info_v5() {
    const UNINIT: ::std::mem::MaybeUninit<rusage_info_v5> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v5>(),
        304usize,
        concat!("Size of: ", stringify!(rusage_info_v5))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v5>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v5))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_default) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cpu_time_qos_default)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_maintenance) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cpu_time_qos_maintenance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_background) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cpu_time_qos_background)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_utility) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cpu_time_qos_utility)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_legacy) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cpu_time_qos_legacy)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_initiated) as usize - ptr as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cpu_time_qos_user_initiated)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_interactive) as usize - ptr as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cpu_time_qos_user_interactive)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_billed_system_time) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_billed_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_serviced_system_time) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_serviced_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_logical_writes) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_logical_writes)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_lifetime_max_phys_footprint) as usize - ptr as usize
        },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_lifetime_max_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_instructions) as usize - ptr as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_instructions)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cycles) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_cycles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_billed_energy) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_billed_energy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_serviced_energy) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_serviced_energy)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_interval_max_phys_footprint) as usize - ptr as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_interval_max_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_runnable_time) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_runnable_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_flags) as usize - ptr as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v5),
            "::",
            stringify!(ri_flags)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rusage_info_v6 {
    pub ri_uuid: [u8; 16usize],
    pub ri_user_time: u64,
    pub ri_system_time: u64,
    pub ri_pkg_idle_wkups: u64,
    pub ri_interrupt_wkups: u64,
    pub ri_pageins: u64,
    pub ri_wired_size: u64,
    pub ri_resident_size: u64,
    pub ri_phys_footprint: u64,
    pub ri_proc_start_abstime: u64,
    pub ri_proc_exit_abstime: u64,
    pub ri_child_user_time: u64,
    pub ri_child_system_time: u64,
    pub ri_child_pkg_idle_wkups: u64,
    pub ri_child_interrupt_wkups: u64,
    pub ri_child_pageins: u64,
    pub ri_child_elapsed_abstime: u64,
    pub ri_diskio_bytesread: u64,
    pub ri_diskio_byteswritten: u64,
    pub ri_cpu_time_qos_default: u64,
    pub ri_cpu_time_qos_maintenance: u64,
    pub ri_cpu_time_qos_background: u64,
    pub ri_cpu_time_qos_utility: u64,
    pub ri_cpu_time_qos_legacy: u64,
    pub ri_cpu_time_qos_user_initiated: u64,
    pub ri_cpu_time_qos_user_interactive: u64,
    pub ri_billed_system_time: u64,
    pub ri_serviced_system_time: u64,
    pub ri_logical_writes: u64,
    pub ri_lifetime_max_phys_footprint: u64,
    pub ri_instructions: u64,
    pub ri_cycles: u64,
    pub ri_billed_energy: u64,
    pub ri_serviced_energy: u64,
    pub ri_interval_max_phys_footprint: u64,
    pub ri_runnable_time: u64,
    pub ri_flags: u64,
    pub ri_user_ptime: u64,
    pub ri_system_ptime: u64,
    pub ri_pinstructions: u64,
    pub ri_pcycles: u64,
    pub ri_energy_nj: u64,
    pub ri_penergy_nj: u64,
    pub ri_secure_time_in_system: u64,
    pub ri_secure_ptime_in_system: u64,
    pub ri_neural_footprint: u64,
    pub ri_lifetime_max_neural_footprint: u64,
    pub ri_interval_max_neural_footprint: u64,
    pub ri_reserved: [u64; 9usize],
}
#[test]
fn bindgen_test_layout_rusage_info_v6() {
    const UNINIT: ::std::mem::MaybeUninit<rusage_info_v6> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rusage_info_v6>(),
        464usize,
        concat!("Size of: ", stringify!(rusage_info_v6))
    );
    assert_eq!(
        ::std::mem::align_of::<rusage_info_v6>(),
        8usize,
        concat!("Alignment of ", stringify!(rusage_info_v6))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_uuid) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_uuid)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_time) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_time) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pkg_idle_wkups) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_interrupt_wkups) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pageins) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_wired_size) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_wired_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_resident_size) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_resident_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_phys_footprint) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_start_abstime) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_proc_start_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_proc_exit_abstime) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_proc_exit_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_user_time) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_child_user_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_system_time) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_child_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pkg_idle_wkups) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_child_pkg_idle_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_interrupt_wkups) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_child_interrupt_wkups)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_pageins) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_child_pageins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_child_elapsed_abstime) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_child_elapsed_abstime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_bytesread) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_diskio_bytesread)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_diskio_byteswritten) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_diskio_byteswritten)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_default) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cpu_time_qos_default)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_maintenance) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cpu_time_qos_maintenance)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_background) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cpu_time_qos_background)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_utility) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cpu_time_qos_utility)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_legacy) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cpu_time_qos_legacy)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_initiated) as usize - ptr as usize
        },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cpu_time_qos_user_initiated)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_cpu_time_qos_user_interactive) as usize - ptr as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cpu_time_qos_user_interactive)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_billed_system_time) as usize - ptr as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_billed_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_serviced_system_time) as usize - ptr as usize },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_serviced_system_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_logical_writes) as usize - ptr as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_logical_writes)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_lifetime_max_phys_footprint) as usize - ptr as usize
        },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_lifetime_max_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_instructions) as usize - ptr as usize },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_instructions)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_cycles) as usize - ptr as usize },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_cycles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_billed_energy) as usize - ptr as usize },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_billed_energy)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_serviced_energy) as usize - ptr as usize },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_serviced_energy)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_interval_max_phys_footprint) as usize - ptr as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_interval_max_phys_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_runnable_time) as usize - ptr as usize },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_runnable_time)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_flags) as usize - ptr as usize },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_user_ptime) as usize - ptr as usize },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_user_ptime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_system_ptime) as usize - ptr as usize },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_system_ptime)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pinstructions) as usize - ptr as usize },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_pinstructions)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_pcycles) as usize - ptr as usize },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_pcycles)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_energy_nj) as usize - ptr as usize },
        336usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_energy_nj)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_penergy_nj) as usize - ptr as usize },
        344usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_penergy_nj)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_secure_time_in_system) as usize - ptr as usize },
        352usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_secure_time_in_system)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_secure_ptime_in_system) as usize - ptr as usize },
        360usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_secure_ptime_in_system)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_neural_footprint) as usize - ptr as usize },
        368usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_neural_footprint)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_lifetime_max_neural_footprint) as usize - ptr as usize
        },
        376usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_lifetime_max_neural_footprint)
        )
    );
    assert_eq!(
        unsafe {
            ::std::ptr::addr_of!((*ptr).ri_interval_max_neural_footprint) as usize - ptr as usize
        },
        384usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_interval_max_neural_footprint)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ri_reserved) as usize - ptr as usize },
        392usize,
        concat!(
            "Offset of field: ",
            stringify!(rusage_info_v6),
            "::",
            stringify!(ri_reserved)
        )
    );
}
pub type rusage_info_current = rusage_info_v6;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
#[test]
fn bindgen_test_layout_rlimit() {
    const UNINIT: ::std::mem::MaybeUninit<rlimit> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<rlimit>(),
        16usize,
        concat!("Size of: ", stringify!(rlimit))
    );
    assert_eq!(
        ::std::mem::align_of::<rlimit>(),
        8usize,
        concat!("Alignment of ", stringify!(rlimit))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rlim_cur) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rlimit),
            "::",
            stringify!(rlim_cur)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rlim_max) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rlimit),
            "::",
            stringify!(rlim_max)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct proc_rlimit_control_wakeupmon {
    pub wm_flags: u32,
    pub wm_rate: i32,
}
#[test]
fn bindgen_test_layout_proc_rlimit_control_wakeupmon() {
    const UNINIT: ::std::mem::MaybeUninit<proc_rlimit_control_wakeupmon> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<proc_rlimit_control_wakeupmon>(),
        8usize,
        concat!("Size of: ", stringify!(proc_rlimit_control_wakeupmon))
    );
    assert_eq!(
        ::std::mem::align_of::<proc_rlimit_control_wakeupmon>(),
        4usize,
        concat!("Alignment of ", stringify!(proc_rlimit_control_wakeupmon))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wm_flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(proc_rlimit_control_wakeupmon),
            "::",
            stringify!(wm_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wm_rate) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(proc_rlimit_control_wakeupmon),
            "::",
            stringify!(wm_rate)
        )
    );
}
extern "C" {
    pub fn getpriority(arg1: ::std::os::raw::c_int, arg2: id_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrlimit(arg1: ::std::os::raw::c_int, arg2: *mut rlimit) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getrusage(arg1: ::std::os::raw::c_int, arg2: *mut rusage) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setpriority(
        arg1: ::std::os::raw::c_int,
        arg2: id_t,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setiopolicy_np(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setrlimit(arg1: ::std::os::raw::c_int, arg2: *const rlimit) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union wait {
    pub w_status: ::std::os::raw::c_int,
    pub w_T: wait__bindgen_ty_1,
    pub w_S: wait__bindgen_ty_2,
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_1 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_wait__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<wait__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(wait__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<wait__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(wait__bindgen_ty_1))
    );
}
impl wait__bindgen_ty_1 {
    #[inline]
    pub fn w_Termsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 7u8) as u32) }
    }
    #[inline]
    pub fn set_w_Termsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 7u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Coredump(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(7usize, 1u8) as u32) }
    }
    #[inline]
    pub fn set_w_Coredump(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(7usize, 1u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Retcode(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Retcode(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Termsig: ::std::os::raw::c_uint,
        w_Coredump: ::std::os::raw::c_uint,
        w_Retcode: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 7u8, {
            let w_Termsig: u32 = unsafe { ::std::mem::transmute(w_Termsig) };
            w_Termsig as u64
        });
        __bindgen_bitfield_unit.set(7usize, 1u8, {
            let w_Coredump: u32 = unsafe { ::std::mem::transmute(w_Coredump) };
            w_Coredump as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Retcode: u32 = unsafe { ::std::mem::transmute(w_Retcode) };
            w_Retcode as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Copy, Clone)]
pub struct wait__bindgen_ty_2 {
    pub _bitfield_align_1: [u16; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 4usize]>,
}
#[test]
fn bindgen_test_layout_wait__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<wait__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(wait__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<wait__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(wait__bindgen_ty_2))
    );
}
impl wait__bindgen_ty_2 {
    #[inline]
    pub fn w_Stopval(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(0usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopval(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Stopsig(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(8usize, 8u8) as u32) }
    }
    #[inline]
    pub fn set_w_Stopsig(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(8usize, 8u8, val as u64)
        }
    }
    #[inline]
    pub fn w_Filler(&self) -> ::std::os::raw::c_uint {
        unsafe { ::std::mem::transmute(self._bitfield_1.get(16usize, 16u8) as u32) }
    }
    #[inline]
    pub fn set_w_Filler(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(16usize, 16u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        w_Stopval: ::std::os::raw::c_uint,
        w_Stopsig: ::std::os::raw::c_uint,
        w_Filler: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 4usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 4usize]> = Default::default();
        __bindgen_bitfield_unit.set(0usize, 8u8, {
            let w_Stopval: u32 = unsafe { ::std::mem::transmute(w_Stopval) };
            w_Stopval as u64
        });
        __bindgen_bitfield_unit.set(8usize, 8u8, {
            let w_Stopsig: u32 = unsafe { ::std::mem::transmute(w_Stopsig) };
            w_Stopsig as u64
        });
        __bindgen_bitfield_unit.set(16usize, 16u8, {
            let w_Filler: u32 = unsafe { ::std::mem::transmute(w_Filler) };
            w_Filler as u64
        });
        __bindgen_bitfield_unit
    }
}
#[test]
fn bindgen_test_layout_wait() {
    const UNINIT: ::std::mem::MaybeUninit<wait> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<wait>(),
        4usize,
        concat!("Size of: ", stringify!(wait))
    );
    assert_eq!(
        ::std::mem::align_of::<wait>(),
        4usize,
        concat!("Alignment of ", stringify!(wait))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w_status) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(wait),
            "::",
            stringify!(w_status)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w_T) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(wait), "::", stringify!(w_T))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).w_S) as usize - ptr as usize },
        0usize,
        concat!("Offset of field: ", stringify!(wait), "::", stringify!(w_S))
    );
}
extern "C" {
    pub fn wait(arg1: *mut ::std::os::raw::c_int) -> pid_t;
}
extern "C" {
    pub fn waitpid(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> pid_t;
}
extern "C" {
    pub fn waitid(
        arg1: idtype_t,
        arg2: id_t,
        arg3: *mut siginfo_t,
        arg4: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wait3(
        arg1: *mut ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
        arg3: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn wait4(
        arg1: pid_t,
        arg2: *mut ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: *mut rusage,
    ) -> pid_t;
}
extern "C" {
    pub fn alloca(arg1: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
pub type ct_rune_t = __darwin_ct_rune_t;
pub type rune_t = __darwin_rune_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct div_t {
    pub quot: ::std::os::raw::c_int,
    pub rem: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_div_t() {
    const UNINIT: ::std::mem::MaybeUninit<div_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<div_t>(),
        8usize,
        concat!("Size of: ", stringify!(div_t))
    );
    assert_eq!(
        ::std::mem::align_of::<div_t>(),
        4usize,
        concat!("Alignment of ", stringify!(div_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(div_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ldiv_t {
    pub quot: ::std::os::raw::c_long,
    pub rem: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_ldiv_t() {
    const UNINIT: ::std::mem::MaybeUninit<ldiv_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<ldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(ldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<ldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(ldiv_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(ldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct lldiv_t {
    pub quot: ::std::os::raw::c_longlong,
    pub rem: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_lldiv_t() {
    const UNINIT: ::std::mem::MaybeUninit<lldiv_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<lldiv_t>(),
        16usize,
        concat!("Size of: ", stringify!(lldiv_t))
    );
    assert_eq!(
        ::std::mem::align_of::<lldiv_t>(),
        8usize,
        concat!("Alignment of ", stringify!(lldiv_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).quot) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(quot)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rem) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(lldiv_t),
            "::",
            stringify!(rem)
        )
    );
}
extern "C" {
    pub static mut __mb_cur_max: ::std::os::raw::c_int;
}
pub type malloc_type_id_t = ::std::os::raw::c_ulonglong;
extern "C" {
    pub fn malloc_type_malloc(
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_calloc(
        count: usize,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_free(ptr: *mut ::std::os::raw::c_void, type_id: malloc_type_id_t);
}
extern "C" {
    pub fn malloc_type_realloc(
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_valloc(
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_aligned_alloc(
        alignment: usize,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_posix_memalign(
        memptr: *mut *mut ::std::os::raw::c_void,
        alignment: usize,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _malloc_zone_t {
    _unused: [u8; 0],
}
pub type malloc_zone_t = _malloc_zone_t;
extern "C" {
    pub fn malloc_type_zone_malloc(
        zone: *mut malloc_zone_t,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_zone_calloc(
        zone: *mut malloc_zone_t,
        count: usize,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_zone_free(
        zone: *mut malloc_zone_t,
        ptr: *mut ::std::os::raw::c_void,
        type_id: malloc_type_id_t,
    );
}
extern "C" {
    pub fn malloc_type_zone_realloc(
        zone: *mut malloc_zone_t,
        ptr: *mut ::std::os::raw::c_void,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_zone_valloc(
        zone: *mut malloc_zone_t,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc_type_zone_memalign(
        zone: *mut malloc_zone_t,
        alignment: usize,
        size: usize,
        type_id: malloc_type_id_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        __count: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(arg1: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn reallocf(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn aligned_alloc(
        __alignment: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn abort() -> !;
}
extern "C" {
    pub fn abs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atexit(arg1: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        arg1: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atof(arg1: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn div(arg1: ::std::os::raw::c_int, arg2: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn exit(arg1: ::std::os::raw::c_int) -> !;
}
extern "C" {
    pub fn getenv(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn labs(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn ldiv(arg1: ::std::os::raw::c_long, arg2: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn llabs(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn lldiv(arg1: ::std::os::raw::c_longlong, arg2: ::std::os::raw::c_longlong) -> lldiv_t;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(arg1: *mut wchar_t, arg2: *const ::std::os::raw::c_char, arg3: usize) -> usize;
}
extern "C" {
    pub fn mbtowc(
        arg1: *mut wchar_t,
        arg2: *const ::std::os::raw::c_char,
        arg3: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn quick_exit(arg1: ::std::os::raw::c_int) -> !;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn strtod(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtol(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtold(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtoll(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoul(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoull(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn system(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wcstombs(arg1: *mut ::std::os::raw::c_char, arg2: *const wchar_t, arg3: usize) -> usize;
}
extern "C" {
    pub fn wctomb(arg1: *mut ::std::os::raw::c_char, arg2: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn _Exit(arg1: ::std::os::raw::c_int) -> !;
}
extern "C" {
    pub fn a64l(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn ecvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn erand48(arg1: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn fcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_int,
        arg4: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        arg1: f64,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getsubopt(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const *mut ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn grantpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate(
        arg1: ::std::os::raw::c_uint,
        arg2: *mut ::std::os::raw::c_char,
        arg3: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn jrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn l64a(arg1: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn lcong48(arg1: *mut ::std::os::raw::c_ushort);
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mktemp(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(arg1: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn posix_openpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ptsname(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ptsname_r(
        fildes: ::std::os::raw::c_int,
        buffer: *mut ::std::os::raw::c_char,
        buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putenv(arg1: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rand_r(arg1: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}_realpath$DARWIN_EXTSN"]
    pub fn realpath(
        arg1: *const ::std::os::raw::c_char,
        arg2: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn seed48(arg1: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __overwrite: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setkey(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn setstate(arg1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn srand48(arg1: ::std::os::raw::c_long);
}
extern "C" {
    pub fn srandom(arg1: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn unlockpt(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
pub type dev_t = __darwin_dev_t;
pub type mode_t = __darwin_mode_t;
extern "C" {
    pub fn arc4random() -> u32;
}
extern "C" {
    pub fn arc4random_addrandom(arg1: *mut ::std::os::raw::c_uchar, arg2: ::std::os::raw::c_int);
}
extern "C" {
    pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __nbytes: usize);
}
extern "C" {
    pub fn arc4random_stir();
}
extern "C" {
    pub fn arc4random_uniform(__upper_bound: u32) -> u32;
}
extern "C" {
    pub fn atexit_b(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bsearch_b(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn cgetcap(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn cgetclose() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetent(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
        arg3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetfirst(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetmatch(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnext(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetnum(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetset(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetstr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn cgetustr(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn daemon(
        arg1: ::std::os::raw::c_int,
        arg2: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn devname(arg1: dev_t, arg2: mode_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn devname_r(
        arg1: dev_t,
        arg2: mode_t,
        buf: *mut ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getbsize(
        arg1: *mut ::std::os::raw::c_int,
        arg2: *mut ::std::os::raw::c_long,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn getloadavg(arg1: *mut f64, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getprogname() -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn setprogname(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn heapsort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn heapsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mergesort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn psort(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *const ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn psort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn psort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn qsort_b(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        __compar: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn qsort_r(
        __base: *mut ::std::os::raw::c_void,
        __nel: usize,
        __width: usize,
        arg1: *mut ::std::os::raw::c_void,
        __compar: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_void,
                arg3: *const ::std::os::raw::c_void,
            ) -> ::std::os::raw::c_int,
        >,
    );
}
extern "C" {
    pub fn radixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rpmatch(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sradixsort(
        __base: *mut *const ::std::os::raw::c_uchar,
        __nel: ::std::os::raw::c_int,
        __table: *const ::std::os::raw::c_uchar,
        __endbyte: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sranddev();
}
extern "C" {
    pub fn srandomdev();
}
extern "C" {
    pub fn strtonum(
        __numstr: *const ::std::os::raw::c_char,
        __minval: ::std::os::raw::c_longlong,
        __maxval: ::std::os::raw::c_longlong,
        __errstrp: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __str: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub static mut suboptarg: *mut ::std::os::raw::c_char;
}
pub type va_list = __darwin_va_list;
extern "C" {
    pub fn renameat(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renamex_np(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameatx_np(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: *const ::std::os::raw::c_char,
        arg5: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
pub type fpos_t = __darwin_off_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sbuf {
    pub _base: *mut ::std::os::raw::c_uchar,
    pub _size: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___sbuf() {
    const UNINIT: ::std::mem::MaybeUninit<__sbuf> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sbuf>(),
        16usize,
        concat!("Size of: ", stringify!(__sbuf))
    );
    assert_eq!(
        ::std::mem::align_of::<__sbuf>(),
        8usize,
        concat!("Alignment of ", stringify!(__sbuf))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._base) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._size) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sbuf),
            "::",
            stringify!(_size)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILEX {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sFILE {
    pub _p: *mut ::std::os::raw::c_uchar,
    pub _r: ::std::os::raw::c_int,
    pub _w: ::std::os::raw::c_int,
    pub _flags: ::std::os::raw::c_short,
    pub _file: ::std::os::raw::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: ::std::os::raw::c_int,
    pub _cookie: *mut ::std::os::raw::c_void,
    pub _close: ::std::option::Option<
        unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub _read: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *mut ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _seek: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: fpos_t,
            arg3: ::std::os::raw::c_int,
        ) -> fpos_t,
    >,
    pub _write: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *mut ::std::os::raw::c_void,
            arg2: *const ::std::os::raw::c_char,
            arg3: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: ::std::os::raw::c_int,
    pub _ubuf: [::std::os::raw::c_uchar; 3usize],
    pub _nbuf: [::std::os::raw::c_uchar; 1usize],
    pub _lb: __sbuf,
    pub _blksize: ::std::os::raw::c_int,
    pub _offset: fpos_t,
}
#[test]
fn bindgen_test_layout___sFILE() {
    const UNINIT: ::std::mem::MaybeUninit<__sFILE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sFILE>(),
        152usize,
        concat!("Size of: ", stringify!(__sFILE))
    );
    assert_eq!(
        ::std::mem::align_of::<__sFILE>(),
        8usize,
        concat!("Alignment of ", stringify!(__sFILE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._p) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_p)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._r) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_r)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._w) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_w)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._flags) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._file) as usize - ptr as usize },
        18usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_file)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._bf) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_bf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lbfsize) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lbfsize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._cookie) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_cookie)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._close) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_close)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._read) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_read)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._seek) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_seek)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._write) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_write)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ub) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ub)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._extra) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_extra)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ur) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ur)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._ubuf) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_ubuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._nbuf) as usize - ptr as usize },
        119usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_nbuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lb) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_lb)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._blksize) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_blksize)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._offset) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(__sFILE),
            "::",
            stringify!(_offset)
        )
    );
}
pub type FILE = __sFILE;
extern "C" {
    pub static mut __stdinp: *mut FILE;
}
extern "C" {
    pub static mut __stdoutp: *mut FILE;
}
extern "C" {
    pub static mut __stderrp: *mut FILE;
}
extern "C" {
    pub fn clearerr(arg1: *mut FILE);
}
extern "C" {
    pub fn fclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn feof(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetpos(arg1: *mut FILE, arg2: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fprintf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputs(arg1: *const ::std::os::raw::c_char, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn freopen(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fscanf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fseek(
        arg1: *mut FILE,
        arg2: ::std::os::raw::c_long,
        arg3: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(arg1: *mut FILE, arg2: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(arg1: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __nitems: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn getc(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn gets(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn perror(arg1: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn putc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn remove(arg1: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rewind(arg1: *mut FILE);
}
extern "C" {
    pub fn scanf(arg1: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuf(arg1: *mut FILE, arg2: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        arg1: *mut FILE,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
        arg4: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ungetc(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        arg1: *const ::std::os::raw::c_char,
        arg2: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        arg1: *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fdopen(arg1: ::std::os::raw::c_int, arg2: *const ::std::os::raw::c_char) -> *mut FILE;
}
extern "C" {
    pub fn fileno(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pclose(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn __srget(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __svfscanf(
        arg1: *mut FILE,
        arg2: *const ::std::os::raw::c_char,
        arg3: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __swbuf(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flockfile(arg1: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(arg1: *mut FILE);
}
extern "C" {
    pub fn getc_unlocked(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(arg1: ::std::os::raw::c_int, arg2: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __prefix: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type off_t = __darwin_off_t;
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __offset: off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> off_t;
}
extern "C" {
    pub fn snprintf(
        __str: *mut ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __str: *mut ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __str: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        arg1: __builtin_va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        arg1: ::std::os::raw::c_int,
        arg2: *const ::std::os::raw::c_char,
        arg3: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getdelim(
        __linep: *mut *mut ::std::os::raw::c_char,
        __linecapp: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> isize;
}
extern "C" {
    pub fn getline(
        __linep: *mut *mut ::std::os::raw::c_char,
        __linecapp: *mut usize,
        __stream: *mut FILE,
    ) -> isize;
}
extern "C" {
    pub fn fmemopen(
        __buf: *mut ::std::os::raw::c_void,
        __size: usize,
        __mode: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufp: *mut *mut ::std::os::raw::c_char,
        __sizep: *mut usize,
    ) -> *mut FILE;
}
extern "C" {
    pub static sys_nerr: ::std::os::raw::c_int;
}
extern "C" {
    pub static sys_errlist: [*const ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn asprintf(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ctermid_r(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fgetln(arg1: *mut FILE, arg2: *mut usize) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fmtcheck(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn fpurge(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(
        arg1: *mut FILE,
        arg2: *mut ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn setlinebuf(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vasprintf(
        arg1: *mut *mut ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funopen(
        arg1: *const ::std::os::raw::c_void,
        arg2: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *mut ::std::os::raw::c_char,
                arg3: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        arg3: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: *const ::std::os::raw::c_char,
                arg3: ::std::os::raw::c_int,
            ) -> ::std::os::raw::c_int,
        >,
        arg4: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: *mut ::std::os::raw::c_void,
                arg2: fpos_t,
                arg3: ::std::os::raw::c_int,
            ) -> fpos_t,
        >,
        arg5: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
        >,
    ) -> *mut FILE;
}
extern "C" {
    pub fn __sprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: usize,
        arg4: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __snprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: usize,
        arg3: ::std::os::raw::c_int,
        arg4: usize,
        arg5: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __vsprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: usize,
        arg4: *const ::std::os::raw::c_char,
        arg5: va_list,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __vsnprintf_chk(
        arg1: *mut ::std::os::raw::c_char,
        arg2: usize,
        arg3: ::std::os::raw::c_int,
        arg4: usize,
        arg5: *const ::std::os::raw::c_char,
        arg6: va_list,
    ) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_callback {
    pub fn_: ::std::option::Option<
        unsafe extern "C" fn(
            text: *const ::std::os::raw::c_char,
            data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub data: *const ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_secp256k1_callback() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_callback> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_callback>(),
        16usize,
        concat!("Size of: ", stringify!(secp256k1_callback))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_callback>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_callback))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fn_) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_callback),
            "::",
            stringify!(fn_)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_callback),
            "::",
            stringify!(data)
        )
    );
}
extern "C" {
    pub static default_illegal_callback: secp256k1_callback;
}
extern "C" {
    pub static default_error_callback: secp256k1_callback;
}
#[doc = " A scalar modulo the group order of the secp256k1 curve."]
#[repr(C)]
#[derive(Debug, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct secp256k1_scalar {
    pub d: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_secp256k1_scalar() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_scalar> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_scalar>(),
        32usize,
        concat!("Size of: ", stringify!(secp256k1_scalar))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_scalar>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_scalar))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).d) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_scalar),
            "::",
            stringify!(d)
        )
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_set_b32(
        r: *mut secp256k1_scalar,
        bin: *const ::std::os::raw::c_uchar,
        overflow: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_set_int(
        r: *mut secp256k1_scalar,
        v: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_get_b32(
        bin: *mut ::std::os::raw::c_uchar,
        a: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_add(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
        b: *const secp256k1_scalar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_mul(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
        b: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_inverse(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_negate(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_eq(
        a: *const secp256k1_scalar,
        b: *const secp256k1_scalar,
    ) -> ::std::os::raw::c_int;
}
#[doc = " This field implementation represents the value as 5 uint64_t limbs in base\n  2^52."]
#[repr(C)]
#[derive(Debug, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct secp256k1_fe {
    pub n: [u64; 5usize],
}
#[test]
fn bindgen_test_layout_secp256k1_fe() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_fe> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_fe>(),
        40usize,
        concat!("Size of: ", stringify!(secp256k1_fe))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_fe>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_fe))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_fe),
            "::",
            stringify!(n)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_fe_storage {
    pub n: [u64; 4usize],
}
#[test]
fn bindgen_test_layout_secp256k1_fe_storage() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_fe_storage> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_fe_storage>(),
        32usize,
        concat!("Size of: ", stringify!(secp256k1_fe_storage))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_fe_storage>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_fe_storage))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).n) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_fe_storage),
            "::",
            stringify!(n)
        )
    );
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_one: secp256k1_fe;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_const_beta: secp256k1_fe;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_normalize(
        r: *mut secp256k1_fe,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_normalize_var(
        r: *mut secp256k1_fe,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_set_int(
        r: *mut secp256k1_fe,
        a: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_is_odd(
        a: *const secp256k1_fe,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_cmp_var(
        a: *const secp256k1_fe,
        b: *const secp256k1_fe,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_set_b32_mod(
        r: *mut secp256k1_fe,
        a: *const ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_set_b32_limit(
        r: *mut secp256k1_fe,
        a: *const ::std::os::raw::c_uchar,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_get_b32(
        r: *mut ::std::os::raw::c_uchar,
        a: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_add(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_mul(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
        b: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_sqr(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_inv(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_impl_negate_unchecked(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
        m: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_fe_negate(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
        m: ::std::os::raw::c_uint,
    );
}
#[doc = " A group element in affine coordinates on the secp256k1 curve,\n  or occasionally on an isomorphic curve of the form y^2 = x^3 + 7*t^6.\n  Note: For exhaustive test mode, secp256k1 is replaced by a small subgroup of a different curve."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_ge {
    pub x: secp256k1_fe,
    pub y: secp256k1_fe,
    pub infinity: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_secp256k1_ge() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_ge> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_ge>(),
        88usize,
        concat!("Size of: ", stringify!(secp256k1_ge))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_ge>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_ge))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ge),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ge),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).infinity) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ge),
            "::",
            stringify!(infinity)
        )
    );
}
#[doc = " A group element of the secp256k1 curve, in jacobian coordinates.\n  Note: For exhastive test mode, secp256k1 is replaced by a small subgroup of a different curve."]
#[repr(C)]
#[derive(Debug, Copy, Clone, serde :: Serialize, serde :: Deserialize)]
pub struct secp256k1_gej {
    pub x: secp256k1_fe,
    pub y: secp256k1_fe,
    pub z: secp256k1_fe,
    pub infinity: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_secp256k1_gej() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_gej> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_gej>(),
        128usize,
        concat!("Size of: ", stringify!(secp256k1_gej))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_gej>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_gej))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_gej),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_gej),
            "::",
            stringify!(y)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).z) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_gej),
            "::",
            stringify!(z)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).infinity) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_gej),
            "::",
            stringify!(infinity)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_ge_storage {
    pub x: secp256k1_fe_storage,
    pub y: secp256k1_fe_storage,
}
#[test]
fn bindgen_test_layout_secp256k1_ge_storage() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_ge_storage> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_ge_storage>(),
        64usize,
        concat!("Size of: ", stringify!(secp256k1_ge_storage))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_ge_storage>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_ge_storage))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ge_storage),
            "::",
            stringify!(x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ge_storage),
            "::",
            stringify!(y)
        )
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ge_set_xo_var(
        r: *mut secp256k1_ge,
        x: *const secp256k1_fe,
        odd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_gej_set_ge(
        r: *mut secp256k1_gej,
        a: *const secp256k1_ge,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_gej_neg(
        r: *mut secp256k1_gej,
        a: *const secp256k1_gej,
    );
}
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_gej_add_var(
        r: *mut secp256k1_gej,
        a: *const secp256k1_gej,
        b: *const secp256k1_gej,
        rzr: *mut secp256k1_fe,
    );
}
pub type uint128_t = u128;
pub type int128_t = i128;
pub type secp256k1_uint128 = uint128_t;
pub type secp256k1_int128 = int128_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_modinv64_signed62 {
    pub v: [i64; 5usize],
}
#[test]
fn bindgen_test_layout_secp256k1_modinv64_signed62() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_modinv64_signed62> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_modinv64_signed62>(),
        40usize,
        concat!("Size of: ", stringify!(secp256k1_modinv64_signed62))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_modinv64_signed62>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_modinv64_signed62))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_modinv64_signed62),
            "::",
            stringify!(v)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_modinv64_modinfo {
    pub modulus: secp256k1_modinv64_signed62,
    pub modulus_inv62: u64,
}
#[test]
fn bindgen_test_layout_secp256k1_modinv64_modinfo() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_modinv64_modinfo> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_modinv64_modinfo>(),
        48usize,
        concat!("Size of: ", stringify!(secp256k1_modinv64_modinfo))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_modinv64_modinfo>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_modinv64_modinfo))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).modulus) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_modinv64_modinfo),
            "::",
            stringify!(modulus)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).modulus_inv62) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_modinv64_modinfo),
            "::",
            stringify!(modulus_inv62)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_modinv64_trans2x2 {
    pub u: i64,
    pub v: i64,
    pub q: i64,
    pub r: i64,
}
#[test]
fn bindgen_test_layout_secp256k1_modinv64_trans2x2() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_modinv64_trans2x2> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_modinv64_trans2x2>(),
        32usize,
        concat!("Size of: ", stringify!(secp256k1_modinv64_trans2x2))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_modinv64_trans2x2>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_modinv64_trans2x2))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).u) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_modinv64_trans2x2),
            "::",
            stringify!(u)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_modinv64_trans2x2),
            "::",
            stringify!(v)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).q) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_modinv64_trans2x2),
            "::",
            stringify!(q)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).r) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_modinv64_trans2x2),
            "::",
            stringify!(r)
        )
    );
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_const_modinfo_fe:
        secp256k1_modinv64_modinfo;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_const_modinfo_scalar:
        secp256k1_modinv64_modinfo;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_one: secp256k1_scalar;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_scalar_zero: secp256k1_scalar;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_const_lambda: secp256k1_scalar;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_ge_const_g: secp256k1_ge;
}
extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memcpy(
        __dst: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(
        __dst: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __len: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(
        __b: *mut ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __len: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcat(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __charset: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strncat(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __charset: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __charset: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strstr(
        __big: *const ::std::os::raw::c_char,
        __little: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(
        __str: *mut ::std::os::raw::c_char,
        __sep: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strxfrm(
        __s1: *mut ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtok_r(
        __str: *mut ::std::os::raw::c_char,
        __sep: *const ::std::os::raw::c_char,
        __lasts: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __strerrbuf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strdup(__s1: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn memccpy(
        __dst: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn stpcpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dst: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(
        __s1: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strnlen(__s1: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
pub type rsize_t = __darwin_size_t;
pub type errno_t = ::std::os::raw::c_int;
extern "C" {
    pub fn memset_s(
        __s: *mut ::std::os::raw::c_void,
        __smax: rsize_t,
        __c: ::std::os::raw::c_int,
        __n: rsize_t,
    ) -> errno_t;
}
extern "C" {
    pub fn memmem(
        __big: *const ::std::os::raw::c_void,
        __big_len: usize,
        __little: *const ::std::os::raw::c_void,
        __little_len: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset_pattern4(
        __b: *mut ::std::os::raw::c_void,
        __pattern4: *const ::std::os::raw::c_void,
        __len: usize,
    );
}
extern "C" {
    pub fn memset_pattern8(
        __b: *mut ::std::os::raw::c_void,
        __pattern8: *const ::std::os::raw::c_void,
        __len: usize,
    );
}
extern "C" {
    pub fn memset_pattern16(
        __b: *mut ::std::os::raw::c_void,
        __pattern16: *const ::std::os::raw::c_void,
        __len: usize,
    );
}
extern "C" {
    pub fn strcasestr(
        __big: *const ::std::os::raw::c_char,
        __little: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strnstr(
        __big: *const ::std::os::raw::c_char,
        __little: *const ::std::os::raw::c_char,
        __len: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlcat(
        __dst: *mut ::std::os::raw::c_char,
        __source: *const ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strlcpy(
        __dst: *mut ::std::os::raw::c_char,
        __source: *const ::std::os::raw::c_char,
        __size: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strmode(__mode: ::std::os::raw::c_int, __bp: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn swab(
        arg1: *const ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
        arg3: isize,
    );
}
extern "C" {
    pub fn timingsafe_bcmp(
        __b1: *const ::std::os::raw::c_void,
        __b2: *const ::std::os::raw::c_void,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strsignal_r(
        __sig: ::std::os::raw::c_int,
        __strsignalbuf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcmp(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
        arg3: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcopy(
        arg1: *const ::std::os::raw::c_void,
        arg2: *mut ::std::os::raw::c_void,
        arg3: usize,
    );
}
extern "C" {
    pub fn bzero(arg1: *mut ::std::os::raw::c_void, arg2: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn index(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp(
        arg1: *const ::std::os::raw::c_char,
        arg2: *const ::std::os::raw::c_char,
        arg3: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsl(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsll(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fls(arg1: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flsl(arg1: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn flsll(arg1: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_scratch_space_struct {
    #[doc = " guard against interpreting this object as other types"]
    pub magic: [::std::os::raw::c_uchar; 8usize],
    #[doc = " actual allocated data"]
    pub data: *mut ::std::os::raw::c_void,
    #[doc = " amount that has been allocated (i.e. `data + offset` is the next\n  available pointer)"]
    pub alloc_size: usize,
    #[doc = " maximum size available to allocate"]
    pub max_size: usize,
}
#[test]
fn bindgen_test_layout_secp256k1_scratch_space_struct() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_scratch_space_struct> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_scratch_space_struct>(),
        32usize,
        concat!("Size of: ", stringify!(secp256k1_scratch_space_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_scratch_space_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_scratch_space_struct))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).magic) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_scratch_space_struct),
            "::",
            stringify!(magic)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).data) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_scratch_space_struct),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).alloc_size) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_scratch_space_struct),
            "::",
            stringify!(alloc_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).max_size) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_scratch_space_struct),
            "::",
            stringify!(max_size)
        )
    );
}
pub type secp256k1_scratch = secp256k1_scratch_space_struct;
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecmult(
        r: *mut secp256k1_gej,
        a: *const secp256k1_gej,
        na: *const secp256k1_scalar,
        ng: *const secp256k1_scalar,
    );
}
pub type secp256k1_ecmult_multi_callback = ::std::option::Option<
    unsafe extern "C" fn(
        sc: *mut secp256k1_scalar,
        pt: *mut secp256k1_ge,
        idx: usize,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecmult_multi_var(
        error_callback: *const secp256k1_callback,
        scratch: *mut secp256k1_scratch,
        r: *mut secp256k1_gej,
        inp_g_sc: *const secp256k1_scalar,
        cb: secp256k1_ecmult_multi_callback,
        cbdata: *mut ::std::os::raw::c_void,
        n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_pre_g:
        [secp256k1_ge_storage; 8192usize];
}
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_pre_g_128:
        [secp256k1_ge_storage; 8192usize];
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_strauss_point_state {
    pub wnaf_na_1: [::std::os::raw::c_int; 129usize],
    pub wnaf_na_lam: [::std::os::raw::c_int; 129usize],
    pub bits_na_1: ::std::os::raw::c_int,
    pub bits_na_lam: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_secp256k1_strauss_point_state() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_strauss_point_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_strauss_point_state>(),
        1040usize,
        concat!("Size of: ", stringify!(secp256k1_strauss_point_state))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_strauss_point_state>(),
        4usize,
        concat!("Alignment of ", stringify!(secp256k1_strauss_point_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wnaf_na_1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_strauss_point_state),
            "::",
            stringify!(wnaf_na_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wnaf_na_lam) as usize - ptr as usize },
        516usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_strauss_point_state),
            "::",
            stringify!(wnaf_na_lam)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bits_na_1) as usize - ptr as usize },
        1032usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_strauss_point_state),
            "::",
            stringify!(bits_na_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bits_na_lam) as usize - ptr as usize },
        1036usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_strauss_point_state),
            "::",
            stringify!(bits_na_lam)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_strauss_state {
    pub aux: *mut secp256k1_fe,
    pub pre_a: *mut secp256k1_ge,
    pub ps: *mut secp256k1_strauss_point_state,
}
#[test]
fn bindgen_test_layout_secp256k1_strauss_state() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_strauss_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_strauss_state>(),
        24usize,
        concat!("Size of: ", stringify!(secp256k1_strauss_state))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_strauss_state>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_strauss_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).aux) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_strauss_state),
            "::",
            stringify!(aux)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).pre_a) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_strauss_state),
            "::",
            stringify!(pre_a)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ps) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_strauss_state),
            "::",
            stringify!(ps)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_pippenger_point_state {
    pub skew_na: ::std::os::raw::c_int,
    pub input_pos: usize,
}
#[test]
fn bindgen_test_layout_secp256k1_pippenger_point_state() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_pippenger_point_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_pippenger_point_state>(),
        16usize,
        concat!("Size of: ", stringify!(secp256k1_pippenger_point_state))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_pippenger_point_state>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_pippenger_point_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).skew_na) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_pippenger_point_state),
            "::",
            stringify!(skew_na)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).input_pos) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_pippenger_point_state),
            "::",
            stringify!(input_pos)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_pippenger_state {
    pub wnaf_na: *mut ::std::os::raw::c_int,
    pub ps: *mut secp256k1_pippenger_point_state,
}
#[test]
fn bindgen_test_layout_secp256k1_pippenger_state() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_pippenger_state> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_pippenger_state>(),
        16usize,
        concat!("Size of: ", stringify!(secp256k1_pippenger_state))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_pippenger_state>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_pippenger_state))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).wnaf_na) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_pippenger_state),
            "::",
            stringify!(wnaf_na)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ps) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_pippenger_state),
            "::",
            stringify!(ps)
        )
    );
}
pub type secp256k1_ecmult_multi_func = ::std::option::Option<
    unsafe extern "C" fn(
        error_callback: *const secp256k1_callback,
        arg1: *mut secp256k1_scratch,
        arg2: *mut secp256k1_gej,
        arg3: *const secp256k1_scalar,
        cb: secp256k1_ecmult_multi_callback,
        arg4: *mut ::std::os::raw::c_void,
        arg5: usize,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub static s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecmult_const_K: secp256k1_scalar;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_ecmult_gen_context {
    pub built: ::std::os::raw::c_int,
    pub scalar_offset: secp256k1_scalar,
    pub ge_offset: secp256k1_ge,
    pub proj_blind: secp256k1_fe,
}
#[test]
fn bindgen_test_layout_secp256k1_ecmult_gen_context() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_ecmult_gen_context> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_ecmult_gen_context>(),
        168usize,
        concat!("Size of: ", stringify!(secp256k1_ecmult_gen_context))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_ecmult_gen_context>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_ecmult_gen_context))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).built) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ecmult_gen_context),
            "::",
            stringify!(built)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).scalar_offset) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ecmult_gen_context),
            "::",
            stringify!(scalar_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).ge_offset) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ecmult_gen_context),
            "::",
            stringify!(ge_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).proj_blind) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ecmult_gen_context),
            "::",
            stringify!(proj_blind)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_sha256 {
    pub s: [u32; 8usize],
    pub buf: [::std::os::raw::c_uchar; 64usize],
    pub bytes: u64,
}
#[test]
fn bindgen_test_layout_secp256k1_sha256() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_sha256> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_sha256>(),
        104usize,
        concat!("Size of: ", stringify!(secp256k1_sha256))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_sha256>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_sha256))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).s) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_sha256),
            "::",
            stringify!(s)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).buf) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_sha256),
            "::",
            stringify!(buf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).bytes) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_sha256),
            "::",
            stringify!(bytes)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_hmac_sha256 {
    pub inner: secp256k1_sha256,
    pub outer: secp256k1_sha256,
}
#[test]
fn bindgen_test_layout_secp256k1_hmac_sha256() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_hmac_sha256> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_hmac_sha256>(),
        208usize,
        concat!("Size of: ", stringify!(secp256k1_hmac_sha256))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_hmac_sha256>(),
        8usize,
        concat!("Alignment of ", stringify!(secp256k1_hmac_sha256))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).inner) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_hmac_sha256),
            "::",
            stringify!(inner)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).outer) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_hmac_sha256),
            "::",
            stringify!(outer)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_rfc6979_hmac_sha256 {
    pub v: [::std::os::raw::c_uchar; 32usize],
    pub k: [::std::os::raw::c_uchar; 32usize],
    pub retry: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_secp256k1_rfc6979_hmac_sha256() {
    const UNINIT: ::std::mem::MaybeUninit<secp256k1_rfc6979_hmac_sha256> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<secp256k1_rfc6979_hmac_sha256>(),
        68usize,
        concat!("Size of: ", stringify!(secp256k1_rfc6979_hmac_sha256))
    );
    assert_eq!(
        ::std::mem::align_of::<secp256k1_rfc6979_hmac_sha256>(),
        4usize,
        concat!("Alignment of ", stringify!(secp256k1_rfc6979_hmac_sha256))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).v) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_rfc6979_hmac_sha256),
            "::",
            stringify!(v)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).k) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_rfc6979_hmac_sha256),
            "::",
            stringify!(k)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).retry) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_rfc6979_hmac_sha256),
            "::",
            stringify!(retry)
        )
    );
}
extern "C" {
    pub static mut s642c885b6102725e25623738529895a95addc4f4_secp256k1_ecmult_gen_prec_table:
        [[secp256k1_ge_storage; 32usize]; 11usize];
}
pub type __builtin_va_list = *mut ::std::os::raw::c_char;
pub type __uint128_t = u128;
