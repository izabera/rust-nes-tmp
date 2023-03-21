void nmi_fn();

void foo_c(void) {}

__attribute__ ((no_isr)) void nmi(void) {

asm(
  /* ".section .nmi_begin,\"axG\",@progbits,nmi\n" */
  /* ".globl __default_nmi\n" */
  /* "nmi:\n" */
  /* "__default_nmi:\n" */
  "  pha\n"
  "  txa\n"
  "  pha\n"
  "  tya\n"
  "  pha\n"
  "  nop\n"
  "  nop\n"
  "  nop\n"
  "  nop\n"
  );

        nmi_fn();
asm(
  /* ".section .nmi_end,\"axG\",@progbits,nmi\n" */
  "  pla\n"
  "  tay\n"
  "  pla\n"
  "  tax\n"
  "  pla\n"
  "  rti\n"
);
}
