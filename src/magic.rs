pub const LIMINE_BOOTLOADER_INFO_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xf55038d8e2a1202f,
    0x279426fcf5f59740,
];
pub const LIMINE_EXECUTABLE_CMDLINE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x4b161536e598651e,
    0xb390ad4a2f1f303a,
];
pub const LIMINE_FIRMWARE_TYPE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x8c2f75d90bef28a8,
    0x7045a4688eac00c3,
];
pub const LIMINE_STACK_SIZE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x224ef0460a8e8926,
    0xe1cb0fc25f46ea3d,
];
pub const LIMINE_HHDM_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x48dcf1cb8ad2b852,
    0x63984e959a98244b,
];
pub const LIMINE_FRAMEBUFFER_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x9d5827dcd881dd75,
    0xa3148604f6fab11b,
];
pub const LIMINE_TERMINAL_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xc8ac59310c2b0844,
    0xa68d0c7265d38878,
];
pub const LIMINE_PAGING_MODE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x95c1a0edab0944cb,
    0xa4e5cb3842f7488a,
];
pub const LIMINE_5_LEVEL_PAGING_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x94469551da9b3192,
    0xebe5e86db7382888,
];
pub const LIMINE_MP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x95a67b819a1b857e,
    0xa0b61b723b6a73e0,
];
pub const LIMINE_SMP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x95a67b819a1b857e,
    0xa0b61b723b6a73e0,
];
pub const LIMINE_MEMMAP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x67cf3d9d378a806f,
    0xe304acdfc50c3c62,
];
pub const LIMINE_ENTRY_POINT_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x13d86c035a1cd3e1,
    0x2b0caa89d8f3026a,
];
pub const LIMINE_EXECUTABLE_FILE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xad97e90e83f1ed67,
    0x31eb5d1c5ff23b69,
];
pub const LIMINE_KERNEL_FILE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xad97e90e83f1ed67,
    0x31eb5d1c5ff23b69,
];
pub const LIMINE_MODULE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x3e7e279702be32af,
    0xca1c4f3bd1280cee,
];
pub const LIMINE_RSDP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xc5e77b6b397e7b43,
    0x27637845accdcf3c,
];
pub const LIMINE_SMBIOS_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x9e9046f11e095391,
    0xaa4a520fefbde5ee,
];
pub const LIMINE_EFI_SYSTEM_TABLE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x5ceba5163eaaf6d6,
    0x0a6981610cf65fcc,
];
pub const LIMINE_EFI_MEMMAP_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x7df62a431d6872d5,
    0xa4fcdfb3e57306c8,
];
pub const LIMINE_DATE_AT_BOOT_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x502746e184c088aa,
    0xfbc5ec83e6327893,
];
pub const LIMINE_BOOT_TIME_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x502746e184c088aa,
    0xfbc5ec83e6327893,
];
pub const LIMINE_EXECUTABLE_ADDRESS_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x71ba76863cc55f63,
    0xb2644a48c516a487,
];
pub const LIMINE_KERNEL_ADDRESS_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x71ba76863cc55f63,
    0xb2644a48c516a487,
];
pub const LIMINE_DTB_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0xb40ddb48fb54bac7,
    0x545081493f81ffb7,
];
pub const LIMINE_RISCV_BSP_HARTID_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x1369359f025525f9,
    0x2ff2a56178391bb6,
];
pub const LIMINE_BOOTLOADER_PERFORMANCE_MAGIC: [u64; 4] = [
    0xc7b1dd30df4c8b88,
    0x0a82e883a194f07b,
    0x6b50ad9bf36d13ad,
    0xdc4c7e88fc759e17,
];
