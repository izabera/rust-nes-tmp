void render();

void foo_c(void) {}

void wait_vblank(void) {
    asm(
        "lda #$00\n"
        "sta $33\n"
        "nop\n"
"checkForNmi:\n"
        "lda $33\n"
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
            "  sta $33\n"
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
