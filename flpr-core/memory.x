/* Linker script for the nRF54L15 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000 + 128K, LENGTH = 1524K - 128K
  RAM_01 : ORIGIN = 0x20020000 + 1K, LENGTH = 0x1f000 /* not complete */

  RAM_SHARED (RW) : ORIGIN = 0x20020000, LENGTH = 1K

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
        . += 1K;
        __shared_mem_end = .;
    } > RAM_SHARED
}

_max_hart_id = 0;      /* Only a single hart */

/* Place everything into RAM. App core will load this. */
REGION_ALIAS("REGION_TEXT", RAM_01);
REGION_ALIAS("REGION_RODATA", RAM_01);
REGION_ALIAS("REGION_DATA", RAM_01);
REGION_ALIAS("REGION_BSS", RAM_01);
REGION_ALIAS("REGION_STACK", RAM_01);

ASSERT((_start == 0x20020400), "Unexpected load address for VPR");
