use crate::utils;
use nix::errno::Errno;
use nix::libc::user_regs_struct;
use nix::sys::ptrace;
use nix::unistd::Pid;
use std::error::Error;
use std::ffi::c_uint;

#[allow(dead_code)]
pub struct UserRegsStructX86 {
    bx: c_uint,
    cx: c_uint,
    dx: c_uint,
    si: c_uint,
    di: c_uint,
    bp: c_uint,
    ax: c_uint,
    ds: c_uint,
    es: c_uint,
    fs: c_uint,
    gs: c_uint,
    orig_ax: c_uint,
    ip: c_uint,
    cs: c_uint,
    flags: c_uint,
    sp: c_uint,
    ss: c_uint,
}

pub static USER_REG_NAMES_X64: &[&str] = &[
    "r15", "r14", "r13", "r12", "rbp", "rbx", "r11", "r10", "r9", "r8", "rax", "rcx", "rdx", "rsi",
    "rdi", "orig_rax", "rip", "cs", "eflags", "rsp", "ss", "fs_base", "gs_base", "ds", "es", "fs",
    "gs",
];

pub fn get_registers(pid: Pid) -> Result<user_regs_struct, Errno> {
    return ptrace::getregs(pid);
}

pub fn get_program_counter(child_pid: i32) -> Result<u64, Box<dyn std::error::Error>> {
    let user_regs = get_registers(Pid::from_raw(child_pid))?;
    let res = get_register_value(user_regs, "rip");
    match res {
        Some(pc) => {
            return Ok(pc);
        }
        None => {
            return Err(utils::throw_custom_error(
                "Failed to retrieve rip".to_string(),
            ))
        }
    }
}

pub fn set_program_counter(child_pid: i32, value: u64) -> Result<(), Box<dyn std::error::Error>> {
    set_register_value(Pid::from_raw(child_pid), "rip", value)?;
    Ok(())
}

pub fn get_register_value(regs: user_regs_struct, reg: &str) -> Option<u64> {
    match reg {
        "r15" => Some(regs.r15),
        "r14" => Some(regs.r14),
        "r13" => Some(regs.r13),
        "r12" => Some(regs.r12),
        "rbp" => Some(regs.rbp),
        "rbx" => Some(regs.rbx),
        "r11" => Some(regs.r11),
        "r10" => Some(regs.r10),
        "r9" => Some(regs.r9),
        "r8" => Some(regs.r8),
        "rax" => Some(regs.rax),
        "rcx" => Some(regs.rcx),
        "rdx" => Some(regs.rdx),
        "rsi" => Some(regs.rsi),
        "rdi" => Some(regs.rdi),
        "orig_rax" => Some(regs.orig_rax),
        "rip" => Some(regs.rip),
        "cs" => Some(regs.cs),
        "eflags" => Some(regs.eflags),
        "rsp" => Some(regs.rsp),
        "ss" => Some(regs.ss),
        "fs_base" => Some(regs.fs_base),
        "gs_base" => Some(regs.gs_base),
        "ds" => Some(regs.ds),
        "es" => Some(regs.es),
        "fs" => Some(regs.fs),
        "gs" => Some(regs.gs),
        _ => None,
    }
}

pub fn set_register_value(pid: Pid, reg_name: &str, val: u64) -> Result<(), Box<dyn Error>> {
    let mut registers = ptrace::getregs(pid)?;
    match reg_name {
        "r15" => registers.r15 = val,
        "r14" => registers.r14 = val,
        "r13" => registers.r13 = val,
        "r12" => registers.r12 = val,
        "rbp" => registers.rbp = val,
        "rbx" => registers.rbx = val,
        "r11" => registers.r11 = val,
        "r10" => registers.r10 = val,
        "r9" => registers.r9 = val,
        "r8" => registers.r8 = val,
        "rax" => registers.rax = val,
        "rcx" => registers.rcx = val,
        "rdx" => registers.rdx = val,
        "rsi" => registers.rsi = val,
        "rdi" => registers.rdi = val,
        "orig_rax" => registers.orig_rax = val,
        "rip" => registers.rip = val,
        "cs" => registers.cs = val,
        "eflags" => registers.eflags = val,
        "rsp" => registers.rsp = val,
        "ss" => registers.ss = val,
        "fs_base" => registers.fs_base = val,
        "gs_base" => registers.gs_base = val,
        "ds" => registers.ds = val,
        "es" => registers.es = val,
        "fs" => registers.fs = val,
        "gs" => registers.gs = val,
        _ => return Err(utils::throw_custom_error("Invalid register".to_string())),
    }
    ptrace::setregs(pid, registers)?;
    Ok(())
}
