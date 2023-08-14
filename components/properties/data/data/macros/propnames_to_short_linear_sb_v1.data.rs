// @generated
/// Implement `DataProvider<SentenceBreakValueToShortNameV1Marker>` on the given struct using the data
/// hardcoded in this file. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_propnames_to_short_linear_sb_v1 {
    ($ provider : path) => {
        #[clippy::msrv = "1.65"]
        impl $provider {
            #[doc(hidden)]
            pub const SINGLETON_PROPNAMES_TO_SHORT_LINEAR_SB_V1: &'static <icu::properties::provider::SentenceBreakValueToShortNameV1Marker as icu_provider::DataMarker>::Yokeable = &icu::properties::provider::names::PropertyEnumToValueNameLinearMapV1 { map: unsafe { zerovec::VarZeroVec::from_bytes_unchecked(b"\x0E\0\0\0\0\0\x02\0\x04\0\x06\0\x08\0\n\0\x0C\0\x0E\0\x10\0\x12\0\x14\0\x16\0\x18\0\x1A\0XXATCLFOLONULESESPSTUPCREXLF") } };
        }
        #[clippy::msrv = "1.65"]
        impl icu_provider::DataProvider<icu::properties::provider::SentenceBreakValueToShortNameV1Marker> for $provider {
            fn load(&self, req: icu_provider::DataRequest) -> Result<icu_provider::DataResponse<icu::properties::provider::SentenceBreakValueToShortNameV1Marker>, icu_provider::DataError> {
                if req.locale.is_empty() {
                    Ok(icu_provider::DataResponse { payload: Some(icu_provider::DataPayload::from_static_ref(Self::SINGLETON_PROPNAMES_TO_SHORT_LINEAR_SB_V1)), metadata: Default::default() })
                } else {
                    Err(icu_provider::DataErrorKind::ExtraneousLocale.with_req(<icu::properties::provider::SentenceBreakValueToShortNameV1Marker as icu_provider::KeyedDataMarker>::KEY, req))
                }
            }
        }
    };
}
