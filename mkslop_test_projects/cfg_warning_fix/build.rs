use patch_build_rs_macros::mkbuildrs;

mkbuildrs! {
    check_cfg: "llvm_component", values = [
        "x86", "arm", "aarch64", "amdgpu", "avr", "m68k", "csky", "loongarch", "mips",
        "powerpc", "systemz", "msp430", "riscv", "sparc", "nvptx", "hexagon", "xtensa",
        "webassembly", "bpf", "ipo", "bitreader", "bitwriter", "linker", "asmparser", "lto",
        "coverage", "instrumentation"
    ];
    cfg: "llvm_component" = "x86";
    cfg: "llvm_component" = "arm";
    cfg: "llvm_component" = "aarch64";
    cfg: "llvm_component" = "amdgpu";
    cfg: "llvm_component" = "avr";
    cfg: "llvm_component" = "m68k";
    cfg: "llvm_component" = "csky";
    cfg: "llvm_component" = "loongarch";
    cfg: "llvm_component" = "mips";
    cfg: "llvm_component" = "powerpc";
    cfg: "llvm_component" = "systemz";
    cfg: "llvm_component" = "msp430";
    cfg: "llvm_component" = "riscv";
    cfg: "llvm_component" = "sparc";
    cfg: "llvm_component" = "nvptx";
    cfg: "llvm_component" = "hexagon";
    cfg: "llvm_component" = "xtensa";
    cfg: "llvm_component" = "webassembly";
    cfg: "llvm_component" = "bpf";
    cfg: "llvm_component" = "ipo";
    cfg: "llvm_component" = "bitreader";
    cfg: "llvm_component" = "bitwriter";
    cfg: "llvm_component" = "linker";
    cfg: "llvm_component" = "asmparser";
    cfg: "llvm_component" = "lto";
    cfg: "llvm_component" = "coverage";
    cfg: "llvm_component" = "instrumentation";
    resource_req: { ram = "4GB", cpu = "2", instance_type = "t3.medium" };
    secret_req: ["DATABASE_PASSWORD", "API_KEY"];
    systemd_service: "my-app", start_command = "/usr/local/bin/my-app";
    userdata_script: "setup.sh";
}
