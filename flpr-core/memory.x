/* Linker script for the nRF54L15 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000 + 128K, LENGTH = 1524K - 128K
  RAM : ORIGIN = 0x20020000 + 1K, LENGTH = 0x1f000 /* not complete */

  RAM_SHARED : ORIGIN = 0x20020000, LENGTH = 1K

  /* TODO: Figure out why this is necessary. If the heap is not specfied explicitly,
   * an error is raised that the heap is not aligned to 4 bytes.
   */
  REGION_HEAP : ORIGIN = 0x2003f000, LENGTH = 0xf000
}

SECTIONS
{
    /* shared memory, initialized by the application core */
    .shared_mem (NOLOAD) : {
        __shared_mem = .;
    } > RAM_SHARED
}

_max_hart_id = 0;      /* Only a single hart */

/* Everything will be loaded into RAM at runtime */
REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_STACK", RAM);
