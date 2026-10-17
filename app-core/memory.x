/* Linker script for the nRF54L15 */
MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 64K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K /* not complete */

  RAM_VPR : ORIGIN = 0x20020000, LENGTH = 1K
}


/* Place shared memory buffer at fixed address */
SECTIONS
{
    .shared_mem : ALIGN(4) {
        __shared_mem = .;
    } > RAM_VPR
}
