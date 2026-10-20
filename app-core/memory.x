/* Linker script for the nRF54L15 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 64K

  /* RAM00: Only used by application core */
  RAM_00 (RW) : ORIGIN = 0x20000000, LENGTH = 128K

  /* RAM01: Used for shared memory, and by FLPR */
  RAM_VPR (RW) : ORIGIN = 0x20020000, LENGTH = 64K
}

REGION_ALIAS("RAM", RAM_00);

SECTIONS
{
    /* Place shared memory buffer at fixed address */
    .shared_mem (NOLOAD) : ALIGN(4) {
        __shared_mem = .;
        . += 1K;
        __shared_mem_end = .;
    } > RAM_VPR
}
