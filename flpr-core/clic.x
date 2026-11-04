/* Image layout required by the CLIC support in `src/clic/rt.rs`.

   On interrupt entry the VPR sets `ra` to INITPC + 0x1F0 (INITPC is where the application
   core starts the FLPR, i.e. `_start`), so that interrupt handlers can be plain functions whose
   `ret` lands on an `mret`. This puts that `mret` at offset 0x1F0, and uses the space before it
   for the startup code and the exception entry, which are both written in assembly and so have
   a fixed size.

   Must be passed to the linker after `memory.x` and before `link.x`. */

SECTIONS
{
  .vpr_head ORIGIN(REGION_TEXT) :
  {
    KEEP(*(.init));             /* riscv-rt `_start`: must be at INITPC */
    KEEP(*(.trap.start.clic));  /* exception entry (`mtvec`) */
    . = ADDR(.vpr_head) + 0x1F0; /* fails to link if the code above does not fit */
    KEEP(*(.trap.isr_return));  /* `mret` for interrupt handlers */
  } > REGION_TEXT
}

/* The riscv-rt `.text` section follows */
_stext = ADDR(.vpr_head) + SIZEOF(.vpr_head);

ASSERT(_start == ADDR(.vpr_head), "
ERROR(clic.x): `_start` must be at the start of the image");
ASSERT(_clic_isr_return == _start + 0x1F0, "
ERROR(clic.x): the interrupt return `mret` must be at offset 0x1F0 of the image");
