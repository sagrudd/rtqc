// We need to forward routine registration from C to Rust
// to avoid the linker removing the static library.

void R_init_rtqc_extendr(void *dll);

void R_init_rtqc(void *dll) {
    R_init_rtqc_extendr(dll);
}
