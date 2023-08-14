// @generated
/// Implement `DataProvider<NfkdInertV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_props_nfkdinert_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPS_NFKDINERT_V1: &'static <icu::properties::provider::NfkdInertV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::PropertyCodePointSetV1::InversionList(unsafe {
                #[allow(unused_unsafe)]
                icu::collections::codepointinvlist::CodePointInversionList::from_parts_unchecked(unsafe { zerovec::ZeroVec::from_bytes_unchecked(b"\0\0\0\0\xA0\0\0\0\xA1\0\0\0\xA8\0\0\0\xA9\0\0\0\xAA\0\0\0\xAB\0\0\0\xAF\0\0\0\xB0\0\0\0\xB2\0\0\0\xB6\0\0\0\xB8\0\0\0\xBB\0\0\0\xBC\0\0\0\xBF\0\0\0\xC0\0\0\0\xC6\0\0\0\xC7\0\0\0\xD0\0\0\0\xD1\0\0\0\xD7\0\0\0\xD9\0\0\0\xDE\0\0\0\xE0\0\0\0\xE6\0\0\0\xE7\0\0\0\xF0\0\0\0\xF1\0\0\0\xF7\0\0\0\xF9\0\0\0\xFE\0\0\0\xFF\0\0\0\x10\x01\0\0\x12\x01\0\0&\x01\0\0(\x01\0\x001\x01\0\x002\x01\0\08\x01\0\09\x01\0\0A\x01\0\0C\x01\0\0J\x01\0\0L\x01\0\0R\x01\0\0T\x01\0\0f\x01\0\0h\x01\0\0\x80\x01\0\0\xA0\x01\0\0\xA2\x01\0\0\xAF\x01\0\0\xB1\x01\0\0\xC4\x01\0\0\xDD\x01\0\0\xDE\x01\0\0\xE4\x01\0\0\xE6\x01\0\0\xF6\x01\0\0\xF8\x01\0\0\x1C\x02\0\0\x1E\x02\0\0 \x02\0\0&\x02\0\x004\x02\0\0\xB0\x02\0\0\xB9\x02\0\0\xD8\x02\0\0\xDE\x02\0\0\xE0\x02\0\0\xE5\x02\0\0\0\x03\0\0O\x03\0\0P\x03\0\0p\x03\0\0t\x03\0\0u\x03\0\0z\x03\0\0{\x03\0\0~\x03\0\0\x7F\x03\0\0\x84\x03\0\0\x8B\x03\0\0\x8C\x03\0\0\x8D\x03\0\0\x8E\x03\0\0\x91\x03\0\0\xAA\x03\0\0\xB1\x03\0\0\xCA\x03\0\0\xCF\x03\0\0\xD0\x03\0\0\xD7\x03\0\0\xF0\x03\0\0\xF3\x03\0\0\xF4\x03\0\0\xF6\x03\0\0\xF9\x03\0\0\xFA\x03\0\0\0\x04\0\0\x02\x04\0\0\x03\x04\0\0\x04\x04\0\0\x07\x04\0\0\x08\x04\0\0\x0C\x04\0\0\x0F\x04\0\0\x19\x04\0\0\x1A\x04\0\09\x04\0\0:\x04\0\0P\x04\0\0R\x04\0\0S\x04\0\0T\x04\0\0W\x04\0\0X\x04\0\0\\\x04\0\0_\x04\0\0v\x04\0\0x\x04\0\0\x83\x04\0\0\x88\x04\0\0\xC1\x04\0\0\xC3\x04\0\0\xD0\x04\0\0\xD4\x04\0\0\xD6\x04\0\0\xD8\x04\0\0\xDA\x04\0\0\xE0\x04\0\0\xE2\x04\0\0\xE8\x04\0\0\xEA\x04\0\0\xF6\x04\0\0\xF8\x04\0\0\xFA\x04\0\0\x87\x05\0\0\x88\x05\0\0\x91\x05\0\0\xBE\x05\0\0\xBF\x05\0\0\xC0\x05\0\0\xC1\x05\0\0\xC3\x05\0\0\xC4\x05\0\0\xC6\x05\0\0\xC7\x05\0\0\xC8\x05\0\0\x10\x06\0\0\x1B\x06\0\0\"\x06\0\0'\x06\0\0K\x06\0\0`\x06\0\0p\x06\0\0q\x06\0\0u\x06\0\0y\x06\0\0\xC0\x06\0\0\xC1\x06\0\0\xC2\x06\0\0\xC3\x06\0\0\xD3\x06\0\0\xD4\x06\0\0\xD6\x06\0\0\xDD\x06\0\0\xDF\x06\0\0\xE5\x06\0\0\xE7\x06\0\0\xE9\x06\0\0\xEA\x06\0\0\xEE\x06\0\0\x11\x07\0\0\x12\x07\0\x000\x07\0\0K\x07\0\0\xEB\x07\0\0\xF4\x07\0\0\xFD\x07\0\0\xFE\x07\0\0\x16\x08\0\0\x1A\x08\0\0\x1B\x08\0\0$\x08\0\0%\x08\0\0(\x08\0\0)\x08\0\0.\x08\0\0Y\x08\0\0\\\x08\0\0\x98\x08\0\0\xA0\x08\0\0\xCA\x08\0\0\xE2\x08\0\0\xE3\x08\0\0\0\t\0\0)\t\0\0*\t\0\x001\t\0\x002\t\0\x004\t\0\x005\t\0\0<\t\0\0=\t\0\0M\t\0\0N\t\0\0Q\t\0\0U\t\0\0X\t\0\0`\t\0\0\xBC\t\0\0\xBD\t\0\0\xCB\t\0\0\xCE\t\0\0\xDC\t\0\0\xDE\t\0\0\xDF\t\0\0\xE0\t\0\0\xFE\t\0\0\xFF\t\0\x003\n\0\x004\n\0\x006\n\0\x007\n\0\0<\n\0\0=\n\0\0M\n\0\0N\n\0\0Y\n\0\0\\\n\0\0^\n\0\0_\n\0\0\xBC\n\0\0\xBD\n\0\0\xCD\n\0\0\xCE\n\0\0<\x0B\0\0=\x0B\0\0H\x0B\0\0I\x0B\0\0K\x0B\0\0N\x0B\0\0\\\x0B\0\0^\x0B\0\0\x94\x0B\0\0\x95\x0B\0\0\xCA\x0B\0\0\xCE\x0B\0\0<\x0C\0\0=\x0C\0\0H\x0C\0\0I\x0C\0\0M\x0C\0\0N\x0C\0\0U\x0C\0\0W\x0C\0\0\xBC\x0C\0\0\xBD\x0C\0\0\xC0\x0C\0\0\xC1\x0C\0\0\xC7\x0C\0\0\xC9\x0C\0\0\xCA\x0C\0\0\xCC\x0C\0\0\xCD\x0C\0\0\xCE\x0C\0\0;\r\0\0=\r\0\0J\r\0\0N\r\0\0\xCA\r\0\0\xCB\r\0\0\xDA\r\0\0\xDB\r\0\0\xDC\r\0\0\xDF\r\0\x003\x0E\0\x004\x0E\0\08\x0E\0\0;\x0E\0\0H\x0E\0\0L\x0E\0\0\xB3\x0E\0\0\xB4\x0E\0\0\xB8\x0E\0\0\xBB\x0E\0\0\xC8\x0E\0\0\xCC\x0E\0\0\xDC\x0E\0\0\xDE\x0E\0\0\x0C\x0F\0\0\r\x0F\0\0\x18\x0F\0\0\x1A\x0F\0\x005\x0F\0\x006\x0F\0\x007\x0F\0\08\x0F\0\09\x0F\0\0:\x0F\0\0C\x0F\0\0D\x0F\0\0M\x0F\0\0N\x0F\0\0R\x0F\0\0S\x0F\0\0W\x0F\0\0X\x0F\0\0\\\x0F\0\0]\x0F\0\0i\x0F\0\0j\x0F\0\0q\x0F\0\0~\x0F\0\0\x80\x0F\0\0\x85\x0F\0\0\x86\x0F\0\0\x88\x0F\0\0\x93\x0F\0\0\x94\x0F\0\0\x9D\x0F\0\0\x9E\x0F\0\0\xA2\x0F\0\0\xA3\x0F\0\0\xA7\x0F\0\0\xA8\x0F\0\0\xAC\x0F\0\0\xAD\x0F\0\0\xB9\x0F\0\0\xBA\x0F\0\0\xC6\x0F\0\0\xC7\x0F\0\0&\x10\0\0'\x10\0\x007\x10\0\08\x10\0\09\x10\0\0;\x10\0\0\x8D\x10\0\0\x8E\x10\0\0\xFC\x10\0\0\xFD\x10\0\0]\x13\0\0`\x13\0\0\x14\x17\0\0\x16\x17\0\x004\x17\0\x005\x17\0\0\xD2\x17\0\0\xD3\x17\0\0\xDD\x17\0\0\xDE\x17\0\0\xA9\x18\0\0\xAA\x18\0\09\x19\0\0<\x19\0\0\x17\x1A\0\0\x19\x1A\0\0`\x1A\0\0a\x1A\0\0u\x1A\0\0}\x1A\0\0\x7F\x1A\0\0\x80\x1A\0\0\xB0\x1A\0\0\xBE\x1A\0\0\xBF\x1A\0\0\xCF\x1A\0\0\x06\x1B\0\0\x07\x1B\0\0\x08\x1B\0\0\t\x1B\0\0\n\x1B\0\0\x0B\x1B\0\0\x0C\x1B\0\0\r\x1B\0\0\x0E\x1B\0\0\x0F\x1B\0\0\x12\x1B\0\0\x13\x1B\0\x004\x1B\0\x005\x1B\0\0;\x1B\0\0<\x1B\0\0=\x1B\0\0>\x1B\0\0@\x1B\0\0B\x1B\0\0C\x1B\0\0E\x1B\0\0k\x1B\0\0t\x1B\0\0\xAA\x1B\0\0\xAC\x1B\0\0\xE6\x1B\0\0\xE7\x1B\0\0\xF2\x1B\0\0\xF4\x1B\0\x007\x1C\0\08\x1C\0\0\xD0\x1C\0\0\xD3\x1C\0\0\xD4\x1C\0\0\xE1\x1C\0\0\xE2\x1C\0\0\xE9\x1C\0\0\xED\x1C\0\0\xEE\x1C\0\0\xF4\x1C\0\0\xF5\x1C\0\0\xF8\x1C\0\0\xFA\x1C\0\0,\x1D\0\0/\x1D\0\x000\x1D\0\0;\x1D\0\0<\x1D\0\0N\x1D\0\0O\x1D\0\0k\x1D\0\0x\x1D\0\0y\x1D\0\0\x9B\x1D\0\0\x9C\x1E\0\0\xA0\x1E\0\0\xFA\x1E\0\0\0\x1F\0\0\x16\x1F\0\0\x18\x1F\0\0\x1E\x1F\0\0 \x1F\0\0F\x1F\0\0H\x1F\0\0N\x1F\0\0P\x1F\0\0X\x1F\0\0Y\x1F\0\0Z\x1F\0\0[\x1F\0\0\\\x1F\0\0]\x1F\0\0^\x1F\0\0_\x1F\0\0~\x1F\0\0\x80\x1F\0\0\xB5\x1F\0\0\xB6\x1F\0\0\xC5\x1F\0\0\xC6\x1F\0\0\xD4\x1F\0\0\xD6\x1F\0\0\xDC\x1F\0\0\xDD\x1F\0\0\xF0\x1F\0\0\xF2\x1F\0\0\xF5\x1F\0\0\xF6\x1F\0\0\xFF\x1F\0\0\0 \0\0\x0B \0\0\x11 \0\0\x12 \0\0\x17 \0\0\x18 \0\0$ \0\0' \0\0/ \0\x000 \0\x003 \0\x005 \0\x006 \0\08 \0\0< \0\0= \0\0> \0\0? \0\0G \0\0J \0\0W \0\0X \0\0_ \0\0` \0\0p \0\0r \0\0t \0\0\x8F \0\0\x90 \0\0\x9D \0\0\xA8 \0\0\xA9 \0\0\xD0 \0\0\xDD \0\0\xE1 \0\0\xE2 \0\0\xE5 \0\0\xF1 \0\0\0!\0\0\x04!\0\0\x05!\0\0\x08!\0\0\t!\0\0\x14!\0\0\x15!\0\0\x17!\0\0\x19!\0\0\x1E!\0\0 !\0\0#!\0\0$!\0\0%!\0\0&!\0\0'!\0\0(!\0\0)!\0\0*!\0\0.!\0\0/!\0\x002!\0\x003!\0\0:!\0\0;!\0\0A!\0\0E!\0\0J!\0\0P!\0\0\x80!\0\0\x89!\0\0\x8A!\0\0\x9A!\0\0\x9C!\0\0\xAE!\0\0\xAF!\0\0\xCD!\0\0\xD0!\0\0\x04\"\0\0\x05\"\0\0\t\"\0\0\n\"\0\0\x0C\"\0\0\r\"\0\0$\"\0\0%\"\0\0&\"\0\0'\"\0\0,\"\0\0.\"\0\0/\"\0\x001\"\0\0A\"\0\0B\"\0\0D\"\0\0E\"\0\0G\"\0\0H\"\0\0I\"\0\0J\"\0\0`\"\0\0a\"\0\0b\"\0\0c\"\0\0m\"\0\0r\"\0\0t\"\0\0v\"\0\0x\"\0\0z\"\0\0\x80\"\0\0\x82\"\0\0\x84\"\0\0\x86\"\0\0\x88\"\0\0\x8A\"\0\0\xAC\"\0\0\xB0\"\0\0\xE0\"\0\0\xE4\"\0\0\xEA\"\0\0\xEE\"\0\0)#\0\0+#\0\0`$\0\0\xEB$\0\0\x0C*\0\0\r*\0\0t*\0\0w*\0\0\xDC*\0\0\xDD*\0\0|,\0\0~,\0\0\xEF,\0\0\xF2,\0\0o-\0\0p-\0\0\x7F-\0\0\x80-\0\0\xE0-\0\0\0.\0\0\x9F.\0\0\xA0.\0\0\xF3.\0\0\xF4.\0\0\0/\0\0\xD6/\0\0\x000\0\0\x010\0\0*0\0\x0000\0\x0060\0\x0070\0\080\0\0;0\0\0L0\0\0M0\0\0N0\0\0O0\0\0P0\0\0Q0\0\0R0\0\0S0\0\0T0\0\0U0\0\0V0\0\0W0\0\0X0\0\0Y0\0\0Z0\0\0[0\0\0\\0\0\0]0\0\0^0\0\0_0\0\0`0\0\0a0\0\0b0\0\0c0\0\0e0\0\0f0\0\0g0\0\0h0\0\0i0\0\0j0\0\0p0\0\0r0\0\0s0\0\0u0\0\0v0\0\0x0\0\0y0\0\0{0\0\0|0\0\0~0\0\0\x940\0\0\x950\0\0\x990\0\0\x9D0\0\0\x9E0\0\0\xA00\0\0\xAC0\0\0\xAD0\0\0\xAE0\0\0\xAF0\0\0\xB00\0\0\xB10\0\0\xB20\0\0\xB30\0\0\xB40\0\0\xB50\0\0\xB60\0\0\xB70\0\0\xB80\0\0\xB90\0\0\xBA0\0\0\xBB0\0\0\xBC0\0\0\xBD0\0\0\xBE0\0\0\xBF0\0\0\xC00\0\0\xC10\0\0\xC20\0\0\xC30\0\0\xC50\0\0\xC60\0\0\xC70\0\0\xC80\0\0\xC90\0\0\xCA0\0\0\xD00\0\0\xD20\0\0\xD30\0\0\xD50\0\0\xD60\0\0\xD80\0\0\xD90\0\0\xDB0\0\0\xDC0\0\0\xDE0\0\0\xF40\0\0\xF50\0\0\xF70\0\0\xFB0\0\0\xFE0\0\0\x001\0\x0011\0\0\x8F1\0\0\x921\0\0\xA01\0\0\x002\0\0\x1F2\0\0 2\0\0H2\0\0P2\0\0\x7F2\0\0\x802\0\0\x004\0\0o\xA6\0\0p\xA6\0\0t\xA6\0\0~\xA6\0\0\x9C\xA6\0\0\xA0\xA6\0\0\xF0\xA6\0\0\xF2\xA6\0\0p\xA7\0\0q\xA7\0\0\xF2\xA7\0\0\xF5\xA7\0\0\xF8\xA7\0\0\xFA\xA7\0\0\x06\xA8\0\0\x07\xA8\0\0,\xA8\0\0-\xA8\0\0\xC4\xA8\0\0\xC5\xA8\0\0\xE0\xA8\0\0\xF2\xA8\0\0+\xA9\0\0.\xA9\0\0S\xA9\0\0T\xA9\0\0\xB3\xA9\0\0\xB4\xA9\0\0\xC0\xA9\0\0\xC1\xA9\0\0\xB0\xAA\0\0\xB1\xAA\0\0\xB2\xAA\0\0\xB5\xAA\0\0\xB7\xAA\0\0\xB9\xAA\0\0\xBE\xAA\0\0\xC0\xAA\0\0\xC1\xAA\0\0\xC2\xAA\0\0\xF6\xAA\0\0\xF7\xAA\0\0\\\xAB\0\0`\xAB\0\0i\xAB\0\0j\xAB\0\0\xED\xAB\0\0\xEE\xAB\0\0\0\xAC\0\0\xA4\xD7\0\0\0\xF9\0\0\x0E\xFA\0\0\x10\xFA\0\0\x11\xFA\0\0\x12\xFA\0\0\x13\xFA\0\0\x15\xFA\0\0\x1F\xFA\0\0 \xFA\0\0!\xFA\0\0\"\xFA\0\0#\xFA\0\0%\xFA\0\0'\xFA\0\0*\xFA\0\0n\xFA\0\0p\xFA\0\0\xDA\xFA\0\0\0\xFB\0\0\x07\xFB\0\0\x13\xFB\0\0\x18\xFB\0\0\x1D\xFB\0\x007\xFB\0\08\xFB\0\0=\xFB\0\0>\xFB\0\0?\xFB\0\0@\xFB\0\0B\xFB\0\0C\xFB\0\0E\xFB\0\0F\xFB\0\0\xB2\xFB\0\0\xD3\xFB\0\0>\xFD\0\0P\xFD\0\0\x90\xFD\0\0\x92\xFD\0\0\xC8\xFD\0\0\xF0\xFD\0\0\xFD\xFD\0\0\x10\xFE\0\0\x1A\xFE\0\0 \xFE\0\0E\xFE\0\0G\xFE\0\0S\xFE\0\0T\xFE\0\0g\xFE\0\0h\xFE\0\0l\xFE\0\0p\xFE\0\0s\xFE\0\0t\xFE\0\0u\xFE\0\0v\xFE\0\0\xFD\xFE\0\0\x01\xFF\0\0\xBF\xFF\0\0\xC2\xFF\0\0\xC8\xFF\0\0\xCA\xFF\0\0\xD0\xFF\0\0\xD2\xFF\0\0\xD8\xFF\0\0\xDA\xFF\0\0\xDD\xFF\0\0\xE0\xFF\0\0\xE7\xFF\0\0\xE8\xFF\0\0\xEF\xFF\0\0\xFD\x01\x01\0\xFE\x01\x01\0\xE0\x02\x01\0\xE1\x02\x01\0v\x03\x01\0{\x03\x01\0\x81\x07\x01\0\x86\x07\x01\0\x87\x07\x01\0\xB1\x07\x01\0\xB2\x07\x01\0\xBB\x07\x01\0\r\n\x01\0\x0E\n\x01\0\x0F\n\x01\0\x10\n\x01\08\n\x01\0;\n\x01\0?\n\x01\0@\n\x01\0\xE5\n\x01\0\xE7\n\x01\0$\r\x01\0(\r\x01\0\xAB\x0E\x01\0\xAD\x0E\x01\0\xFD\x0E\x01\0\0\x0F\x01\0F\x0F\x01\0Q\x0F\x01\0\x82\x0F\x01\0\x86\x0F\x01\0F\x10\x01\0G\x10\x01\0p\x10\x01\0q\x10\x01\0\x7F\x10\x01\0\x80\x10\x01\0\x9A\x10\x01\0\x9B\x10\x01\0\x9C\x10\x01\0\x9D\x10\x01\0\xAB\x10\x01\0\xAC\x10\x01\0\xB9\x10\x01\0\xBB\x10\x01\0\0\x11\x01\0\x03\x11\x01\0.\x11\x01\x000\x11\x01\x003\x11\x01\x005\x11\x01\0s\x11\x01\0t\x11\x01\0\xC0\x11\x01\0\xC1\x11\x01\0\xCA\x11\x01\0\xCB\x11\x01\x005\x12\x01\x007\x12\x01\0\xE9\x12\x01\0\xEB\x12\x01\0;\x13\x01\0=\x13\x01\0K\x13\x01\0N\x13\x01\0f\x13\x01\0m\x13\x01\0p\x13\x01\0u\x13\x01\0B\x14\x01\0C\x14\x01\0F\x14\x01\0G\x14\x01\0^\x14\x01\0_\x14\x01\0\xBB\x14\x01\0\xBD\x14\x01\0\xBE\x14\x01\0\xBF\x14\x01\0\xC2\x14\x01\0\xC4\x14\x01\0\xBA\x15\x01\0\xBC\x15\x01\0\xBF\x15\x01\0\xC1\x15\x01\0?\x16\x01\0@\x16\x01\0\xB6\x16\x01\0\xB8\x16\x01\0+\x17\x01\0,\x17\x01\09\x18\x01\0;\x18\x01\08\x19\x01\09\x19\x01\0=\x19\x01\0?\x19\x01\0C\x19\x01\0D\x19\x01\0\xE0\x19\x01\0\xE1\x19\x01\x004\x1A\x01\x005\x1A\x01\0G\x1A\x01\0H\x1A\x01\0\x99\x1A\x01\0\x9A\x1A\x01\0?\x1C\x01\0@\x1C\x01\0B\x1D\x01\0C\x1D\x01\0D\x1D\x01\0F\x1D\x01\0\x97\x1D\x01\0\x98\x1D\x01\0A\x1F\x01\0C\x1F\x01\0\xF0j\x01\0\xF5j\x01\x000k\x01\x007k\x01\0\xF0o\x01\0\xF2o\x01\0\x9E\xBC\x01\0\x9F\xBC\x01\0^\xD1\x01\0j\xD1\x01\0m\xD1\x01\0s\xD1\x01\0{\xD1\x01\0\x83\xD1\x01\0\x85\xD1\x01\0\x8C\xD1\x01\0\xAA\xD1\x01\0\xAE\xD1\x01\0\xBB\xD1\x01\0\xC1\xD1\x01\0B\xD2\x01\0E\xD2\x01\0\0\xD4\x01\0U\xD4\x01\0V\xD4\x01\0\x9D\xD4\x01\0\x9E\xD4\x01\0\xA0\xD4\x01\0\xA2\xD4\x01\0\xA3\xD4\x01\0\xA5\xD4\x01\0\xA7\xD4\x01\0\xA9\xD4\x01\0\xAD\xD4\x01\0\xAE\xD4\x01\0\xBA\xD4\x01\0\xBB\xD4\x01\0\xBC\xD4\x01\0\xBD\xD4\x01\0\xC4\xD4\x01\0\xC5\xD4\x01\0\x06\xD5\x01\0\x07\xD5\x01\0\x0B\xD5\x01\0\r\xD5\x01\0\x15\xD5\x01\0\x16\xD5\x01\0\x1D\xD5\x01\0\x1E\xD5\x01\0:\xD5\x01\0;\xD5\x01\0?\xD5\x01\0@\xD5\x01\0E\xD5\x01\0F\xD5\x01\0G\xD5\x01\0J\xD5\x01\0Q\xD5\x01\0R\xD5\x01\0\xA6\xD6\x01\0\xA8\xD6\x01\0\xCC\xD7\x01\0\xCE\xD7\x01\0\0\xD8\x01\0\0\xE0\x01\0\x07\xE0\x01\0\x08\xE0\x01\0\x19\xE0\x01\0\x1B\xE0\x01\0\"\xE0\x01\0#\xE0\x01\0%\xE0\x01\0&\xE0\x01\0+\xE0\x01\x000\xE0\x01\0n\xE0\x01\0\x8F\xE0\x01\0\x90\xE0\x01\x000\xE1\x01\x007\xE1\x01\0\xAE\xE2\x01\0\xAF\xE2\x01\0\xEC\xE2\x01\0\xF0\xE2\x01\0\xEC\xE4\x01\0\xF0\xE4\x01\0\xD0\xE8\x01\0\xD7\xE8\x01\0D\xE9\x01\0K\xE9\x01\0\0\xEE\x01\0\x04\xEE\x01\0\x05\xEE\x01\0 \xEE\x01\0!\xEE\x01\0#\xEE\x01\0$\xEE\x01\0%\xEE\x01\0'\xEE\x01\0(\xEE\x01\0)\xEE\x01\x003\xEE\x01\x004\xEE\x01\08\xEE\x01\09\xEE\x01\0:\xEE\x01\0;\xEE\x01\0<\xEE\x01\0B\xEE\x01\0C\xEE\x01\0G\xEE\x01\0H\xEE\x01\0I\xEE\x01\0J\xEE\x01\0K\xEE\x01\0L\xEE\x01\0M\xEE\x01\0P\xEE\x01\0Q\xEE\x01\0S\xEE\x01\0T\xEE\x01\0U\xEE\x01\0W\xEE\x01\0X\xEE\x01\0Y\xEE\x01\0Z\xEE\x01\0[\xEE\x01\0\\\xEE\x01\0]\xEE\x01\0^\xEE\x01\0_\xEE\x01\0`\xEE\x01\0a\xEE\x01\0c\xEE\x01\0d\xEE\x01\0e\xEE\x01\0g\xEE\x01\0k\xEE\x01\0l\xEE\x01\0s\xEE\x01\0t\xEE\x01\0x\xEE\x01\0y\xEE\x01\0}\xEE\x01\0~\xEE\x01\0\x7F\xEE\x01\0\x80\xEE\x01\0\x8A\xEE\x01\0\x8B\xEE\x01\0\x9C\xEE\x01\0\xA1\xEE\x01\0\xA4\xEE\x01\0\xA5\xEE\x01\0\xAA\xEE\x01\0\xAB\xEE\x01\0\xBC\xEE\x01\0\0\xF1\x01\0\x0B\xF1\x01\0\x10\xF1\x01\0/\xF1\x01\x000\xF1\x01\0P\xF1\x01\0j\xF1\x01\0m\xF1\x01\0\x90\xF1\x01\0\x91\xF1\x01\0\0\xF2\x01\0\x03\xF2\x01\0\x10\xF2\x01\0<\xF2\x01\0@\xF2\x01\0I\xF2\x01\0P\xF2\x01\0R\xF2\x01\0\xF0\xFB\x01\0\xFA\xFB\x01\0\0\xF8\x02\0\x1E\xFA\x02\0\0\0\x11\0") }, 1096165u32)
            });
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::properties::provider::NfkdInertV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::NfkdInertV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPS_NFKDINERT_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::NfkdInertV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
