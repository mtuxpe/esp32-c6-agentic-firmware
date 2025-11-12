# CLAUDE.md - Guidelines for Claude Code Assistant

## Model Selection

**DEFAULT: Use Haiku Model**
- Unless explicitly told otherwise, use Claude Haiku (fastest, most cost-effective)
- Only use Sonnet or Opus if the task requires complex reasoning
- Current model: claude-haiku-4-5-20251001

**How to specify model:**
```
/model sonnet    # Switch to Sonnet
/model opus      # Switch to Opus
/model haiku     # Back to Haiku (default)
```

---

## File Operations

### ❌ Task() CANNOT Create Files
- `Task()` launches a subprocess agent for complex work
- **Agents cannot create files** - they can only read and report back
- **Don't use Task()** for file generation

### ✅ Use These Tools Instead
- `Write()` - Create new files or overwrite existing
- `Edit()` - Modify specific parts of existing files
- `Bash` - Create files via shell commands
- `Read()` - Read files before editing

### Rule of Thumb
**If you need to create/modify files → Use Write/Edit/Bash directly, NOT Task()**

---

## When to Use Task()

Task() is useful for:
- ✅ Complex research/exploration (general-purpose agent)
- ✅ Finding patterns in large codebases (Explore agent)
- ✅ Multi-step analysis and reporting
- ❌ **NOT for file creation/modification**

---

## Lean Lessons Approach

These lessons should be **simple and practical**:
- Focus on working code, not massive documentation
- Minimal READMEs (just basics)
- One main .rs file per lesson (~100-150 lines)
- Test on hardware immediately
- Keep it type-able for YouTube videos

---

## Project Conventions

### Directory Structure
```
lessons/{NN}-{name}/
├── src/
│   ├── bin/
│   │   └── main.rs          # Main firmware
│   └── lib.rs               # (optional library code)
├── .cargo/
│   └── config.toml          # Build config
├── Cargo.toml               # Dependencies
├── rust-toolchain.toml      # Toolchain
├── build.rs                 # Build script
└── README.md                # Simple docs (keep short!)
```

### Cargo.toml
- Always include `[[bin]]` section pointing to `src/bin/main.rs`
- Keep dependencies minimal
- Use esp-hal 1.0.0

### Documentation
- README.md: Keep it short (< 300 lines)
- Focus on: wiring, expected output, troubleshooting
- Skip lengthy theory - put that in code comments

---

## Testing Approach

1. **Build:** `cargo build --release`
2. **Flash:** `cargo run --release`
3. **Test:** Manual hardware validation
4. **Iterate:** Fix issues, re-test

No massive test plans until code works on hardware.

---

## Git Workflow

- Commit after each working lesson
- Keep commit messages clear and concise
- Format: `feat(lesson-{NN}): {brief description}`

---

## Bash Execution Best Practices

### Shell Limitations in LLM Context

The LLM's bash execution environment has important limitations:

**❌ Complex conditionals fail inline:**
```bash
# This will cause parse errors:
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
```

**✅ Use temp scripts for complex logic:**
```bash
# This works reliably:
cat > /tmp/script.sh << 'SCRIPT'
#!/bin/bash
if [ $EXIT_CODE -eq 0 ]; then
    echo "success"
fi
SCRIPT
chmod +x /tmp/script.sh
/tmp/script.sh
```

### Variable Persistence

**Variables DON'T persist across Bash() tool calls:**
```bash
# Step 1:
export MY_VAR="value"

# Step 2 (different Bash call):
echo $MY_VAR  # Empty! Variable is gone
```

**Use file-based state management:**
```bash
# Step 1: Save to file
echo "value" > /tmp/my_var.txt

# Step 2: Read from file
MY_VAR=$(cat /tmp/my_var.txt)
echo $MY_VAR  # Works!
```

**Don't rely on `export` or `source`** - they don't work across tool invocations.

### When to Use Temp Scripts

Use temp scripts (`/tmp/*.sh`) for:
- Commands with if/then/fi conditionals
- Loops (for, while)
- Complex variable manipulation
- Multi-step operations with error checking

Use inline bash for:
- Simple single commands
- Command chains with `&&` or `||`
- Quick reads/writes without conditionals

---

## Common Mistakes to Avoid

1. ❌ Using Task() to generate files
2. ❌ Over-engineering lessons (keep them simple!)
3. ❌ Massive documentation before working code
4. ❌ Not testing on hardware
5. ❌ Using expensive models (Sonnet/Opus) by default
6. ❌ Using complex conditionals inline in Bash (use temp scripts!)
7. ❌ Expecting variables to persist across Bash() calls (use files!)

---

## Quick Reference

| Task | Tool | Time |
|------|------|------|
| Create lesson code | Write() + Bash | 5-10 min |
| Modify file | Edit() | 2-5 min |
| Create README | Write() | 3-5 min |
| Test on hardware | Manual | 10-20 min |
| **Avoid: Massive planning** | ~~Task()~~ | ⏱️ Don't |

---

## Slash Commands & Tools

Custom slash commands are stored in `.claude/commands/`:

### Lesson Testing
- **`/test-lesson <number> [mode]`** - Unified hardware testing for any lesson
  - Examples: `/test-lesson 07`, `/test-lesson 08 full`
  - Modes: `quick` (default, 3-5 min) or `full` (10-20 min)
  - Auto-detects hardware (USB ports, JTAG probes)
  - Reads lesson-specific `TEST.md` for test procedures
  - Generates comprehensive test reports

Each lesson has a `TEST.md` specification that documents:
- Hardware setup and wiring
- Automated tests (build, flash, infrastructure)
- Interactive tests (manual verification)
- Expected outputs and troubleshooting

### RTT Debugging
- **`/rtt [subcommand]`** - RTT (Real-Time Transfer) debugging and validation tools
  - `tutorial [topic]` - Learn RTT best practices interactively
  - `sweep [options]` - Performance characterization for your device
  - `validate [file]` - Automated firmware testing on hardware
  - `analyze [log]` - Log analysis and parsing
  - `tools` - Reference and system diagnostics
  - `guide` - Open full RTT Mastery reference

See `.claude/rtt-guide.md` for complete RTT reference documentation.

---

**Last Updated:** 2025-11-12
**Current Work:** Lesson 08 (Structured Logging with defmt + RTT)
**Next:** Lesson 08-C3 (Flash size comparison)
