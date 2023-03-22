// "The techniques llvm-mos uses for interrupt handling are somewhat unusual" - https://llvm-mos.org/wiki/C_interrupts

void render();

void wait_vblank(void) {
    asm(
        "lda #$00\n"
        "sta $F\n"
        "nop\n"
"checkForNmi:\n"
        "lda $F\n"
        "beq checkForNmi\n"
    );
}

__attribute__ ((no_isr)) void nmi(void) {
    asm(
            "  pha\n"
            "  txa\n"
            "  pha\n"
            "  tya\n"
            "  pha\n"
            "  lda #$1\n"
            "  sta $F\n"
       );
    render();
    asm(
            "  pla\n"
            "  tay\n"
            "  pla\n"
            "  tax\n"
            "  pla\n"
            "  rti\n"
       );
}
