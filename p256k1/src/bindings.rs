#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
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
pub const _FEATURES_H: u32 = 1;
pub const _DEFAULT_SOURCE: u32 = 1;
pub const __GLIBC_USE_ISOC2X: u32 = 0;
pub const __USE_ISOC11: u32 = 1;
pub const __USE_ISOC99: u32 = 1;
pub const __USE_ISOC95: u32 = 1;
pub const __USE_POSIX_IMPLICITLY: u32 = 1;
pub const _POSIX_SOURCE: u32 = 1;
pub const _POSIX_C_SOURCE: u32 = 200809;
pub const __USE_POSIX: u32 = 1;
pub const __USE_POSIX2: u32 = 1;
pub const __USE_POSIX199309: u32 = 1;
pub const __USE_POSIX199506: u32 = 1;
pub const __USE_XOPEN2K: u32 = 1;
pub const __USE_XOPEN2K8: u32 = 1;
pub const _ATFILE_SOURCE: u32 = 1;
pub const __WORDSIZE: u32 = 64;
pub const __WORDSIZE_TIME64_COMPAT32: u32 = 1;
pub const __SYSCALL_WORDSIZE: u32 = 64;
pub const __TIMESIZE: u32 = 64;
pub const __USE_MISC: u32 = 1;
pub const __USE_ATFILE: u32 = 1;
pub const __USE_FORTIFY_LEVEL: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_GETS: u32 = 0;
pub const __GLIBC_USE_DEPRECATED_SCANF: u32 = 0;
pub const _STDC_PREDEF_H: u32 = 1;
pub const __STDC_IEC_559__: u32 = 1;
pub const __STDC_IEC_60559_BFP__: u32 = 201404;
pub const __STDC_IEC_559_COMPLEX__: u32 = 1;
pub const __STDC_IEC_60559_COMPLEX__: u32 = 201404;
pub const __STDC_ISO_10646__: u32 = 201706;
pub const __GNU_LIBRARY__: u32 = 6;
pub const __GLIBC__: u32 = 2;
pub const __GLIBC_MINOR__: u32 = 37;
pub const _SYS_CDEFS_H: u32 = 1;
pub const __glibc_c99_flexarr_available: u32 = 1;
pub const __LDOUBLE_REDIRECTS_TO_FLOAT128_ABI: u32 = 0;
pub const __HAVE_GENERIC_SELECTION: u32 = 1;
pub const __GLIBC_USE_LIB_EXT2: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_BFP_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT: u32 = 0;
pub const __GLIBC_USE_IEC_60559_FUNCS_EXT_C2X: u32 = 0;
pub const __GLIBC_USE_IEC_60559_TYPES_EXT: u32 = 0;
pub const _STDLIB_H: u32 = 1;
pub const WNOHANG: u32 = 1;
pub const WUNTRACED: u32 = 2;
pub const WSTOPPED: u32 = 2;
pub const WEXITED: u32 = 4;
pub const WCONTINUED: u32 = 8;
pub const WNOWAIT: u32 = 16777216;
pub const __WNOTHREAD: u32 = 536870912;
pub const __WALL: u32 = 1073741824;
pub const __WCLONE: u32 = 2147483648;
pub const __W_CONTINUED: u32 = 65535;
pub const __WCOREFLAG: u32 = 128;
pub const __HAVE_FLOAT128: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128: u32 = 0;
pub const __HAVE_FLOAT64X: u32 = 1;
pub const __HAVE_FLOAT64X_LONG_DOUBLE: u32 = 1;
pub const __HAVE_FLOAT16: u32 = 0;
pub const __HAVE_FLOAT32: u32 = 1;
pub const __HAVE_FLOAT64: u32 = 1;
pub const __HAVE_FLOAT32X: u32 = 1;
pub const __HAVE_FLOAT128X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT16: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT32X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT64X: u32 = 0;
pub const __HAVE_DISTINCT_FLOAT128X: u32 = 0;
pub const __HAVE_FLOATN_NOT_TYPEDEF: u32 = 0;
pub const __ldiv_t_defined: u32 = 1;
pub const __lldiv_t_defined: u32 = 1;
pub const RAND_MAX: u32 = 2147483647;
pub const EXIT_FAILURE: u32 = 1;
pub const EXIT_SUCCESS: u32 = 0;
pub const _SYS_TYPES_H: u32 = 1;
pub const _BITS_TYPES_H: u32 = 1;
pub const _BITS_TYPESIZES_H: u32 = 1;
pub const __OFF_T_MATCHES_OFF64_T: u32 = 1;
pub const __INO_T_MATCHES_INO64_T: u32 = 1;
pub const __RLIM_T_MATCHES_RLIM64_T: u32 = 1;
pub const __STATFS_MATCHES_STATFS64: u32 = 1;
pub const __KERNEL_OLD_TIMEVAL_MATCHES_TIMEVAL64: u32 = 1;
pub const __FD_SETSIZE: u32 = 1024;
pub const _BITS_TIME64_H: u32 = 1;
pub const __clock_t_defined: u32 = 1;
pub const __clockid_t_defined: u32 = 1;
pub const __time_t_defined: u32 = 1;
pub const __timer_t_defined: u32 = 1;
pub const _BITS_STDINT_INTN_H: u32 = 1;
pub const __BIT_TYPES_DEFINED__: u32 = 1;
pub const _ENDIAN_H: u32 = 1;
pub const _BITS_ENDIAN_H: u32 = 1;
pub const __LITTLE_ENDIAN: u32 = 1234;
pub const __BIG_ENDIAN: u32 = 4321;
pub const __PDP_ENDIAN: u32 = 3412;
pub const _BITS_ENDIANNESS_H: u32 = 1;
pub const __BYTE_ORDER: u32 = 1234;
pub const __FLOAT_WORD_ORDER: u32 = 1234;
pub const LITTLE_ENDIAN: u32 = 1234;
pub const BIG_ENDIAN: u32 = 4321;
pub const PDP_ENDIAN: u32 = 3412;
pub const BYTE_ORDER: u32 = 1234;
pub const _BITS_BYTESWAP_H: u32 = 1;
pub const _BITS_UINTN_IDENTITY_H: u32 = 1;
pub const _SYS_SELECT_H: u32 = 1;
pub const __sigset_t_defined: u32 = 1;
pub const __timeval_defined: u32 = 1;
pub const _STRUCT_TIMESPEC: u32 = 1;
pub const FD_SETSIZE: u32 = 1024;
pub const _BITS_PTHREADTYPES_COMMON_H: u32 = 1;
pub const _THREAD_SHARED_TYPES_H: u32 = 1;
pub const _BITS_PTHREADTYPES_ARCH_H: u32 = 1;
pub const __SIZEOF_PTHREAD_MUTEX_T: u32 = 40;
pub const __SIZEOF_PTHREAD_ATTR_T: u32 = 56;
pub const __SIZEOF_PTHREAD_RWLOCK_T: u32 = 56;
pub const __SIZEOF_PTHREAD_BARRIER_T: u32 = 32;
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_COND_T: u32 = 48;
pub const __SIZEOF_PTHREAD_CONDATTR_T: u32 = 4;
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: u32 = 8;
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: u32 = 4;
pub const _THREAD_MUTEX_INTERNAL_H: u32 = 1;
pub const __PTHREAD_MUTEX_HAVE_PREV: u32 = 1;
pub const __have_pthread_attr_t: u32 = 1;
pub const _ALLOCA_H: u32 = 1;
pub const _STDINT_H: u32 = 1;
pub const _BITS_WCHAR_H: u32 = 1;
pub const _BITS_STDINT_UINTN_H: u32 = 1;
pub const INT8_MIN: i32 = -128;
pub const INT16_MIN: i32 = -32768;
pub const INT32_MIN: i32 = -2147483648;
pub const INT8_MAX: u32 = 127;
pub const INT16_MAX: u32 = 32767;
pub const INT32_MAX: u32 = 2147483647;
pub const UINT8_MAX: u32 = 255;
pub const UINT16_MAX: u32 = 65535;
pub const UINT32_MAX: u32 = 4294967295;
pub const INT_LEAST8_MIN: i32 = -128;
pub const INT_LEAST16_MIN: i32 = -32768;
pub const INT_LEAST32_MIN: i32 = -2147483648;
pub const INT_LEAST8_MAX: u32 = 127;
pub const INT_LEAST16_MAX: u32 = 32767;
pub const INT_LEAST32_MAX: u32 = 2147483647;
pub const UINT_LEAST8_MAX: u32 = 255;
pub const UINT_LEAST16_MAX: u32 = 65535;
pub const UINT_LEAST32_MAX: u32 = 4294967295;
pub const INT_FAST8_MIN: i32 = -128;
pub const INT_FAST16_MIN: i64 = -9223372036854775808;
pub const INT_FAST32_MIN: i64 = -9223372036854775808;
pub const INT_FAST8_MAX: u32 = 127;
pub const INT_FAST16_MAX: u64 = 9223372036854775807;
pub const INT_FAST32_MAX: u64 = 9223372036854775807;
pub const UINT_FAST8_MAX: u32 = 255;
pub const UINT_FAST16_MAX: i32 = -1;
pub const UINT_FAST32_MAX: i32 = -1;
pub const INTPTR_MIN: i64 = -9223372036854775808;
pub const INTPTR_MAX: u64 = 9223372036854775807;
pub const UINTPTR_MAX: i32 = -1;
pub const PTRDIFF_MIN: i64 = -9223372036854775808;
pub const PTRDIFF_MAX: u64 = 9223372036854775807;
pub const SIG_ATOMIC_MIN: i32 = -2147483648;
pub const SIG_ATOMIC_MAX: u32 = 2147483647;
pub const SIZE_MAX: i32 = -1;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 4294967295;
pub const _STDIO_H: u32 = 1;
pub const _____fpos_t_defined: u32 = 1;
pub const ____mbstate_t_defined: u32 = 1;
pub const _____fpos64_t_defined: u32 = 1;
pub const ____FILE_defined: u32 = 1;
pub const __FILE_defined: u32 = 1;
pub const __struct_FILE_defined: u32 = 1;
pub const _IO_EOF_SEEN: u32 = 16;
pub const _IO_ERR_SEEN: u32 = 32;
pub const _IO_USER_LOCK: u32 = 32768;
pub const _IOFBF: u32 = 0;
pub const _IOLBF: u32 = 1;
pub const _IONBF: u32 = 2;
pub const BUFSIZ: u32 = 8192;
pub const EOF: i32 = -1;
pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const P_tmpdir: &[u8; 5usize] = b"/tmp\0";
pub const _BITS_STDIO_LIM_H: u32 = 1;
pub const L_tmpnam: u32 = 20;
pub const TMP_MAX: u32 = 238328;
pub const FILENAME_MAX: u32 = 4096;
pub const L_ctermid: u32 = 9;
pub const FOPEN_MAX: u32 = 16;
pub const _LIBC_LIMITS_H_: u32 = 1;
pub const MB_LEN_MAX: u32 = 16;
pub const _BITS_POSIX1_LIM_H: u32 = 1;
pub const _POSIX_AIO_LISTIO_MAX: u32 = 2;
pub const _POSIX_AIO_MAX: u32 = 1;
pub const _POSIX_ARG_MAX: u32 = 4096;
pub const _POSIX_CHILD_MAX: u32 = 25;
pub const _POSIX_DELAYTIMER_MAX: u32 = 32;
pub const _POSIX_HOST_NAME_MAX: u32 = 255;
pub const _POSIX_LINK_MAX: u32 = 8;
pub const _POSIX_LOGIN_NAME_MAX: u32 = 9;
pub const _POSIX_MAX_CANON: u32 = 255;
pub const _POSIX_MAX_INPUT: u32 = 255;
pub const _POSIX_MQ_OPEN_MAX: u32 = 8;
pub const _POSIX_MQ_PRIO_MAX: u32 = 32;
pub const _POSIX_NAME_MAX: u32 = 14;
pub const _POSIX_NGROUPS_MAX: u32 = 8;
pub const _POSIX_OPEN_MAX: u32 = 20;
pub const _POSIX_PATH_MAX: u32 = 256;
pub const _POSIX_PIPE_BUF: u32 = 512;
pub const _POSIX_RE_DUP_MAX: u32 = 255;
pub const _POSIX_RTSIG_MAX: u32 = 8;
pub const _POSIX_SEM_NSEMS_MAX: u32 = 256;
pub const _POSIX_SEM_VALUE_MAX: u32 = 32767;
pub const _POSIX_SIGQUEUE_MAX: u32 = 32;
pub const _POSIX_SSIZE_MAX: u32 = 32767;
pub const _POSIX_STREAM_MAX: u32 = 8;
pub const _POSIX_SYMLINK_MAX: u32 = 255;
pub const _POSIX_SYMLOOP_MAX: u32 = 8;
pub const _POSIX_TIMER_MAX: u32 = 32;
pub const _POSIX_TTY_NAME_MAX: u32 = 9;
pub const _POSIX_TZNAME_MAX: u32 = 6;
pub const _POSIX_CLOCKRES_MIN: u32 = 20000000;
pub const NR_OPEN: u32 = 1024;
pub const NGROUPS_MAX: u32 = 65536;
pub const ARG_MAX: u32 = 131072;
pub const LINK_MAX: u32 = 127;
pub const MAX_CANON: u32 = 255;
pub const MAX_INPUT: u32 = 255;
pub const NAME_MAX: u32 = 255;
pub const PATH_MAX: u32 = 4096;
pub const PIPE_BUF: u32 = 4096;
pub const XATTR_NAME_MAX: u32 = 255;
pub const XATTR_SIZE_MAX: u32 = 65536;
pub const XATTR_LIST_MAX: u32 = 65536;
pub const RTSIG_MAX: u32 = 32;
pub const _POSIX_THREAD_KEYS_MAX: u32 = 128;
pub const PTHREAD_KEYS_MAX: u32 = 1024;
pub const _POSIX_THREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const PTHREAD_DESTRUCTOR_ITERATIONS: u32 = 4;
pub const _POSIX_THREAD_THREADS_MAX: u32 = 64;
pub const AIO_PRIO_DELTA_MAX: u32 = 20;
pub const PTHREAD_STACK_MIN: u32 = 16384;
pub const DELAYTIMER_MAX: u32 = 2147483647;
pub const TTY_NAME_MAX: u32 = 32;
pub const LOGIN_NAME_MAX: u32 = 256;
pub const HOST_NAME_MAX: u32 = 64;
pub const MQ_PRIO_MAX: u32 = 32768;
pub const SEM_VALUE_MAX: u32 = 2147483647;
pub const _BITS_POSIX2_LIM_H: u32 = 1;
pub const _POSIX2_BC_BASE_MAX: u32 = 99;
pub const _POSIX2_BC_DIM_MAX: u32 = 2048;
pub const _POSIX2_BC_SCALE_MAX: u32 = 99;
pub const _POSIX2_BC_STRING_MAX: u32 = 1000;
pub const _POSIX2_COLL_WEIGHTS_MAX: u32 = 2;
pub const _POSIX2_EXPR_NEST_MAX: u32 = 32;
pub const _POSIX2_LINE_MAX: u32 = 2048;
pub const _POSIX2_RE_DUP_MAX: u32 = 255;
pub const _POSIX2_CHARCLASS_NAME_MAX: u32 = 14;
pub const BC_BASE_MAX: u32 = 99;
pub const BC_DIM_MAX: u32 = 2048;
pub const BC_SCALE_MAX: u32 = 99;
pub const BC_STRING_MAX: u32 = 1000;
pub const COLL_WEIGHTS_MAX: u32 = 255;
pub const EXPR_NEST_MAX: u32 = 32;
pub const LINE_MAX: u32 = 2048;
pub const CHARCLASS_NAME_MAX: u32 = 2048;
pub const RE_DUP_MAX: u32 = 32767;
pub const I64FORMAT: &[u8; 4usize] = b"lld\0";
pub const I64uFORMAT: &[u8; 4usize] = b"llu\0";
pub const SECP256K1_WIDEMUL_INT128: u32 = 1;
pub const SECP256K1_INT128_NATIVE: u32 = 1;
pub const SECP256K1_CHECKMEM_ENABLED: u32 = 0;
pub const JACOBI64_ITERATIONS: u32 = 25;
pub const SECP256K1_N_C_2: u32 = 1;
pub const SECP256K1_B: u32 = 7;
pub const _STRING_H: u32 = 1;
pub const _BITS_TYPES_LOCALE_T_H: u32 = 1;
pub const _BITS_TYPES___LOCALE_T_H: u32 = 1;
pub const _STRINGS_H: u32 = 1;
pub const WINDOW_G: u32 = 15;
pub const WINDOW_A: u32 = 5;
pub const WNAF_BITS: u32 = 128;
pub const PIPPENGER_SCRATCH_OBJECTS: u32 = 6;
pub const STRAUSS_SCRATCH_OBJECTS: u32 = 5;
pub const PIPPENGER_MAX_BUCKET_WINDOW: u32 = 12;
pub const ECMULT_PIPPENGER_THRESHOLD: u32 = 88;
pub const ECMULT_MAX_POINTS_PER_BATCH: u32 = 5000000;
pub type wchar_t = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(16))]
#[derive(Debug, Copy, Clone)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: ::std::os::raw::c_longlong,
    pub __bindgen_padding_0: u64,
    pub __clang_max_align_nonce2: u128,
}
#[test]
fn bindgen_test_layout_max_align_t() {
    const UNINIT: ::std::mem::MaybeUninit<max_align_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<max_align_t>(),
        32usize,
        concat!("Size of: ", stringify!(max_align_t))
    );
    assert_eq!(
        ::std::mem::align_of::<max_align_t>(),
        16usize,
        concat!("Alignment of ", stringify!(max_align_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce1) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__clang_max_align_nonce2) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(max_align_t),
            "::",
            stringify!(__clang_max_align_nonce2)
        )
    );
}
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
    pub static mut s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_static:
        *const secp256k1_context;
}
extern "C" {
    pub static mut s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_no_precomp:
        *const secp256k1_context;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_selftest();
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_create(
        flags: ::std::os::raw::c_uint,
    ) -> *mut secp256k1_context;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_clone(
        ctx: *const secp256k1_context,
    ) -> *mut secp256k1_context;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_destroy(
        ctx: *mut secp256k1_context,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_set_illegal_callback(
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_set_error_callback(
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scratch_space_create(
        ctx: *const secp256k1_context,
        size: usize,
    ) -> *mut secp256k1_scratch_space;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scratch_space_destroy(
        ctx: *const secp256k1_context,
        scratch: *mut secp256k1_scratch_space,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_parse(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        input: *const ::std::os::raw::c_uchar,
        inputlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_serialize(
        ctx: *const secp256k1_context,
        output: *mut ::std::os::raw::c_uchar,
        outputlen: *mut usize,
        pubkey: *const secp256k1_pubkey,
        flags: ::std::os::raw::c_uint,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_cmp(
        ctx: *const secp256k1_context,
        pubkey1: *const secp256k1_pubkey,
        pubkey2: *const secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecdsa_signature_parse_compact(
        ctx: *const secp256k1_context,
        sig: *mut secp256k1_ecdsa_signature,
        input64: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecdsa_signature_parse_der(
        ctx: *const secp256k1_context,
        sig: *mut secp256k1_ecdsa_signature,
        input: *const ::std::os::raw::c_uchar,
        inputlen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecdsa_signature_serialize_der(
        ctx: *const secp256k1_context,
        output: *mut ::std::os::raw::c_uchar,
        outputlen: *mut usize,
        sig: *const secp256k1_ecdsa_signature,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecdsa_signature_serialize_compact(
        ctx: *const secp256k1_context,
        output64: *mut ::std::os::raw::c_uchar,
        sig: *const secp256k1_ecdsa_signature,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecdsa_verify(
        ctx: *const secp256k1_context,
        sig: *const secp256k1_ecdsa_signature,
        msghash32: *const ::std::os::raw::c_uchar,
        pubkey: *const secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecdsa_signature_normalize(
        ctx: *const secp256k1_context,
        sigout: *mut secp256k1_ecdsa_signature,
        sigin: *const secp256k1_ecdsa_signature,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_nonce_function_rfc6979:
        secp256k1_nonce_function;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_nonce_function_default:
        secp256k1_nonce_function;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecdsa_sign(
        ctx: *const secp256k1_context,
        sig: *mut secp256k1_ecdsa_signature,
        msghash32: *const ::std::os::raw::c_uchar,
        seckey: *const ::std::os::raw::c_uchar,
        noncefp: secp256k1_nonce_function,
        ndata: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_seckey_verify(
        ctx: *const secp256k1_context,
        seckey: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_create(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        seckey: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_seckey_negate(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_privkey_negate(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_negate(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_seckey_tweak_add(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_privkey_tweak_add(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_tweak_add(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_seckey_tweak_mul(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_privkey_tweak_mul(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_tweak_mul(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_context_randomize(
        ctx: *mut secp256k1_context,
        seed32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ec_pubkey_combine(
        ctx: *const secp256k1_context,
        out: *mut secp256k1_pubkey,
        ins: *const *const secp256k1_pubkey,
        n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_tagged_sha256(
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_xonly_pubkey_parse(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_xonly_pubkey,
        input32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_xonly_pubkey_serialize(
        ctx: *const secp256k1_context,
        output32: *mut ::std::os::raw::c_uchar,
        pubkey: *const secp256k1_xonly_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_xonly_pubkey_cmp(
        ctx: *const secp256k1_context,
        pk1: *const secp256k1_xonly_pubkey,
        pk2: *const secp256k1_xonly_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_xonly_pubkey_from_pubkey(
        ctx: *const secp256k1_context,
        xonly_pubkey: *mut secp256k1_xonly_pubkey,
        pk_parity: *mut ::std::os::raw::c_int,
        pubkey: *const secp256k1_pubkey,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_xonly_pubkey_tweak_add(
        ctx: *const secp256k1_context,
        output_pubkey: *mut secp256k1_pubkey,
        internal_pubkey: *const secp256k1_xonly_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_xonly_pubkey_tweak_add_check(
        ctx: *const secp256k1_context,
        tweaked_pubkey32: *const ::std::os::raw::c_uchar,
        tweaked_pk_parity: ::std::os::raw::c_int,
        internal_pubkey: *const secp256k1_xonly_pubkey,
        tweak32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_keypair_create(
        ctx: *const secp256k1_context,
        keypair: *mut secp256k1_keypair,
        seckey: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_keypair_sec(
        ctx: *const secp256k1_context,
        seckey: *mut ::std::os::raw::c_uchar,
        keypair: *const secp256k1_keypair,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_keypair_pub(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_pubkey,
        keypair: *const secp256k1_keypair,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_keypair_xonly_pub(
        ctx: *const secp256k1_context,
        pubkey: *mut secp256k1_xonly_pubkey,
        pk_parity: *mut ::std::os::raw::c_int,
        keypair: *const secp256k1_keypair,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_keypair_xonly_tweak_add(
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
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_nonce_function_bip340:
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_schnorrsig_sign32(
        ctx: *const secp256k1_context,
        sig64: *mut ::std::os::raw::c_uchar,
        msg32: *const ::std::os::raw::c_uchar,
        keypair: *const secp256k1_keypair,
        aux_rand32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_schnorrsig_sign(
        ctx: *const secp256k1_context,
        sig64: *mut ::std::os::raw::c_uchar,
        msg32: *const ::std::os::raw::c_uchar,
        keypair: *const secp256k1_keypair,
        aux_rand32: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_schnorrsig_sign_custom(
        ctx: *const secp256k1_context,
        sig64: *mut ::std::os::raw::c_uchar,
        msg: *const ::std::os::raw::c_uchar,
        msglen: usize,
        keypair: *const secp256k1_keypair,
        extraparams: *mut secp256k1_schnorrsig_extraparams,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_schnorrsig_verify(
        ctx: *const secp256k1_context,
        sig64: *const ::std::os::raw::c_uchar,
        msg: *const ::std::os::raw::c_uchar,
        msglen: usize,
        pubkey: *const secp256k1_xonly_pubkey,
    ) -> ::std::os::raw::c_int;
}
pub type _Float32 = f32;
pub type _Float64 = f64;
pub type _Float32x = f64;
pub type _Float64x = u128;
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
    pub fn __ctype_get_mb_cur_max() -> usize;
}
extern "C" {
    pub fn atof(__nptr: *const ::std::os::raw::c_char) -> f64;
}
extern "C" {
    pub fn atoi(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn atol(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn atoll(__nptr: *const ::std::os::raw::c_char) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtod(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f64;
}
extern "C" {
    pub fn strtof(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
    ) -> f32;
}
extern "C" {
    pub fn strtol(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn strtoul(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strtoq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtouq(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn strtoll(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn strtoull(
        __nptr: *const ::std::os::raw::c_char,
        __endptr: *mut *mut ::std::os::raw::c_char,
        __base: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_ulonglong;
}
extern "C" {
    pub fn l64a(__n: ::std::os::raw::c_long) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn a64l(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
pub type __u_char = ::std::os::raw::c_uchar;
pub type __u_short = ::std::os::raw::c_ushort;
pub type __u_int = ::std::os::raw::c_uint;
pub type __u_long = ::std::os::raw::c_ulong;
pub type __int8_t = ::std::os::raw::c_schar;
pub type __uint8_t = ::std::os::raw::c_uchar;
pub type __int16_t = ::std::os::raw::c_short;
pub type __uint16_t = ::std::os::raw::c_ushort;
pub type __int32_t = ::std::os::raw::c_int;
pub type __uint32_t = ::std::os::raw::c_uint;
pub type __int64_t = ::std::os::raw::c_long;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __int_least8_t = __int8_t;
pub type __uint_least8_t = __uint8_t;
pub type __int_least16_t = __int16_t;
pub type __uint_least16_t = __uint16_t;
pub type __int_least32_t = __int32_t;
pub type __uint_least32_t = __uint32_t;
pub type __int_least64_t = __int64_t;
pub type __uint_least64_t = __uint64_t;
pub type __quad_t = ::std::os::raw::c_long;
pub type __u_quad_t = ::std::os::raw::c_ulong;
pub type __intmax_t = ::std::os::raw::c_long;
pub type __uintmax_t = ::std::os::raw::c_ulong;
pub type __dev_t = ::std::os::raw::c_ulong;
pub type __uid_t = ::std::os::raw::c_uint;
pub type __gid_t = ::std::os::raw::c_uint;
pub type __ino_t = ::std::os::raw::c_ulong;
pub type __ino64_t = ::std::os::raw::c_ulong;
pub type __mode_t = ::std::os::raw::c_uint;
pub type __nlink_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type __pid_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __fsid_t {
    pub __val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___fsid_t() {
    const UNINIT: ::std::mem::MaybeUninit<__fsid_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__fsid_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__fsid_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type __clock_t = ::std::os::raw::c_long;
pub type __rlim_t = ::std::os::raw::c_ulong;
pub type __rlim64_t = ::std::os::raw::c_ulong;
pub type __id_t = ::std::os::raw::c_uint;
pub type __time_t = ::std::os::raw::c_long;
pub type __useconds_t = ::std::os::raw::c_uint;
pub type __suseconds_t = ::std::os::raw::c_long;
pub type __suseconds64_t = ::std::os::raw::c_long;
pub type __daddr_t = ::std::os::raw::c_int;
pub type __key_t = ::std::os::raw::c_int;
pub type __clockid_t = ::std::os::raw::c_int;
pub type __timer_t = *mut ::std::os::raw::c_void;
pub type __blksize_t = ::std::os::raw::c_long;
pub type __blkcnt_t = ::std::os::raw::c_long;
pub type __blkcnt64_t = ::std::os::raw::c_long;
pub type __fsblkcnt_t = ::std::os::raw::c_ulong;
pub type __fsblkcnt64_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt_t = ::std::os::raw::c_ulong;
pub type __fsfilcnt64_t = ::std::os::raw::c_ulong;
pub type __fsword_t = ::std::os::raw::c_long;
pub type __ssize_t = ::std::os::raw::c_long;
pub type __syscall_slong_t = ::std::os::raw::c_long;
pub type __syscall_ulong_t = ::std::os::raw::c_ulong;
pub type __loff_t = __off64_t;
pub type __caddr_t = *mut ::std::os::raw::c_char;
pub type __intptr_t = ::std::os::raw::c_long;
pub type __socklen_t = ::std::os::raw::c_uint;
pub type __sig_atomic_t = ::std::os::raw::c_int;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type u_long = __u_long;
pub type quad_t = __quad_t;
pub type u_quad_t = __u_quad_t;
pub type fsid_t = __fsid_t;
pub type loff_t = __loff_t;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type nlink_t = __nlink_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type pid_t = __pid_t;
pub type id_t = __id_t;
pub type daddr_t = __daddr_t;
pub type caddr_t = __caddr_t;
pub type key_t = __key_t;
pub type clock_t = __clock_t;
pub type clockid_t = __clockid_t;
pub type time_t = __time_t;
pub type timer_t = __timer_t;
pub type ulong = ::std::os::raw::c_ulong;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type u_int8_t = __uint8_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
pub type register_t = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __sigset_t {
    pub __val: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___sigset_t() {
    const UNINIT: ::std::mem::MaybeUninit<__sigset_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__sigset_t>(),
        128usize,
        concat!("Size of: ", stringify!(__sigset_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__sigset_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__sigset_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__val) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__sigset_t),
            "::",
            stringify!(__val)
        )
    );
}
pub type sigset_t = __sigset_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[test]
fn bindgen_test_layout_timespec() {
    const UNINIT: ::std::mem::MaybeUninit<timespec> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<timespec>(),
        16usize,
        concat!("Size of: ", stringify!(timespec))
    );
    assert_eq!(
        ::std::mem::align_of::<timespec>(),
        8usize,
        concat!("Alignment of ", stringify!(timespec))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_sec) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_sec)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).tv_nsec) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(timespec),
            "::",
            stringify!(tv_nsec)
        )
    );
}
pub type suseconds_t = __suseconds_t;
pub type __fd_mask = ::std::os::raw::c_long;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16usize],
}
#[test]
fn bindgen_test_layout_fd_set() {
    const UNINIT: ::std::mem::MaybeUninit<fd_set> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(fd_set))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__fds_bits) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(fd_set),
            "::",
            stringify!(__fds_bits)
        )
    );
}
pub type fd_mask = __fd_mask;
extern "C" {
    pub fn select(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pselect(
        __nfds: ::std::os::raw::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::std::os::raw::c_int;
}
pub type blksize_t = __blksize_t;
pub type blkcnt_t = __blkcnt_t;
pub type fsblkcnt_t = __fsblkcnt_t;
pub type fsfilcnt_t = __fsfilcnt_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub union __atomic_wide_counter {
    pub __value64: ::std::os::raw::c_ulonglong,
    pub __value32: __atomic_wide_counter__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __atomic_wide_counter__bindgen_ty_1 {
    pub __low: ::std::os::raw::c_uint,
    pub __high: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___atomic_wide_counter__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<__atomic_wide_counter__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__atomic_wide_counter__bindgen_ty_1>(),
        8usize,
        concat!("Size of: ", stringify!(__atomic_wide_counter__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__atomic_wide_counter__bindgen_ty_1>(),
        4usize,
        concat!(
            "Alignment of ",
            stringify!(__atomic_wide_counter__bindgen_ty_1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__low) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__atomic_wide_counter__bindgen_ty_1),
            "::",
            stringify!(__low)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__high) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__atomic_wide_counter__bindgen_ty_1),
            "::",
            stringify!(__high)
        )
    );
}
#[test]
fn bindgen_test_layout___atomic_wide_counter() {
    const UNINIT: ::std::mem::MaybeUninit<__atomic_wide_counter> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__atomic_wide_counter>(),
        8usize,
        concat!("Size of: ", stringify!(__atomic_wide_counter))
    );
    assert_eq!(
        ::std::mem::align_of::<__atomic_wide_counter>(),
        8usize,
        concat!("Alignment of ", stringify!(__atomic_wide_counter))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__value64) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__atomic_wide_counter),
            "::",
            stringify!(__value64)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__value32) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__atomic_wide_counter),
            "::",
            stringify!(__value32)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
#[test]
fn bindgen_test_layout___pthread_internal_list() {
    const UNINIT: ::std::mem::MaybeUninit<__pthread_internal_list> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_list>(),
        16usize,
        concat!("Size of: ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_list>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_list))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__prev) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__prev)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__next) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_list),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_slist {
    pub __next: *mut __pthread_internal_slist,
}
#[test]
fn bindgen_test_layout___pthread_internal_slist() {
    const UNINIT: ::std::mem::MaybeUninit<__pthread_internal_slist> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Size of: ", stringify!(__pthread_internal_slist))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_internal_slist>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_internal_slist))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__next) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_internal_slist),
            "::",
            stringify!(__next)
        )
    );
}
pub type __pthread_slist_t = __pthread_internal_slist;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[test]
fn bindgen_test_layout___pthread_mutex_s() {
    const UNINIT: ::std::mem::MaybeUninit<__pthread_mutex_s> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__pthread_mutex_s>(),
        40usize,
        concat!("Size of: ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_mutex_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_mutex_s))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__lock) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__lock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__count) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__owner) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__owner)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__nusers) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__nusers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__kind) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__kind)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__spins) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__spins)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__elision) as usize - ptr as usize },
        22usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__elision)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__list) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_mutex_s),
            "::",
            stringify!(__list)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_rwlock_arch_t {
    pub __readers: ::std::os::raw::c_uint,
    pub __writers: ::std::os::raw::c_uint,
    pub __wrphase_futex: ::std::os::raw::c_uint,
    pub __writers_futex: ::std::os::raw::c_uint,
    pub __pad3: ::std::os::raw::c_uint,
    pub __pad4: ::std::os::raw::c_uint,
    pub __cur_writer: ::std::os::raw::c_int,
    pub __shared: ::std::os::raw::c_int,
    pub __rwelision: ::std::os::raw::c_schar,
    pub __pad1: [::std::os::raw::c_uchar; 7usize],
    pub __pad2: ::std::os::raw::c_ulong,
    pub __flags: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout___pthread_rwlock_arch_t() {
    const UNINIT: ::std::mem::MaybeUninit<__pthread_rwlock_arch_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__pthread_rwlock_arch_t>(),
        56usize,
        concat!("Size of: ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_rwlock_arch_t>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_rwlock_arch_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__readers) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__readers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__writers) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wrphase_futex) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__wrphase_futex)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__writers_futex) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__writers_futex)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad3) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad4) as usize - ptr as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__cur_writer) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__cur_writer)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__shared) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__shared)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__rwelision) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__rwelision)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad1) as usize - ptr as usize },
        33usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad2) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__flags) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_rwlock_arch_t),
            "::",
            stringify!(__flags)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [::std::os::raw::c_uint; 2usize],
    pub __g_size: [::std::os::raw::c_uint; 2usize],
    pub __g1_orig_size: ::std::os::raw::c_uint,
    pub __wrefs: ::std::os::raw::c_uint,
    pub __g_signals: [::std::os::raw::c_uint; 2usize],
}
#[test]
fn bindgen_test_layout___pthread_cond_s() {
    const UNINIT: ::std::mem::MaybeUninit<__pthread_cond_s> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__pthread_cond_s>(),
        48usize,
        concat!("Size of: ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        ::std::mem::align_of::<__pthread_cond_s>(),
        8usize,
        concat!("Alignment of ", stringify!(__pthread_cond_s))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wseq) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__wseq)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__g1_start) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g1_start)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__g_refs) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_refs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__g_size) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__g1_orig_size) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g1_orig_size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wrefs) as usize - ptr as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__wrefs)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__g_signals) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(__pthread_cond_s),
            "::",
            stringify!(__g_signals)
        )
    );
}
pub type __tss_t = ::std::os::raw::c_uint;
pub type __thrd_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __once_flag {
    pub __data: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout___once_flag() {
    const UNINIT: ::std::mem::MaybeUninit<__once_flag> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__once_flag>(),
        4usize,
        concat!("Size of: ", stringify!(__once_flag))
    );
    assert_eq!(
        ::std::mem::align_of::<__once_flag>(),
        4usize,
        concat!("Alignment of ", stringify!(__once_flag))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__once_flag),
            "::",
            stringify!(__data)
        )
    );
}
pub type pthread_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutexattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_mutexattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_mutexattr_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutexattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_mutexattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutexattr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_condattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_condattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_condattr_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_condattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_condattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_condattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_condattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_key_t = ::std::os::raw::c_uint;
pub type pthread_once_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_attr_t {
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_attr_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_attr_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_attr_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_attr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_attr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_attr_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_mutex_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_mutex_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_mutex_t>(),
        40usize,
        concat!("Size of: ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_mutex_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_mutex_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_mutex_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::std::os::raw::c_char; 48usize],
    pub __align: ::std::os::raw::c_longlong,
}
#[test]
fn bindgen_test_layout_pthread_cond_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_cond_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_cond_t>(),
        48usize,
        concat!("Size of: ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_cond_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_cond_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_cond_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlock_t {
    pub __data: __pthread_rwlock_arch_t,
    pub __size: [::std::os::raw::c_char; 56usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_rwlock_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_rwlock_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlock_t>(),
        56usize,
        concat!("Size of: ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlock_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlock_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__data) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlock_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_rwlockattr_t {
    pub __size: [::std::os::raw::c_char; 8usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_rwlockattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_rwlockattr_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Size of: ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_rwlockattr_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_rwlockattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_rwlockattr_t),
            "::",
            stringify!(__align)
        )
    );
}
pub type pthread_spinlock_t = ::std::os::raw::c_int;
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrier_t {
    pub __size: [::std::os::raw::c_char; 32usize],
    pub __align: ::std::os::raw::c_long,
}
#[test]
fn bindgen_test_layout_pthread_barrier_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_barrier_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_barrier_t>(),
        32usize,
        concat!("Size of: ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrier_t>(),
        8usize,
        concat!("Alignment of ", stringify!(pthread_barrier_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrier_t),
            "::",
            stringify!(__align)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_barrierattr_t {
    pub __size: [::std::os::raw::c_char; 4usize],
    pub __align: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_pthread_barrierattr_t() {
    const UNINIT: ::std::mem::MaybeUninit<pthread_barrierattr_t> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Size of: ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        ::std::mem::align_of::<pthread_barrierattr_t>(),
        4usize,
        concat!("Alignment of ", stringify!(pthread_barrierattr_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__size) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__size)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__align) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(pthread_barrierattr_t),
            "::",
            stringify!(__align)
        )
    );
}
extern "C" {
    pub fn random() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srandom(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn initstate(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn setstate(__statebuf: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct random_data {
    pub fptr: *mut i32,
    pub rptr: *mut i32,
    pub state: *mut i32,
    pub rand_type: ::std::os::raw::c_int,
    pub rand_deg: ::std::os::raw::c_int,
    pub rand_sep: ::std::os::raw::c_int,
    pub end_ptr: *mut i32,
}
#[test]
fn bindgen_test_layout_random_data() {
    const UNINIT: ::std::mem::MaybeUninit<random_data> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<random_data>(),
        48usize,
        concat!("Size of: ", stringify!(random_data))
    );
    assert_eq!(
        ::std::mem::align_of::<random_data>(),
        8usize,
        concat!("Alignment of ", stringify!(random_data))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fptr) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(fptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rptr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).state) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(state)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rand_type) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_type)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rand_deg) as usize - ptr as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_deg)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).rand_sep) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(rand_sep)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).end_ptr) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(random_data),
            "::",
            stringify!(end_ptr)
        )
    );
}
extern "C" {
    pub fn random_r(__buf: *mut random_data, __result: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srandom_r(
        __seed: ::std::os::raw::c_uint,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn initstate_r(
        __seed: ::std::os::raw::c_uint,
        __statebuf: *mut ::std::os::raw::c_char,
        __statelen: usize,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setstate_r(
        __statebuf: *mut ::std::os::raw::c_char,
        __buf: *mut random_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rand() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand(__seed: ::std::os::raw::c_uint);
}
extern "C" {
    pub fn rand_r(__seed: *mut ::std::os::raw::c_uint) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn drand48() -> f64;
}
extern "C" {
    pub fn erand48(__xsubi: *mut ::std::os::raw::c_ushort) -> f64;
}
extern "C" {
    pub fn lrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn nrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn mrand48() -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn jrand48(__xsubi: *mut ::std::os::raw::c_ushort) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn srand48(__seedval: ::std::os::raw::c_long);
}
extern "C" {
    pub fn seed48(__seed16v: *mut ::std::os::raw::c_ushort) -> *mut ::std::os::raw::c_ushort;
}
extern "C" {
    pub fn lcong48(__param: *mut ::std::os::raw::c_ushort);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct drand48_data {
    pub __x: [::std::os::raw::c_ushort; 3usize],
    pub __old_x: [::std::os::raw::c_ushort; 3usize],
    pub __c: ::std::os::raw::c_ushort,
    pub __init: ::std::os::raw::c_ushort,
    pub __a: ::std::os::raw::c_ulonglong,
}
#[test]
fn bindgen_test_layout_drand48_data() {
    const UNINIT: ::std::mem::MaybeUninit<drand48_data> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<drand48_data>(),
        24usize,
        concat!("Size of: ", stringify!(drand48_data))
    );
    assert_eq!(
        ::std::mem::align_of::<drand48_data>(),
        8usize,
        concat!("Alignment of ", stringify!(drand48_data))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__x) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__old_x) as usize - ptr as usize },
        6usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__old_x)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__c) as usize - ptr as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__c)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__init) as usize - ptr as usize },
        14usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__init)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__a) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(drand48_data),
            "::",
            stringify!(__a)
        )
    );
}
extern "C" {
    pub fn drand48_r(__buffer: *mut drand48_data, __result: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn erand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut f64,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mrand48_r(
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn jrand48_r(
        __xsubi: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
        __result: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn srand48_r(
        __seedval: ::std::os::raw::c_long,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn seed48_r(
        __seed16v: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lcong48_r(
        __param: *mut ::std::os::raw::c_ushort,
        __buffer: *mut drand48_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn arc4random() -> __uint32_t;
}
extern "C" {
    pub fn arc4random_buf(__buf: *mut ::std::os::raw::c_void, __size: usize);
}
extern "C" {
    pub fn arc4random_uniform(__upper_bound: __uint32_t) -> __uint32_t;
}
extern "C" {
    pub fn malloc(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn calloc(
        __nmemb: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn realloc(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn free(__ptr: *mut ::std::os::raw::c_void);
}
extern "C" {
    pub fn reallocarray(
        __ptr: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn alloca(__size: ::std::os::raw::c_ulong) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn valloc(__size: usize) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn posix_memalign(
        __memptr: *mut *mut ::std::os::raw::c_void,
        __alignment: usize,
        __size: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn aligned_alloc(
        __alignment: ::std::os::raw::c_ulong,
        __size: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn abort() -> !;
}
extern "C" {
    pub fn atexit(__func: ::std::option::Option<unsafe extern "C" fn()>) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn at_quick_exit(
        __func: ::std::option::Option<unsafe extern "C" fn()>,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn on_exit(
        __func: ::std::option::Option<
            unsafe extern "C" fn(
                __status: ::std::os::raw::c_int,
                __arg: *mut ::std::os::raw::c_void,
            ),
        >,
        __arg: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn exit(__status: ::std::os::raw::c_int) -> !;
}
extern "C" {
    pub fn quick_exit(__status: ::std::os::raw::c_int) -> !;
}
extern "C" {
    pub fn _Exit(__status: ::std::os::raw::c_int) -> !;
}
extern "C" {
    pub fn getenv(__name: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn putenv(__string: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setenv(
        __name: *const ::std::os::raw::c_char,
        __value: *const ::std::os::raw::c_char,
        __replace: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn unsetenv(__name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearenv() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mktemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn mkstemp(__template: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkstemps(
        __template: *mut ::std::os::raw::c_char,
        __suffixlen: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mkdtemp(__template: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn system(__command: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn realpath(
        __name: *const ::std::os::raw::c_char,
        __resolved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub type __compar_fn_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: *const ::std::os::raw::c_void,
        arg2: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn bsearch(
        __key: *const ::std::os::raw::c_void,
        __base: *const ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn qsort(
        __base: *mut ::std::os::raw::c_void,
        __nmemb: usize,
        __size: usize,
        __compar: __compar_fn_t,
    );
}
extern "C" {
    pub fn abs(__x: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn labs(__x: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn llabs(__x: ::std::os::raw::c_longlong) -> ::std::os::raw::c_longlong;
}
extern "C" {
    pub fn div(__numer: ::std::os::raw::c_int, __denom: ::std::os::raw::c_int) -> div_t;
}
extern "C" {
    pub fn ldiv(__numer: ::std::os::raw::c_long, __denom: ::std::os::raw::c_long) -> ldiv_t;
}
extern "C" {
    pub fn lldiv(
        __numer: ::std::os::raw::c_longlong,
        __denom: ::std::os::raw::c_longlong,
    ) -> lldiv_t;
}
extern "C" {
    pub fn ecvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn gcvt(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ecvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fcvt_r(
        __value: f64,
        __ndigit: ::std::os::raw::c_int,
        __decpt: *mut ::std::os::raw::c_int,
        __sign: *mut ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __len: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mblen(__s: *const ::std::os::raw::c_char, __n: usize) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbtowc(
        __pwc: *mut wchar_t,
        __s: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn wctomb(__s: *mut ::std::os::raw::c_char, __wchar: wchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn mbstowcs(__pwcs: *mut wchar_t, __s: *const ::std::os::raw::c_char, __n: usize) -> usize;
}
extern "C" {
    pub fn wcstombs(__s: *mut ::std::os::raw::c_char, __pwcs: *const wchar_t, __n: usize) -> usize;
}
extern "C" {
    pub fn rpmatch(__response: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getsubopt(
        __optionp: *mut *mut ::std::os::raw::c_char,
        __tokens: *const *mut ::std::os::raw::c_char,
        __valuep: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getloadavg(__loadavg: *mut f64, __nelem: ::std::os::raw::c_int)
        -> ::std::os::raw::c_int;
}
pub type int_least8_t = __int_least8_t;
pub type int_least16_t = __int_least16_t;
pub type int_least32_t = __int_least32_t;
pub type int_least64_t = __int_least64_t;
pub type uint_least8_t = __uint_least8_t;
pub type uint_least16_t = __uint_least16_t;
pub type uint_least32_t = __uint_least32_t;
pub type uint_least64_t = __uint_least64_t;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_long;
pub type int_fast32_t = ::std::os::raw::c_long;
pub type int_fast64_t = ::std::os::raw::c_long;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_ulong;
pub type uint_fast32_t = ::std::os::raw::c_ulong;
pub type uint_fast64_t = ::std::os::raw::c_ulong;
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type __gnuc_va_list = __builtin_va_list;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct __mbstate_t {
    pub __count: ::std::os::raw::c_int,
    pub __value: __mbstate_t__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union __mbstate_t__bindgen_ty_1 {
    pub __wch: ::std::os::raw::c_uint,
    pub __wchb: [::std::os::raw::c_char; 4usize],
}
#[test]
fn bindgen_test_layout___mbstate_t__bindgen_ty_1() {
    const UNINIT: ::std::mem::MaybeUninit<__mbstate_t__bindgen_ty_1> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t__bindgen_ty_1))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wch) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wch)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__wchb) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t__bindgen_ty_1),
            "::",
            stringify!(__wchb)
        )
    );
}
#[test]
fn bindgen_test_layout___mbstate_t() {
    const UNINIT: ::std::mem::MaybeUninit<__mbstate_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__mbstate_t>(),
        8usize,
        concat!("Size of: ", stringify!(__mbstate_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__mbstate_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__mbstate_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__count) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__count)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__value) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__mbstate_t),
            "::",
            stringify!(__value)
        )
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos_t() {
    const UNINIT: ::std::mem::MaybeUninit<_G_fpos_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_G_fpos_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pos) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__state) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos_t),
            "::",
            stringify!(__state)
        )
    );
}
pub type __fpos_t = _G_fpos_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _G_fpos64_t {
    pub __pos: __off64_t,
    pub __state: __mbstate_t,
}
#[test]
fn bindgen_test_layout__G_fpos64_t() {
    const UNINIT: ::std::mem::MaybeUninit<_G_fpos64_t> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_G_fpos64_t>(),
        16usize,
        concat!("Size of: ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        ::std::mem::align_of::<_G_fpos64_t>(),
        8usize,
        concat!("Alignment of ", stringify!(_G_fpos64_t))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pos) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__pos)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__state) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_G_fpos64_t),
            "::",
            stringify!(__state)
        )
    );
}
pub type __fpos64_t = _G_fpos64_t;
pub type __FILE = _IO_FILE;
pub type FILE = _IO_FILE;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_codecvt {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_wide_data {
    _unused: [u8; 0],
}
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    const UNINIT: ::std::mem::MaybeUninit<_IO_FILE> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._flags) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_read_ptr) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_ptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_read_end) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_read_base) as usize - ptr as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_write_base) as usize - ptr as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_write_ptr) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_ptr)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_write_end) as usize - ptr as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_buf_base) as usize - ptr as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_buf_end) as usize - ptr as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_save_base) as usize - ptr as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_backup_base) as usize - ptr as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_backup_base)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._IO_save_end) as usize - ptr as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_end)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._markers) as usize - ptr as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_markers)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._chain) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_chain)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._fileno) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_fileno)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._flags2) as usize - ptr as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags2)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._old_offset) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_old_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._cur_column) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_cur_column)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._vtable_offset) as usize - ptr as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_vtable_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._shortbuf) as usize - ptr as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_shortbuf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._lock) as usize - ptr as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_lock)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._offset) as usize - ptr as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._codecvt) as usize - ptr as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_codecvt)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._wide_data) as usize - ptr as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_wide_data)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._freeres_list) as usize - ptr as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_freeres_list)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._freeres_buf) as usize - ptr as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_freeres_buf)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__pad5) as usize - ptr as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad5)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._mode) as usize - ptr as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_mode)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr)._unused2) as usize - ptr as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_unused2)
        )
    );
}
pub type va_list = __gnuc_va_list;
pub type fpos_t = __fpos_t;
extern "C" {
    pub static mut stdin: *mut FILE;
}
extern "C" {
    pub static mut stdout: *mut FILE;
}
extern "C" {
    pub static mut stderr: *mut FILE;
}
extern "C" {
    pub fn remove(__filename: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn rename(
        __old: *const ::std::os::raw::c_char,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn renameat(
        __oldfd: ::std::os::raw::c_int,
        __old: *const ::std::os::raw::c_char,
        __newfd: ::std::os::raw::c_int,
        __new: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn tmpfile() -> *mut FILE;
}
extern "C" {
    pub fn tmpnam(arg1: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tmpnam_r(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn tempnam(
        __dir: *const ::std::os::raw::c_char,
        __pfx: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn fflush(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fflush_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn freopen(
        __filename: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
}
extern "C" {
    pub fn fdopen(__fd: ::std::os::raw::c_int, __modes: *const ::std::os::raw::c_char)
        -> *mut FILE;
}
extern "C" {
    pub fn fmemopen(
        __s: *mut ::std::os::raw::c_void,
        __len: usize,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn open_memstream(
        __bufloc: *mut *mut ::std::os::raw::c_char,
        __sizeloc: *mut usize,
    ) -> *mut FILE;
}
extern "C" {
    pub fn setbuf(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char);
}
extern "C" {
    pub fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::std::os::raw::c_char,
        __modes: ::std::os::raw::c_int,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn setbuffer(__stream: *mut FILE, __buf: *mut ::std::os::raw::c_char, __size: usize);
}
extern "C" {
    pub fn setlinebuf(__stream: *mut FILE);
}
extern "C" {
    pub fn fprintf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn printf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfprintf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vprintf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsprintf(
        __s: *mut ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn snprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsnprintf(
        __s: *mut ::std::os::raw::c_char,
        __maxlen: ::std::os::raw::c_ulong,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vdprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn dprintf(
        __fd: ::std::os::raw::c_int,
        __fmt: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fscanf(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn scanf(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn sscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_fscanf"]
    pub fn fscanf1(
        __stream: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_scanf"]
    pub fn scanf1(__format: *const ::std::os::raw::c_char, ...) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_sscanf"]
    pub fn sscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vfscanf(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vscanf(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn vsscanf(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vfscanf"]
    pub fn vfscanf1(
        __s: *mut FILE,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vscanf"]
    pub fn vscanf1(
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[link_name = "\u{1}__isoc99_vsscanf"]
    pub fn vsscanf1(
        __s: *const ::std::os::raw::c_char,
        __format: *const ::std::os::raw::c_char,
        __arg: *mut __va_list_tag,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getchar_unlocked() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgetc_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fputc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE)
        -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putc_unlocked(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putchar_unlocked(__c: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn getw(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn putw(__w: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fgets(
        __s: *mut ::std::os::raw::c_char,
        __n: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getdelim(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __delimiter: ::std::os::raw::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn getline(
        __lineptr: *mut *mut ::std::os::raw::c_char,
        __n: *mut usize,
        __stream: *mut FILE,
    ) -> __ssize_t;
}
extern "C" {
    pub fn fputs(__s: *const ::std::os::raw::c_char, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn puts(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ungetc(__c: ::std::os::raw::c_int, __stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fread(
        __ptr: *mut ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __stream: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fwrite(
        __ptr: *const ::std::os::raw::c_void,
        __size: ::std::os::raw::c_ulong,
        __n: ::std::os::raw::c_ulong,
        __s: *mut FILE,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn fread_unlocked(
        __ptr: *mut ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fwrite_unlocked(
        __ptr: *const ::std::os::raw::c_void,
        __size: usize,
        __n: usize,
        __stream: *mut FILE,
    ) -> usize;
}
extern "C" {
    pub fn fseek(
        __stream: *mut FILE,
        __off: ::std::os::raw::c_long,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftell(__stream: *mut FILE) -> ::std::os::raw::c_long;
}
extern "C" {
    pub fn rewind(__stream: *mut FILE);
}
extern "C" {
    pub fn fseeko(
        __stream: *mut FILE,
        __off: __off_t,
        __whence: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ftello(__stream: *mut FILE) -> __off_t;
}
extern "C" {
    pub fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr(__stream: *mut FILE);
}
extern "C" {
    pub fn feof(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn clearerr_unlocked(__stream: *mut FILE);
}
extern "C" {
    pub fn feof_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ferror_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn perror(__s: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn fileno(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn fileno_unlocked(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn pclose(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn popen(
        __command: *const ::std::os::raw::c_char,
        __modes: *const ::std::os::raw::c_char,
    ) -> *mut FILE;
}
extern "C" {
    pub fn ctermid(__s: *mut ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn flockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn ftrylockfile(__stream: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn funlockfile(__stream: *mut FILE);
}
extern "C" {
    pub fn __uflow(arg1: *mut FILE) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn __overflow(arg1: *mut FILE, arg2: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_set_b32(
        r: *mut secp256k1_scalar,
        bin: *const ::std::os::raw::c_uchar,
        overflow: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_set_int(
        r: *mut secp256k1_scalar,
        v: ::std::os::raw::c_uint,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_get_b32(
        bin: *mut ::std::os::raw::c_uchar,
        a: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_add(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
        b: *const secp256k1_scalar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_mul(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
        b: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_inverse(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_negate(
        r: *mut secp256k1_scalar,
        a: *const secp256k1_scalar,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_eq(
        a: *const secp256k1_scalar,
        b: *const secp256k1_scalar,
    ) -> ::std::os::raw::c_int;
}
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
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_one: secp256k1_fe;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_const_beta: secp256k1_fe;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_normalize(r: *mut secp256k1_fe);
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_normalize_var(
        r: *mut secp256k1_fe,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_set_int(
        r: *mut secp256k1_fe,
        a: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_is_odd(
        a: *const secp256k1_fe,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_cmp_var(
        a: *const secp256k1_fe,
        b: *const secp256k1_fe,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_set_b32(
        r: *mut secp256k1_fe,
        a: *const ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_get_b32(
        r: *mut ::std::os::raw::c_uchar,
        a: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_negate(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
        m: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_add(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_mul(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
        b: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_sqr(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_inv(
        r: *mut secp256k1_fe,
        a: *const secp256k1_fe,
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ge_set_xo_var(
        r: *mut secp256k1_ge,
        x: *const secp256k1_fe,
        odd: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_gej_set_ge(
        r: *mut secp256k1_gej,
        a: *const secp256k1_ge,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_gej_neg(
        r: *mut secp256k1_gej,
        a: *const secp256k1_gej,
    );
}
extern "C" {
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_gej_add_var(
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
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_const_modinfo_fe:
        secp256k1_modinv64_modinfo;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_const_modinfo_scalar:
        secp256k1_modinv64_modinfo;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_one: secp256k1_scalar;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_scalar_zero: secp256k1_scalar;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_const_lambda: secp256k1_scalar;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ge_const_g: secp256k1_ge;
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_fe_const_b: secp256k1_fe;
}
extern "C" {
    pub fn memcpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memmove(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memccpy(
        __dest: *mut ::std::os::raw::c_void,
        __src: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn memset(
        __s: *mut ::std::os::raw::c_void,
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
    pub fn __memcmpeq(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn memchr(
        __s: *const ::std::os::raw::c_void,
        __c: ::std::os::raw::c_int,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn strcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strncat(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcoll(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_ulong;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_struct {
    pub __locales: [*mut __locale_data; 13usize],
    pub __ctype_b: *const ::std::os::raw::c_ushort,
    pub __ctype_tolower: *const ::std::os::raw::c_int,
    pub __ctype_toupper: *const ::std::os::raw::c_int,
    pub __names: [*const ::std::os::raw::c_char; 13usize],
}
#[test]
fn bindgen_test_layout___locale_struct() {
    const UNINIT: ::std::mem::MaybeUninit<__locale_struct> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__locale_struct>(),
        232usize,
        concat!("Size of: ", stringify!(__locale_struct))
    );
    assert_eq!(
        ::std::mem::align_of::<__locale_struct>(),
        8usize,
        concat!("Alignment of ", stringify!(__locale_struct))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__locales) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__locales)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__ctype_b) as usize - ptr as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_b)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__ctype_tolower) as usize - ptr as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_tolower)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__ctype_toupper) as usize - ptr as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__ctype_toupper)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).__names) as usize - ptr as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(__locale_struct),
            "::",
            stringify!(__names)
        )
    );
}
pub type __locale_t = *mut __locale_struct;
pub type locale_t = __locale_t;
extern "C" {
    pub fn strcoll_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __l: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strxfrm_l(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
        __l: locale_t,
    ) -> usize;
}
extern "C" {
    pub fn strdup(__s: *const ::std::os::raw::c_char) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strndup(
        __string: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strrchr(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strcspn(
        __s: *const ::std::os::raw::c_char,
        __reject: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strspn(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strpbrk(
        __s: *const ::std::os::raw::c_char,
        __accept: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strstr(
        __haystack: *const ::std::os::raw::c_char,
        __needle: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strtok_r(
        __s: *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
        __save_ptr: *mut *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strlen(__s: *const ::std::os::raw::c_char) -> ::std::os::raw::c_ulong;
}
extern "C" {
    pub fn strnlen(__string: *const ::std::os::raw::c_char, __maxlen: usize) -> usize;
}
extern "C" {
    pub fn strerror(__errnum: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    #[link_name = "\u{1}__xpg_strerror_r"]
    pub fn strerror_r(
        __errnum: ::std::os::raw::c_int,
        __buf: *mut ::std::os::raw::c_char,
        __buflen: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strerror_l(
        __errnum: ::std::os::raw::c_int,
        __l: locale_t,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn bcmp(
        __s1: *const ::std::os::raw::c_void,
        __s2: *const ::std::os::raw::c_void,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn bcopy(
        __src: *const ::std::os::raw::c_void,
        __dest: *mut ::std::os::raw::c_void,
        __n: usize,
    );
}
extern "C" {
    pub fn bzero(__s: *mut ::std::os::raw::c_void, __n: ::std::os::raw::c_ulong);
}
extern "C" {
    pub fn index(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn rindex(
        __s: *const ::std::os::raw::c_char,
        __c: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn ffs(__i: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsl(__l: ::std::os::raw::c_long) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ffsll(__ll: ::std::os::raw::c_longlong) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strcasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn strncasecmp_l(
        __s1: *const ::std::os::raw::c_char,
        __s2: *const ::std::os::raw::c_char,
        __n: usize,
        __loc: locale_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn explicit_bzero(__s: *mut ::std::os::raw::c_void, __n: usize);
}
extern "C" {
    pub fn strsep(
        __stringp: *mut *mut ::std::os::raw::c_char,
        __delim: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn strsignal(__sig: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpcpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn __stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: usize,
    ) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn stpncpy(
        __dest: *mut ::std::os::raw::c_char,
        __src: *const ::std::os::raw::c_char,
        __n: ::std::os::raw::c_ulong,
    ) -> *mut ::std::os::raw::c_char;
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecmult(
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
    pub fn s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecmult_multi_var(
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
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_pre_g:
        [secp256k1_ge_storage; 8192usize];
}
extern "C" {
    pub static s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_pre_g_128:
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
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct secp256k1_ecmult_gen_context {
    pub built: ::std::os::raw::c_int,
    pub blind: secp256k1_scalar,
    pub initial: secp256k1_gej,
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
        unsafe { ::std::ptr::addr_of!((*ptr).blind) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ecmult_gen_context),
            "::",
            stringify!(blind)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).initial) as usize - ptr as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(secp256k1_ecmult_gen_context),
            "::",
            stringify!(initial)
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
    pub static mut s2bca0a5cbf756dd4ff1f0bda4585a7d3c64e1480_secp256k1_ecmult_gen_prec_table:
        [[secp256k1_ge_storage; 16usize]; 64usize];
}
pub type __builtin_va_list = [__va_list_tag; 1usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __va_list_tag {
    pub gp_offset: ::std::os::raw::c_uint,
    pub fp_offset: ::std::os::raw::c_uint,
    pub overflow_arg_area: *mut ::std::os::raw::c_void,
    pub reg_save_area: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout___va_list_tag() {
    const UNINIT: ::std::mem::MaybeUninit<__va_list_tag> = ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<__va_list_tag>(),
        24usize,
        concat!("Size of: ", stringify!(__va_list_tag))
    );
    assert_eq!(
        ::std::mem::align_of::<__va_list_tag>(),
        8usize,
        concat!("Alignment of ", stringify!(__va_list_tag))
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).gp_offset) as usize - ptr as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(gp_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).fp_offset) as usize - ptr as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(fp_offset)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).overflow_arg_area) as usize - ptr as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(overflow_arg_area)
        )
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).reg_save_area) as usize - ptr as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(__va_list_tag),
            "::",
            stringify!(reg_save_area)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __locale_data {
    pub _address: u8,
}
