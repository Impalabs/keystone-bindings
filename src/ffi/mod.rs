//! Unsafe Rust bindings for the Keystone Engine assembler library.
#![allow(non_camel_case_types)]

use bitflags::bitflags;
use libc::*;

use core::marker::{PhantomData, PhantomPinned};

// -----------------------------------------------------------------------------------------------
// Types
// -----------------------------------------------------------------------------------------------

/// Opaque type for the keystone engine.
#[repr(C)]
pub struct KsEngine {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

/// Pointer to a [`KsEngine`] object.
pub type KsHandle = std::ptr::NonNull<KsEngine>;

// -----------------------------------------------------------------------------------------------
// Constants
// -----------------------------------------------------------------------------------------------

// These values have been generated using the const_generator.py script of the official
// keystone repository:
//     - https://github.com/keystone-engine/keystone/blob/0.9.2/bindings/const_generator.py

/// Keystone major API version.
pub const API_MAJOR: c_uint = 0;
/// Keystone minor API version.
pub const API_MINOR: c_uint = 9;

/// Architecture type.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Arch {
    /// ARM architecture (including Thumb, Thumb-2).
    ARM = 1,
    /// ARM-64, also called AArch64.
    ARM64 = 2,
    /// Mips architecture.
    MIPS = 3,
    /// X86 architecture (including x86 & x86-64).
    X86 = 4,
    /// PowerPC architecture (currently unsupported).
    PPC = 5,
    /// Sparc architecture.
    SPARC = 6,
    /// SystemZ architecture (S390X).
    SYSTEMZ = 7,
    /// Hexagon architecture.
    HEXAGON = 8,
    /// Ethereum Virtual Machine architecture.
    EVM = 9,
    /// Maximum value for the architecture enum.
    MAX = 10,
}

bitflags! {
    ///  Mode type.
    #[repr(C)]
    pub struct Mode: c_int {
        /// Little-endian mode (default mode).
        const LITTLE_ENDIAN = 0;
        /// Big-endian mode.
        const BIG_ENDIAN = 1073741824;
        /// ARM/ARM64 - ARM mode.
        const ARM = 1;
        /// ARM/ARM64 - THUMB mode (including Thumb-2).
        const THUMB = 16;
        /// ARM/ARM64 - ARMv8 A32 encodings for ARM.
        const V8 = 64;
        /// MIPS - MicroMips mode.
        const MICRO = 16;
        /// MIPS - Mips III ISA.
        const MIPS3 = 32;
        /// MIPS - Mips32r6 ISA.
        const MIPS32R6 = 64;
        /// MIPS - Mips32 ISA.
        const MIPS32 = 4;
        /// MIPS - Mips64 ISA.
        const MIPS64 = 8;
        /// X86/X64 - 16-bit mode.
        const MODE_16 = 2;
        /// X86/X64 - 32-bit mode.
        const MODE_32 = 4;
        /// X86/X64 - 64-bit mode.
        const MODE_64 = 8;
        /// X86/X64 - 32-bit mode.
        const PPC32 = 4;
        /// PPC - 64-bit mode.
        const PPC64 = 8;
        /// PPC - Quad Processing eXtensions mode.
        const QPX = 16;
        /// PPC - 32-bit mode.
        const SPARC32 = 4;
        /// SPARC - 64-bit mode.
        const SPARC64 = 8;
        /// SPARC - SparcV9 mode.
        const V9 = 16;
    }
}

/// All type of errors encountered by Keystone API.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Error {
    /// No error: everything was fine.
    OK = 0,
    /// Out-Of-Memory error: ks_open(), ks_emulate().
    NOMEM = 1,
    /// Unsupported architecture: ks_open().
    ARCH = 2,
    /// Invalid handle.
    HANDLE = 3,
    /// Invalid/unsupported mode: ks_open().
    MODE = 4,
    /// Unsupported version (bindings).
    VERSION = 5,
    /// Unsupported option.
    OPT_INVALID = 6,
    /// Unknown token in expression.
    ASM_EXPR_TOKEN = 128,
    /// Literal value out of range for directive.
    ASM_DIRECTIVE_VALUE_RANGE = 129,
    /// Expected identifier in directive.
    ASM_DIRECTIVE_ID = 130,
    /// Unexpected token in directive.
    ASM_DIRECTIVE_TOKEN = 131,
    /// Expected string in directive.
    ASM_DIRECTIVE_STR = 132,
    /// Expected comma in directive.
    ASM_DIRECTIVE_COMMA = 133,
    /// Expected relocation name in directive.
    ASM_DIRECTIVE_RELOC_NAME = 134,
    /// Unexpected token in .reloc directive.
    ASM_DIRECTIVE_RELOC_TOKEN = 135,
    /// Invalid floating point in directive.
    ASM_DIRECTIVE_FPOINT = 136,
    /// Unknown directive.
    ASM_DIRECTIVE_UNKNOWN = 137,
    /// Invalid equal directive.
    ASM_DIRECTIVE_EQU = 138,
    /// (Generic) invalid directive.
    ASM_DIRECTIVE_INVALID = 139,
    /// Invalid variant.
    ASM_VARIANT_INVALID = 140,
    /// Brackets expression not supported on this target.
    ASM_EXPR_BRACKET = 141,
    /// Unexpected symbol modifier following '@'.
    ASM_SYMBOL_MODIFIER = 142,
    /// Invalid symbol redefinition.
    ASM_SYMBOL_REDEFINED = 143,
    /// Cannot find a symbol.
    ASM_SYMBOL_MISSING = 144,
    /// Expected ')' in parentheses expression.
    ASM_RPAREN = 145,
    /// Unexpected token at start of statement.
    ASM_STAT_TOKEN = 146,
    /// Unsupported token yet.
    ASM_UNSUPPORTED = 147,
    /// Unexpected token in macro instantiation.
    ASM_MACRO_TOKEN = 148,
    /// Unbalanced parentheses in macro argument.
    ASM_MACRO_PAREN = 149,
    /// Expected '=' after formal parameter identifier.
    ASM_MACRO_EQU = 150,
    /// Too many positional arguments.
    ASM_MACRO_ARGS = 151,
    /// Macros cannot be nested more than 20 levels deep.
    ASM_MACRO_LEVELS_EXCEED = 152,
    /// Invalid macro string.
    ASM_MACRO_STR = 153,
    /// Invalid macro (generic error).
    ASM_MACRO_INVALID = 154,
    /// Unexpected backslash at end of escaped string.
    ASM_ESC_BACKSLASH = 155,
    /// Invalid octal escape sequence  (out of range).
    ASM_ESC_OCTAL = 156,
    /// Invalid escape sequence (unrecognized character).
    ASM_ESC_SEQUENCE = 157,
    /// Broken escape string.
    ASM_ESC_STR = 158,
    /// Invalid token.
    ASM_TOKEN_INVALID = 159,
    /// This instruction is unsupported in this mode.
    ASM_INSN_UNSUPPORTED = 160,
    /// Invalid fixup.
    ASM_FIXUP_INVALID = 161,
    /// Invalid label.
    ASM_LABEL_INVALID = 162,
    /// Invalid fragment.
    ASM_FRAGMENT_INVALID = 163,
    /// Generic input assembly errors (invalid operand) - architecture specific.
    ASM_INVALIDOPERAND = 512,
    /// Generic input assembly errors (missing feature) - architecture specific.
    ASM_MISSINGFEATURE = 513,
    /// Generic input assembly errors (mnemonic fail) - architecture specific.
    ASM_MNEMONICFAIL = 514,
}

impl Error {
    /// Returns the latest error recorded error, if any.
    pub fn new(ks: KsHandle) -> Option<Self> {
        let err = unsafe { ks_errno(ks) };
        if err == Error::OK {
            None
        } else {
            Some(err)
        }
    }

    /// Returns a description for a given Keystone error.
    pub fn strerror(self) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(ks_strerror(self))
                .to_string_lossy()
                .into_owned()
        }
    }
}

impl std::error::Error for Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.strerror())
    }
}

/// Runtime option for the Keystone engine.
#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum OptionType {
    /// Choose syntax for input assembly.
    SYNTAX = 1,
    /// Set symbol resolver callback.
    SYM_RESOLVER = 2,
}

bitflags! {
    /// Runtime option value (associated with OptionType above)
    #[repr(C)]
    pub struct OptionValue: size_t {
        /// X86 Intel syntax - default on X86 (KS_OPT_SYNTAX).
        const SYNTAX_INTEL = 1;
        /// X86 ATT asm syntax (KS_OPT_SYNTAX).
        const SYNTAX_ATT = 2;
        /// X86 Nasm syntax (KS_OPT_SYNTAX).
        const SYNTAX_NASM = 4;
        /// X86 Masm syntax (KS_OPT_SYNTAX) - unsupported yet.
        const SYNTAX_MASM = 8;
        /// X86 GNU GAS syntax (KS_OPT_SYNTAX).
        const SYNTAX_GAS = 16;
        /// All immediates are in hex format (i.e 12 is 0x12).
        const SYNTAX_RADIX16 = 32;
    }
}

// -----------------------------------------------------------------------------------------------
// API
// -----------------------------------------------------------------------------------------------

extern "C" {
    /// Returns the combined API version, as well as the major and minor version numbers.
    ///
    /// **Inputs:**
    ///
    ///  * `major`: major number of API version;
    ///  * `minor`: minor number of API version.
    ///
    /// **Return value:**
    ///
    ///  * hexical number as `major << 8 | minor`, which encodes both major & minor versions.
    pub fn ks_version(major: *mut c_uint, minor: *mut c_uint) -> c_uint;

    /// Determines if the given architecture is supported by this library.
    ///
    /// **Input:**
    ///
    ///  * `arch`: architecture type ([`Arch`]).
    ///
    /// **Return value:**
    ///
    ///  * `True` if this library supports the given arch.
    pub fn ks_arch_supported(arch: Arch) -> c_int;

    /// Crates a new instance of the Keystone engine.
    ///
    /// **Inputs:**
    ///
    ///  * `arch`: architecture type ([`Arch`]);
    ///  * `mode`: hardware mode ([`Mode`]);
    ///  * `ks`: pointer to a Keystone engine object created by the function if it returns without
    ///    error.
    ///
    /// **Return value:**
    ///
    ///  * [`Error::OK`] on success, or another value on failure (refer to [`Error`] for more
    ///    details).
    pub fn ks_open(arch: Arch, mode: Mode, ks: *mut Option<KsHandle>) -> Error;

    /// Closes the Keystone instance.
    ///
    /// This operation must be performed once the handle is not used anymore to release it. This
    /// API releases cached memory, thus accessing the Keystone API through this handle once it
    /// has been closed with `ks_close` could crash the application.
    ///
    /// **Input:**
    ///
    ///  * `ks`: pointer to a handle returned by ks_open().
    ///
    /// **Return value:**
    ///
    ///  * [`Error::OK`] on success, or another value on failure (refer to [`Error`] for more
    ///    details).
    pub fn ks_close(ks: KsHandle);

    /// Reports the latest error number after an API call failed.
    ///
    /// Similarly to glibc's errno, `ks_errno` might not retain its old error once accessed.
    ///
    /// **Input:**
    ///
    ///  * `ks`: pointer to a handle returned by ks_open().
    ///
    /// **Return value:**
    ///
    ///  * The latest error code number (refer to [`Error`] for more details).
    pub fn ks_errno(ks: KsHandle) -> Error;

    /// Returns a string describing the given error code.
    ///
    /// **Input:**
    ///
    ///  * `code`: error code number (refer to [`Error`] for more details).
    ///
    /// **Return value:**
    ///
    ///  * A pointer to a string that describes the error code.
    pub fn ks_strerror(code: Error) -> *const c_char;

    /// Sets an option of the Keystone engine after the instance has been created.
    ///
    /// **Input:**
    ///
    ///  * `ks`: pointer to a handle returned by ks_open();
    ///  * `opt_type: [`OptionType`] to set;
    ///  * `value: corresponding [`OptionValue`] to set;
    ///  * `code`:  error code number (refer to [`Error`] for more details).
    ///
    /// **Return value:**
    ///
    ///  * A pointer to a string that describes the error code.
    pub fn ks_option(engine: KsHandle, opt_type: OptionType, value: OptionValue) -> Error;

    /// Assembles a program from an input string containing assembly instructions.
    ///
    /// The resulting machine code depends on the input buffer, its size, a base address and the
    /// number of instructions to encode. This API dynamically allocates memory to contain the
    /// assembled instructions. The caller is responsible for freeing memory allocated by the
    /// function and returned through `encoding`.
    ///
    /// **Input:**
    ///
    ///  * `ks`: pointer to a handle returned by ks_open();
    ///  * `string: NULL-terminated assembly string. Use ; or \n to separate statements;
    ///  * `address`: address of the first assembly instruction, or 0 to ignore;
    ///  * `encoding`: array of bytes containing the resulting encoding of the input assembly
    ///    string (this array is allocated by the function and should be freed manually using
    ///    [`ks_free`]);
    ///  * `encoding_size`: size of `*encoding`;
    ///  * `stat_count`: the number of statements successfully processed.
    ///
    /// **Return value:**
    ///
    ///  * 0 on success, or -1 on failure.
    pub fn ks_asm(
        ks: KsHandle,
        string: *const c_char,
        address: u64,
        encoding: *mut *mut c_uchar,
        encoding_size: *mut size_t,
        stat_count: *mut size_t,
    ) -> c_int;

    /// Frees memory allocated by ks_asm().
    ///
    /// **Input:**
    ///
    ///  * `p`: memory allocated by `ks_asm()` and returned in `encoding`.
    pub fn ks_free(p: *mut c_uchar);
}

// -----------------------------------------------------------------------------------------------
// Tests
// -----------------------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ks_version() {
        let mut major = 0;
        let mut minor = 0;
        let version = unsafe { ks_version(&mut major, &mut minor) };
        assert_eq!(major, API_MAJOR);
        assert_eq!(minor, API_MINOR);
        assert_eq!(version, API_MAJOR << 8 | API_MINOR);
    }

    #[test]
    fn test_ks_arch_supported() {
        assert!(unsafe { ks_arch_supported(Arch::ARM) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::ARM64) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::MIPS) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::X86) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::PPC) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::SPARC) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::SYSTEMZ) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::HEXAGON) != 0 });
        assert!(unsafe { ks_arch_supported(Arch::EVM) != 0 });
    }

    #[test]
    fn test_ks_open_ks_close() {
        // ARM - valid arch/mode combination
        let mut ks = None;
        let err = unsafe { ks_open(Arch::ARM, Mode::LITTLE_ENDIAN | Mode::ARM, &mut ks) };
        assert_eq!(err, Error::OK);
        assert!(ks.is_some());
        unsafe { ks_close(ks.unwrap()) };

        // ARM64 - invalid arch/mode combination
        let mut ks = None;
        let err = unsafe { ks_open(Arch::ARM64, Mode::LITTLE_ENDIAN | Mode::ARM, &mut ks) };
        assert_eq!(err, Error::MODE);
        assert!(ks.is_none());
    }

    #[test]
    fn test_ks_asm() {
        // Create a handle to the Keystone engine.
        let mut ks = None;
        let err = unsafe { ks_open(Arch::ARM, Mode::LITTLE_ENDIAN | Mode::ARM, &mut ks) };
        assert_eq!(err, Error::OK);
        assert!(ks.is_some());

        // Assemble instructions.
        let mut encoding: *mut c_uchar = std::ptr::null_mut();
        let mut encoding_size: size_t = 0;
        let mut stat_count: size_t = 0;
        let address = 0x1000;
        let insns = std::ffi::CString::new(
            "mov r0, #0x42
            label:
                str r0, [r1, #4]
                b label",
        )
        .unwrap();
        let err = unsafe {
            ks_asm(
                ks.unwrap(),
                insns.as_ptr(),
                address,
                &mut encoding,
                &mut encoding_size,
                &mut stat_count,
            )
        };
        assert_eq!(err, 0);
        assert!(!encoding.is_null());

        // Check the resulting machine code.
        let insns_slice = unsafe { std::slice::from_raw_parts(encoding, encoding_size) };
        let insns = insns_slice.to_vec();
        assert_eq!(
            insns,
            vec![66, 0, 160, 227, 4, 0, 129, 229, 253, 255, 255, 234]
        );
        unsafe { ks_free(encoding) };
    }

    #[test]
    fn test_ks_errno() {
        // Create a handle to the Keystone engine.
        let mut ks = None;
        let err = unsafe { ks_open(Arch::ARM, Mode::LITTLE_ENDIAN | Mode::ARM, &mut ks) };
        assert_eq!(err, Error::OK);
        assert!(ks.is_some());

        // Assemble instructions.
        let mut encoding: *mut c_uchar = std::ptr::null_mut();
        let mut encoding_size: size_t = 0;
        let mut stat_count: size_t = 0;
        let address = 0x1000;
        let insns = std::ffi::CString::new(
            "mov x0, #0x42", // Using X0 on ARM should result in an error.
        )
        .unwrap();
        let err = unsafe {
            ks_asm(
                ks.unwrap(),
                insns.as_ptr(),
                address,
                &mut encoding,
                &mut encoding_size,
                &mut stat_count,
            )
        };
        assert_eq!(err, -1);
        let errno = unsafe { ks_errno(ks.unwrap()) };
        assert_eq!(errno, Error::ASM_INVALIDOPERAND);
        let err_str = unsafe {
            std::ffi::CStr::from_ptr(ks_strerror(errno))
                .to_string_lossy()
                .into_owned()
        };
        assert_eq!(err_str, "Invalid operand (KS_ERR_ASM_INVALIDOPERAND)");
    }

    #[test]
    fn test_ks_option() {
        // Create a handle to the Keystone engine.
        let mut ks = None;
        let err = unsafe { ks_open(Arch::X86, Mode::MODE_32, &mut ks) };
        assert_eq!(err, Error::OK);
        assert!(ks.is_some());

        // Change an option after the instance has been created.
        let err = unsafe { ks_option(ks.unwrap(), OptionType::SYNTAX, OptionValue::SYNTAX_ATT) };
        assert_eq!(err, Error::OK);
    }
}
