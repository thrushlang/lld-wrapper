#[repr(C)]
#[derive(Debug)]
pub enum LLDFlavor {
    Elf = 0,
    Wasm = 1,
    MachO = 2,
    Coff = 3,
}
