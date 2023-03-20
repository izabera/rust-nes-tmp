void nmi(); // rust function

typedef void (*function_type)(void);

/* function_type *const NMI = (function_type *)0xfffA; */

void nmi_fn(void) {
    asm(
            ".section .nmi_begin,\"axG\",@progbits,nmi\n"
            ".weak nmi\n"
            ".globl __default_nmi\n"
            "nmi:\n"
            "__default_nmi:\n"
            "  pha\n"
            "  txa\n"
            "  pha\n"
            "  tya\n"
            "  pha\n"
       );

        nmi();
    asm(
            ".section .nmi_end,\"axG\",@progbits,nmi\n"
            "  pla\n"
            "  tay\n"
            "  pla\n"
            "  tax\n"
            "  pla\n"
            "  rti\n"
       );
}

/* void something_c() { */
/*   *NMI = nmi_fn; */
/* } */
