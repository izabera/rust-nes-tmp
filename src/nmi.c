void nmi();

__attribute__ ((no_isr)) void nmi_fn(void) {
    asm(
            ".section .nmi.101,\"axG\",@progbits,nmi\n"
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
