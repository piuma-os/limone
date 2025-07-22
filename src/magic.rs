pub const LIMINE_BOOTLOADER_INFO_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xf55038d8e2a1202f,
    0x279426fcf5f59740,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineBootloaderInfoMagic([u64; 4]);

impl LimineBootloaderInfoMagic {
    pub const fn new() -> Self {
        Self(LIMINE_BOOTLOADER_INFO_MAGIC)
    }
}

impl Default for LimineBootloaderInfoMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_EXECUTABLE_CMDLINE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x4b161536e598651e,
    0xb390ad4a2f1f303a,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineExecutableCmdlineMagic([u64; 4]);

impl LimineExecutableCmdlineMagic {
    pub const fn new() -> Self {
        Self(LIMINE_EXECUTABLE_CMDLINE_MAGIC)
    }
}

impl Default for LimineExecutableCmdlineMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_FIRMWARE_TYPE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x8c2f75d90bef28a8,
    0x7045a4688eac00c3,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineFirmwareTypeMagic([u64; 4]);

impl LimineFirmwareTypeMagic {
    pub const fn new() -> Self {
        Self(LIMINE_FIRMWARE_TYPE_MAGIC)
    }
}

impl Default for LimineFirmwareTypeMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_STACK_SIZE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x224ef0460a8e8926,
    0xe1cb0fc25f46ea3d,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineStackSizeMagic([u64; 4]);

impl LimineStackSizeMagic {
    pub const fn new() -> Self {
        Self(LIMINE_STACK_SIZE_MAGIC)
    }
}

impl Default for LimineStackSizeMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_HHDM_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x48dcf1cb8ad2b852,
    0x63984e959a98244b,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineHhdmMagic([u64; 4]);

impl LimineHhdmMagic {
    pub const fn new() -> Self {
        Self(LIMINE_HHDM_MAGIC)
    }
}

impl Default for LimineHhdmMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_FRAMEBUFFER_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x9d5827dcd881dd75,
    0xa3148604f6fab11b,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineFramebufferMagic([u64; 4]);

impl LimineFramebufferMagic {
    pub const fn new() -> Self {
        Self(LIMINE_FRAMEBUFFER_MAGIC)
    }
}

impl Default for LimineFramebufferMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_TERMINAL_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xc8ac59310c2b0844,
    0xa68d0c7265d38878,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineTerminalMagic([u64; 4]);

impl LimineTerminalMagic {
    pub const fn new() -> Self {
        Self(LIMINE_TERMINAL_MAGIC)
    }
}

impl Default for LimineTerminalMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_PAGING_MODE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x95c1a0edab0944cb,
    0xa4e5cb3842f7488a,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LiminePagingModeMagic([u64; 4]);

impl LiminePagingModeMagic {
    pub const fn new() -> Self {
        Self(LIMINE_PAGING_MODE_MAGIC)
    }
}

impl Default for LiminePagingModeMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_5_LEVEL_PAGING_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x94469551da9b3192,
    0xebe5e86db7382888,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Limine5LevelPagingMagic([u64; 4]);

impl Limine5LevelPagingMagic {
    pub const fn new() -> Self {
        Self(LIMINE_5_LEVEL_PAGING_MAGIC)
    }
}

impl Default for Limine5LevelPagingMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_MP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x95a67b819a1b857e,
    0xa0b61b723b6a73e0,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineMpMagic([u64; 4]);

impl LimineMpMagic {
    pub const fn new() -> Self {
        Self(LIMINE_MP_MAGIC)
    }
}

impl Default for LimineMpMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_SMP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x95a67b819a1b857e,
    0xa0b61b723b6a73e0,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineSmpMagic([u64; 4]);

impl LimineSmpMagic {
    pub const fn new() -> Self {
        Self(LIMINE_SMP_MAGIC)
    }
}

impl Default for LimineSmpMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_MEMMAP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x67cf3d9d378a806f,
    0xe304acdfc50c3c62,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineMemmapMagic([u64; 4]);

impl LimineMemmapMagic {
    pub const fn new() -> Self {
        Self(LIMINE_MEMMAP_MAGIC)
    }
}

impl Default for LimineMemmapMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_ENTRY_POINT_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x13d86c035a1cd3e1,
    0x2b0caa89d8f3026a,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineEntryPointMagic([u64; 4]);

impl LimineEntryPointMagic {
    pub const fn new() -> Self {
        Self(LIMINE_ENTRY_POINT_MAGIC)
    }
}

impl Default for LimineEntryPointMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_EXECUTABLE_FILE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xad97e90e83f1ed67,
    0x31eb5d1c5ff23b69,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineExecutableFileMagic([u64; 4]);

impl LimineExecutableFileMagic {
    pub const fn new() -> Self {
        Self(LIMINE_EXECUTABLE_FILE_MAGIC)
    }
}

impl Default for LimineExecutableFileMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_KERNEL_FILE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xad97e90e83f1ed67,
    0x31eb5d1c5ff23b69,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineKernelFileMagic([u64; 4]);

impl LimineKernelFileMagic {
    pub const fn new() -> Self {
        Self(LIMINE_KERNEL_FILE_MAGIC)
    }
}

impl Default for LimineKernelFileMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_MODULE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x3e7e279702be32af,
    0xca1c4f3bd1280cee,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineModuleMagic([u64; 4]);

impl LimineModuleMagic {
    pub const fn new() -> Self {
        Self(LIMINE_MODULE_MAGIC)
    }
}

impl Default for LimineModuleMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_RSDP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xc5e77b6b397e7b43,
    0x27637845accdcf3c,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineRsdpMagic([u64; 4]);

impl LimineRsdpMagic {
    pub const fn new() -> Self {
        Self(LIMINE_RSDP_MAGIC)
    }
}

impl Default for LimineRsdpMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_SMBIOS_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x9e9046f11e095391,
    0xaa4a520fefbde5ee,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineSmbiosMagic([u64; 4]);

impl LimineSmbiosMagic {
    pub const fn new() -> Self {
        Self(LIMINE_SMBIOS_MAGIC)
    }
}

impl Default for LimineSmbiosMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_EFI_SYSTEM_TABLE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x5ceba5163eaaf6d6,
    0x0a6981610cf65fcc,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineEfiSystemTableMagic([u64; 4]);

impl LimineEfiSystemTableMagic {
    pub const fn new() -> Self {
        Self(LIMINE_EFI_SYSTEM_TABLE_MAGIC)
    }
}

impl Default for LimineEfiSystemTableMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_EFI_MEMMAP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x7df62a431d6872d5,
    0xa4fcdfb3e57306c8,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineEfiMemmapMagic([u64; 4]);

impl LimineEfiMemmapMagic {
    pub const fn new() -> Self {
        Self(LIMINE_EFI_MEMMAP_MAGIC)
    }
}

impl Default for LimineEfiMemmapMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_DATE_AT_BOOT_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x502746e184c088aa,
    0xfbc5ec83e6327893,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineDateAtBootMagic([u64; 4]);

impl LimineDateAtBootMagic {
    pub const fn new() -> Self {
        Self(LIMINE_DATE_AT_BOOT_MAGIC)
    }
}

impl Default for LimineDateAtBootMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_BOOT_TIME_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x502746e184c088aa,
    0xfbc5ec83e6327893,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineBootTimeMagic([u64; 4]);

impl LimineBootTimeMagic {
    pub const fn new() -> Self {
        Self(LIMINE_BOOT_TIME_MAGIC)
    }
}

impl Default for LimineBootTimeMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_EXECUTABLE_ADDRESS_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x71ba76863cc55f63,
    0xb2644a48c516a487,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineExecutableAddressMagic([u64; 4]);

impl LimineExecutableAddressMagic {
    pub const fn new() -> Self {
        Self(LIMINE_EXECUTABLE_ADDRESS_MAGIC)
    }
}

impl Default for LimineExecutableAddressMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_KERNEL_ADDRESS_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x71ba76863cc55f63,
    0xb2644a48c516a487,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineKernelAddressMagic([u64; 4]);

impl LimineKernelAddressMagic {
    pub const fn new() -> Self {
        Self(LIMINE_KERNEL_ADDRESS_MAGIC)
    }
}

impl Default for LimineKernelAddressMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_DTB_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xb40ddb48fb54bac7,
    0x545081493f81ffb7,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineDtbMagic([u64; 4]);

impl LimineDtbMagic {
    pub const fn new() -> Self {
        Self(LIMINE_DTB_MAGIC)
    }
}

impl Default for LimineDtbMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_RISCV_BSP_HARTID_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x1369359f025525f9,
    0x2ff2a56178391bb6,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineRiscvBspHartidMagic([u64; 4]);

impl LimineRiscvBspHartidMagic {
    pub const fn new() -> Self {
        Self(LIMINE_RISCV_BSP_HARTID_MAGIC)
    }
}

impl Default for LimineRiscvBspHartidMagic {
    fn default() -> Self {
        Self::new()
    }
}

pub const LIMINE_BOOTLOADER_PERFORMANCE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x6b50ad9bf36d13ad,
    0xdc4c7e88fc759e17,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct LimineBootloaderPerformanceMagic([u64; 4]);

impl LimineBootloaderPerformanceMagic {
    pub const fn new() -> Self {
        Self(LIMINE_BOOTLOADER_PERFORMANCE_MAGIC)
    }
}

impl Default for LimineBootloaderPerformanceMagic {
    fn default() -> Self {
        Self::new()
    }
}
