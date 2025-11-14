---
description: Comprehensive repository review for esp32-c6-agentic-firmware - validate lessons, docs, structure, and prepare for community sharing
---

# /review-repo - Complete Repository Review & Cleanup

**Purpose**: Thoroughly review the entire esp32-c6-agentic-firmware repository to ensure it's ready for sharing with the embedded community. This command validates every lesson, documentation, project structure, and tooling.

**Target Audience**: Repository maintainer preparing to share with embedded Rust community

**Use when**: Before major releases, community sharing, or periodic quality audits

---

## Review Goals

1. **Educational Quality**: Ensure lessons progressively teach esp-hal 1.0.0 effectively
2. **Technical Accuracy**: Verify all code builds, flashes, and runs on hardware
3. **Documentation Clarity**: Check that docs are comprehensive but not verbose
4. **Community Readiness**: Ensure repo is approachable for external contributors
5. **LLM Optimization**: Validate Claude Code integration and debugging workflows

---

## Phase 1: Repository Structure & Documentation Review

### Step 1.1: Analyze Repository Structure

**Examine the overall project layout:**

```bash
# Review top-level structure
# Try tree first, fall back to find if not available
if command -v tree &>/dev/null; then
  tree -L 2 -I 'target|.git'
else
  # Fallback: use find to show structure
  echo "=== Repository Structure ==="
  find . -maxdepth 2 -type d | grep -v -E '(target|\.git|node_modules)' | sort
fi

# Check for consistency across lessons
find lessons -name "Cargo.toml" | wc -l
find lessons -name "README.md" | wc -l
find lessons -name "TEST.md" | wc -l
```

**Questions to answer:**
- Does the structure make intuitive sense for learners?
- Are lesson directories consistently organized?
- Are there any orphaned or unnecessary files?
- Is there clear separation between lessons, tools, and infrastructure?

**Actions:**
- [ ] Review directory tree and identify inconsistencies
- [ ] List any files that should be deleted or consolidated
- [ ] Check if lesson numbering makes logical sense (01, 02, 03...)
- [ ] Verify `.gitignore` is comprehensive

---

### Step 1.2: Review CLAUDE.md

**Read and analyze** `/CLAUDE.md`:

1. **Check accuracy**: Are all guidelines still current with esp-hal 1.0.0?
2. **Validate conventions**: Do they match actual codebase practices?
3. **Find gaps**: What's missing that would help Claude Code?
4. **Simplify**: Can any sections be clearer or more concise?

**Specific checks:**
- [ ] Memory map constants (RAM_START/RAM_END) documented correctly
- [ ] UART debugging workflow matches Lesson 08
- [ ] Bash execution best practices are accurate
- [ ] Hardware testing infrastructure documented
- [ ] esp-hal 1.0.0 API patterns up to date
- [ ] File operation guidelines clear (Task vs Write/Edit)

**Output**: List of CLAUDE.md improvements needed

---

### Step 1.3: Review README.md

**Read** `/README.md` (if exists, or note if missing):

**First, verify lesson list accuracy:**

```bash
# Get actual lesson names
ls -1 lessons/

# Compare to what README claims
# Read README and check lesson section matches reality
```

**If discrepancies found, note them immediately before proceeding.**

**Questions:**
- Does it explain the repo purpose clearly?
- Does it target the right audience (engineers learning esp-hal)?
- Are prerequisites listed (Rust, esp-hal, hardware)?
- Is setup/installation documented?
- Does it explain the LLM-enhanced debugging approach?
- Are there clear "Getting Started" instructions?

**Actions:**
- [ ] Validate lesson list matches actual lesson directories
- [ ] Check if README exists and is comprehensive
- [ ] Verify hardware requirements listed
- [ ] Check if software dependencies documented
- [ ] Ensure community contribution guidelines present

---

### Step 1.4: Review .claude/ Infrastructure

**Examine each file in `.claude/`:**

```bash
# List all Claude Code infrastructure
find .claude -type f | sort
ls -1 .claude/commands/
```

**Current infrastructure:**
```
.claude/
├── commands/
│   ├── esp32-debug.md
│   ├── gen-lesson.md
│   ├── improve-command.md
│   ├── review-repo.md (this file)
│   ├── setup-hardware-lesson.md
│   ├── test-debug-infrastructure.md
│   ├── test-lesson.md
│   └── test-uart-pins.md
├── templates/
│   ├── check-cargo-locks.sh
│   ├── read_uart.py
│   ├── test-all-lessons.sh
│   ├── uart_test_minimal.rs
│   └── validate-lesson.sh
├── TEST.md.template
├── TESTING-GUIDE.md
└── test-lesson-improvements.md
```

**For EACH command in `.claude/commands/`, evaluate:**

1. **Purpose & Relevance**
   - [ ] Still needed for current workflow?
   - [ ] Duplicates functionality of another command?
   - [ ] Could be combined with similar commands?
   - [ ] Should be split into multiple focused commands?

2. **Accuracy & Currency**
   - [ ] Instructions match current esp-hal 1.0.0?
   - [ ] References to CLAUDE.md conventions accurate?
   - [ ] Hardware setup matches actual lessons?
   - [ ] Example commands tested and working?

3. **Command-Specific Reviews:**

   **`/esp32-debug`** - General debugging helper
   - [ ] Still relevant or superseded by /test-debug-infrastructure?
   - [ ] Combine with /test-lesson?

   **`/gen-lesson`** - Lesson generation workflow
   - [ ] Does it create lessons matching current structure?
   - [ ] Should it call /setup-hardware-lesson internally?

   **`/improve-command`** - Meta-tool for command improvement
   - [ ] Has this been used? Was it helpful?
   - [ ] Document successful patterns from this review

   **`/setup-hardware-lesson`** - New lesson setup
   - [ ] Template up to date with esp-hal 1.0.0?
   - [ ] Should this be merged into /gen-lesson?

   **`/test-debug-infrastructure`** - Debug validation
   - [ ] Covers Lesson 07 + 08 workflows?
   - [ ] Overlaps with /test-lesson?

   **`/test-lesson`** - Unified hardware testing
   - [ ] Most comprehensive? Should this be the primary command?
   - [ ] Does it replace /test-uart-pins?

   **`/test-uart-pins`** - GPIO pin verification
   - [ ] Utility still needed or merged into /test-lesson?
   - [ ] Specific enough to keep separate?

**For `.claude/templates/`:**
- [ ] `uart_test_minimal.rs` - Uses esp-hal 1.0.0 correctly?
- [ ] `read_uart.py` - Works reliably across platforms?
- [ ] `check-cargo-locks.sh` - New script, tested?
- [ ] `test-all-lessons.sh` - New script, tested?
- [ ] `validate-lesson.sh` - New script, tested?

**For other `.claude/` files:**
- [ ] `TESTING-GUIDE.md` - Current and referenced by commands?
- [ ] `TEST.md.template` - Used by lessons? Up to date?
- [ ] `test-lesson-improvements.md` - Still relevant or archive?

**Consolidation Opportunities:**

**Candidate for merge:**
- `/esp32-debug` + `/test-debug-infrastructure` → Single debug command?
- `/setup-hardware-lesson` + `/gen-lesson` → Unified lesson creator?
- `/test-uart-pins` + `/test-lesson` → Absorb into main test command?

**Candidate for split:**
- `/gen-lesson` → Separate PRD generation from code scaffolding?
- `/test-lesson` → Split hardware test from simulation/build-only?

**Actions:**
- [ ] List commands to delete, merge, or split
- [ ] Note templates that need testing
- [ ] Identify gaps in command coverage
- [ ] Recommend command consolidation plan

---

## ⚠️ CRITICAL: Testing Best Practices

**READ THIS FIRST - Violations will cause review to fail.**

### Core Principles

1. **NEVER use `cd`** - Always use absolute paths or `--manifest-path`
2. **Verify environment first** - Check state before running tests
3. **Test one lesson first** - Don't batch test until single lesson works
4. **Use reliable scripts** - Prefer provided templates over custom scripts
5. **Capture full logs** - Don't truncate output, save to /tmp/lesson-test-logs/
6. **Build errors are expected** - Rust nightly toolchain may have breaking changes; document but don't block review

### Why These Rules Exist

- `cd` changes persist across bash calls → working directory confusion
- Batch testing hides individual failures → false diagnoses
- Grep-based success detection is fragile → use exit codes
- Custom scripts may have bugs → use tested templates
- esp-hal/esp-rom-sys may lag behind latest Rust nightly → not a lesson issue

---

## Reliable Testing Workflow

### Step 0: Verify Test Environment

**Always verify environment before any testing:**

```bash
bash .claude/templates/verify-test-environment.sh
```

**Expected output:**
- ✅ Working directory correct
- ✅ Repository structure intact
- ✅ Rust toolchain available
- ✅ No conflicting processes

**If verification fails, STOP and fix issues before proceeding.**

---

### Step 1: Test Single Lesson (Sanity Check)

**Test the simplest lesson first:**

```bash
bash .claude/templates/test-single-lesson-reliable.sh lessons/01-button-neopixel
```

**Expected:**
- Build succeeds with exit code 0
- Binary found and sized
- Full log saved to /tmp/lesson-test-logs/

**If this fails:**
1. Check the full log: `cat /tmp/lesson-test-logs/01-button-neopixel.log`
2. Look for actual error (not just symptoms)
3. Fix the one lesson before proceeding
4. **DO NOT** assume systemic toolchain issues without evidence

---

### Step 2: Test Representative Sample

**Test 3 lessons of varying complexity:**

```bash
bash .claude/templates/test-single-lesson-reliable.sh lessons/01-button-neopixel
bash .claude/templates/test-single-lesson-reliable.sh lessons/03-mpu9250
bash .claude/templates/test-single-lesson-reliable.sh lessons/08-uart-gdb-tandem
```

**Expected:** All three succeed

**If any fail:** Investigate individually, don't proceed to bulk testing

---

### Step 3: Bulk Test All Lessons

**Only proceed if Steps 0-2 passed:**

```bash
bash .claude/templates/test-all-lessons-reliable.sh
```

**This script will:**
- Run pre-flight checks
- Test each lesson individually
- Capture full logs for each
- Generate summary report

**Output:** Summary file at /tmp/lesson-test-logs/test-summary.txt

---

### Testing Anti-Patterns (DO NOT USE)

❌ **Don't loop in bash:**
```bash
# BAD: Hidden failures, fragile
for lesson in lessons/*/; do
  cargo build --manifest-path "$lesson/Cargo.toml"
done
```

❌ **Don't use cd:**
```bash
# BAD: Changes working directory persistently
cd lessons/01-button-neopixel
cargo build
```

❌ **Don't grep for success:**
```bash
# BAD: Fragile pattern matching
cargo build 2>&1 | grep -q "Finished"
```

❌ **Don't truncate error output:**
```bash
# BAD: Might miss critical context
cargo build 2>&1 | head -5
```

✅ **Use provided scripts:**
```bash
# GOOD: Tested, reliable, comprehensive
bash .claude/templates/test-single-lesson-reliable.sh lessons/XX-name
bash .claude/templates/test-all-lessons-reliable.sh
```

---

## Phase 2: Lesson-by-Lesson Review

### Step 2.0: Check Dependency Freshness

**CRITICAL: Before building lessons, check for stale Cargo.lock files.**

**Why this matters:**
- esp-hal ecosystem evolves rapidly
- Stale `Cargo.lock` can cause build failures with newer Rust nightly
- Fresh checkout won't have `Cargo.lock` (gitignored), but repo maintainer might
- Dependency drift can create false build failures

**Check Cargo.lock timestamps:**

**Option 1: Use provided template script**

```bash
# Use the pre-made checker
.claude/templates/check-cargo-locks.sh
```

**Option 2: Manual check**

```bash
# Check when lessons were last updated
ls -lh lessons/*/Cargo.lock | awk '{print $6, $7, $9}'

# Identify stale locks (>7 days old)
find lessons -name "Cargo.lock" -mtime +7 -exec ls -lh {} \;
```

**If stale Cargo.lock files found (or if any lesson fails to build):**

```bash
# Update dependencies for specific lesson (no cd needed)
cargo update --manifest-path lessons/01-button-neopixel/Cargo.toml

# OR update all lessons (use temp script with --manifest-path)
cat > /tmp/update-deps.sh << 'EOF'
#!/bin/bash
REPO_ROOT="/Users/shanemattner/Desktop/esp32-c6-agentic-firmware"

for lesson_dir in "$REPO_ROOT"/lessons/*/; do
  name=$(basename "$lesson_dir")
  echo "=== Updating $name ==="
  cargo update --manifest-path "$lesson_dir/Cargo.toml"
done
EOF
chmod +x /tmp/update-deps.sh
/tmp/update-deps.sh
```

**Proceed to build validation only after ensuring dependencies are fresh.**

---

### Step 2.1: Validate Each Lesson

**IMPORTANT: Follow the "Reliable Testing Workflow" outlined above (Steps 0-3).**

**Summary of workflow:**

1. **Verify environment** - `bash .claude/templates/verify-test-environment.sh`
2. **Test one lesson** - `bash .claude/templates/test-single-lesson-reliable.sh lessons/01-button-neopixel`
3. **Test sample (3 lessons)** - Simple, medium, complex
4. **Bulk test all** - `bash .claude/templates/test-all-lessons-reliable.sh`

**Do NOT skip steps. Do NOT create custom scripts unless templates fail.**

---

#### Build & Flash Validation (DEPRECATED - Use workflow above)

<details>
<summary>Legacy instructions (click to expand - NOT RECOMMENDED)</summary>

**Option 1: Use provided template script**

```bash
# Use the pre-made validation script
.claude/templates/validate-lesson.sh lessons/01-button-neopixel/
.claude/templates/validate-lesson.sh lessons/02-task-scheduler/
# ... continue for each lesson
```

**Option 2: Create custom validation script (AVOID THIS)**

```bash
# Create build validation script
cat > /tmp/validate-lesson.sh << 'EOF'
#!/bin/bash
LESSON_DIR="$1"

if [ -z "$LESSON_DIR" ]; then
  echo "Usage: $0 lessons/XX-name/"
  exit 1
fi

LESSON_NAME=$(basename "$LESSON_DIR")
echo "=== Validating $LESSON_NAME ==="

# Build the lesson
echo "Building..."
cargo build --release --manifest-path "$LESSON_DIR/Cargo.toml" 2>&1 | tail -10

if [ ${PIPESTATUS[0]} -eq 0 ]; then
  echo "✅ Build succeeded"

  # Find the actual binary (package name may vary)
  BINARY=$(find "$LESSON_DIR/target/riscv32imac-unknown-none-elf/release/" -type f -perm +111 2>/dev/null | grep -E '(lesson-|main$)' | head -1)

  if [ -n "$BINARY" ]; then
    echo "Binary: $(ls -lh "$BINARY" | awk '{print $9, $5}')"
    command -v size &>/dev/null && size "$BINARY" || true
  else
    echo "⚠️ Binary not found in target directory"
  fi
else
  echo "❌ Build failed"
  echo "Trying with cargo update first..."
  (cd "$LESSON_DIR" && cargo update)
  cargo build --release --manifest-path "$LESSON_DIR/Cargo.toml" 2>&1 | grep -E '(error|warning)' | head -20
fi
EOF

chmod +x /tmp/validate-lesson.sh

# Test each lesson
/tmp/validate-lesson.sh lessons/01-button-neopixel/
/tmp/validate-lesson.sh lessons/02-task-scheduler/
# ... continue for each lesson
```

**For individual lesson testing:**

```bash
# Simple build test
cargo build --release --manifest-path lessons/01-button-neopixel/Cargo.toml 2>&1 | tail -5
```

**If build fails with dependency errors:**

```bash
# Try updating dependencies first (no cd!)
cargo update --manifest-path lessons/XX-name/Cargo.toml
cargo build --release --manifest-path lessons/XX-name/Cargo.toml 2>&1 | tail -10
```

</details>

**Questions:**
- Does it build without errors?
- Are warnings acceptable or should they be fixed?
- Is binary size reasonable (<100KB for simple lessons)?

#### Documentation Review

**Read `lessons/XX-name/README.md`:**

- [ ] Purpose clearly stated
- [ ] Hardware wiring documented with pin numbers
- [ ] Expected behavior described
- [ ] Build/flash instructions present
- [ ] Troubleshooting section included
- [ ] Learning objectives clear
- [ ] Language appropriate (not too complex, not too simple)
- [ ] Length reasonable (<300 lines, as per CLAUDE.md)

**Read `lessons/XX-name/TEST.md` (if exists):**

- [ ] Test procedures defined
- [ ] Success criteria clear
- [ ] Hardware validation steps listed

#### Code Quality Review

**Read `lessons/XX-name/src/bin/main.rs`:**

- [ ] Code follows esp-hal 1.0.0 patterns
- [ ] Comments explain "why" not "what"
- [ ] No hardcoded magic numbers (use constants)
- [ ] Memory safety practices followed
- [ ] Appropriate logging/debugging output
- [ ] Code length reasonable (~100-150 lines for simple lessons)
- [ ] No over-engineering (keep it lean)

**Check dependencies in `Cargo.toml`:**

- [ ] Only necessary dependencies included
- [ ] esp-hal version is 1.0.0
- [ ] `[[bin]]` section present and correct

#### Lesson Progression Check

**Compare to previous lesson:**
- Does complexity increase gradually?
- Does it build on prior concepts?
- Is the jump in difficulty appropriate?

---

### Step 2.2: Specific Lesson Reviews

**Review each lesson for specific concerns:**

#### Lesson 01: Button + NeoPixel
- [ ] Simplest possible intro to GPIO and delays
- [ ] Button input working reliably?
- [ ] NeoPixel timing correct?
- [ ] Good starting point for beginners?

#### Lesson 02: Task Scheduler
- [ ] Task scheduling concepts clear?
- [ ] Code demonstrates cooperative multitasking?
- [ ] Too complex too early? Should it move later?

#### Lesson 03: MPU9250
- [ ] I2C driver clear and well-documented?
- [ ] Sensor initialization explained?
- [ ] Error handling appropriate?

#### Lesson 04: Static Color Navigator
- [ ] Name makes sense? (typo: "statig"?)
- [ ] UI/interaction patterns clear?
- [ ] Dependencies reasonable?

#### Lesson 05: Unit and Integration Testing
- [ ] Test patterns applicable to other lessons?
- [ ] Both unit and integration tests present?
- [ ] Valuable for teaching testing practices?

#### Lesson 06: UART Terminal
- [ ] UART configuration clear?
- [ ] Terminal interaction documented?
- [ ] Prepares well for Lesson 07/08?

#### Lesson 07: GDB Debugging
- [ ] GDB workflow documented?
- [ ] Breakpoints, inspection examples shown?
- [ ] Works with current esp-hal?

#### Lesson 08: UART + GDB Tandem
- [ ] Combines UART streaming + GDB inspection?
- [ ] RAM bounds correct (0x40800000 - 0x40880000)?
- [ ] Memory-safe variable streaming working?
- [ ] Good capstone lesson?

---

## Phase 3: Hardware Testing (Critical)

### Step 3.1: Test Each Lesson on Hardware

**For each lesson, perform actual hardware validation:**

```bash
# Use the test-lesson command
/test-lesson 01
/test-lesson 02
# ... through 08
```

**For each lesson:**
- [ ] Builds successfully
- [ ] Flashes without errors
- [ ] Runs as documented
- [ ] Expected output matches README
- [ ] No unexpected errors or warnings
- [ ] Hardware behavior matches description

**Document any failures** and fix them before proceeding.

---

## Phase 4: AI-Generated Content & Code Quality Review

### Step 4.1: Identify and Remove LLM Artifacts

**Purpose**: Ensure the repository appears professional, not AI-generated. We want clean, simple Rust code showcasing esp-hal 1.0.0 and effective Claude Code usage—not LLM quirks.

**What to look for and ELIMINATE:**

#### 🚫 Exaggerated or Unfounded Claims

**Check all documentation for:**
- [ ] Superlatives without evidence ("revolutionary", "best-in-class", "game-changing")
- [ ] Unsubstantiated performance claims ("10x faster" without benchmarks)
- [ ] Marketing language ("cutting-edge", "next-generation", "industry-leading")
- [ ] Over-promising ("complete guide", "everything you need", "ultimate solution")

**Examples to fix:**
```markdown
❌ "This revolutionary approach to embedded debugging changes everything!"
✅ "This approach combines UART streaming with GDB for hardware debugging."

❌ "The most comprehensive esp-hal tutorial available anywhere!"
✅ "Progressive lessons teaching esp-hal 1.0.0 fundamentals."
```

**Action**: Replace marketing language with factual, technical descriptions.

---

#### 🚫 Excessive Emojis

**Check for emoji overuse:**
- [ ] More than 1-2 emojis per document section
- [ ] Emojis in code comments
- [ ] Emojis in technical explanations
- [ ] Decorative emojis without purpose

**Acceptable emoji use:**
- Status indicators (✅ ❌ ⚠️) in checklists
- Section markers in VERY long documents (sparingly)

**Examples to fix:**
```markdown
❌ "🚀 Amazing GPIO Features! 🎉✨"
✅ "GPIO Features"

❌ "Let's build something awesome! 💪🔥"
✅ "Let's build a button input handler."
```

**Action**: Remove decorative emojis. Keep only functional status markers if needed.

---

#### 🚫 Excessive Dividers (Dashes)

**Check for overuse of visual separators:**
- [ ] Multiple `---` or `***` dividers in short documents
- [ ] Decorative ASCII art borders
- [ ] Overuse of `═══` or `───` patterns

**Acceptable use:**
- Markdown horizontal rules (`---`) for major section breaks (sparingly)
- Code block fences (` ``` `)

**Examples to fix:**
```markdown
❌
---
### Section 1
---
Content here
---
### Section 2
---

✅
### Section 1
Content here

### Section 2
```

**Action**: Limit dividers to major section breaks only. Rely on headers for structure.

---

#### 🚫 Over-Complication

**Check for unnecessary complexity:**
- [ ] Abstractions that obscure learning (in lessons)
- [ ] Over-engineered patterns for simple tasks
- [ ] Verbose explanations when concise would suffice
- [ ] Nested structures deeper than necessary
- [ ] Too many layers of indirection

**Code patterns to simplify:**
```rust
❌ // Over-engineered
pub trait ButtonHandler {
    fn handle_press(&mut self);
}
struct ConcreteButtonHandler;
impl ButtonHandler for ConcreteButtonHandler { ... }

✅ // Simple and direct
pub fn handle_button_press(pin: &Input<GpioPin>) {
    // Direct implementation
}
```

**Documentation to simplify:**
```markdown
❌ "In order to facilitate the initialization of the peripheral subsystem..."
✅ "To initialize the UART peripheral..."
```

**Action**: Prefer directness over cleverness. Use simple patterns for lessons.

---

#### 🚫 Too Many Files

**Check for file proliferation:**
- [ ] Multiple files for single concepts (split unnecessarily)
- [ ] Duplicate information across files
- [ ] Archive/backup files not gitignored
- [ ] Intermediate files left in repo

**Consolidation candidates:**
```
❌ Too fragmented:
docs/gpio.md
docs/gpio-input.md
docs/gpio-output.md
docs/gpio-examples.md

✅ Consolidated:
docs/gpio.md (covers input, output, examples)
```

**Action**: Merge related content. Delete duplicates and archives.

---

#### 🚫 Other LLM Quirks to Remove

**Check for:**
- [ ] **Apologetic language**: "Sorry, but...", "Unfortunately..."
- [ ] **Hedging**: "It seems like...", "Perhaps...", "You might want to..."
- [ ] **Conversational filler**: "Now let's...", "As you can see...", "Moving on..."
- [ ] **Excessive politeness**: "Please note that...", "Kindly ensure..."
- [ ] **Meta-commentary**: "This is a great example of...", "Note how we..."
- [ ] **Redundant warnings**: Multiple "IMPORTANT" or "WARNING" markers

**Examples to fix:**
```markdown
❌ "Now let's move on to the exciting topic of UART! As you can see..."
✅ "## UART Communication"

❌ "Please note that you should kindly ensure the pins are configured."
✅ "Configure the pins before use."

❌ "Unfortunately, the I2C peripheral requires initialization."
✅ "Initialize the I2C peripheral."
```

**Action**: Use technical, direct language. Remove conversational fluff.

---

### Step 4.2: Ensure Professional, Clean Presentation

**Goals:**
- Repository reads like professional embedded systems documentation
- Code examples are clear, idiomatic Rust
- esp-hal 1.0.0 usage is correct and well-explained
- Claude Code integration is demonstrated effectively (but not over-hyped)

**Review each document for:**
- [ ] **Tone**: Technical, informative, not promotional
- [ ] **Clarity**: Direct statements, no unnecessary words
- [ ] **Structure**: Headers and content, minimal decorations
- [ ] **Accuracy**: Claims are backed by code/hardware validation

**What GOOD looks like:**

```markdown
# Lesson 01: Button Input and NeoPixel Control

This lesson demonstrates GPIO input (button) and RMT output (NeoPixel LED).

## Hardware Requirements

- ESP32-C6 development board
- Push button connected to GPIO 9
- WS2812 NeoPixel LED connected to GPIO 8
- Pull-up resistor for button (internal or external)

## Concepts Covered

- GPIO input configuration
- Debouncing techniques
- RMT peripheral for precise timing
- WS2812 protocol basics

## Build and Flash

```bash
cargo build --release
cargo run --release
```

## Expected Behavior

- Press button: NeoPixel cycles through colors (red → green → blue)
- Release button: LED turns off
```

**This is clean, factual, sufficient.**

---

### Step 4.3: Validate Claude Code Integration

**Ensure .claude/ infrastructure showcases good practices:**

- [ ] **Commands are focused**: Each does one thing well
- [ ] **No command bloat**: Consolidate overlapping functionality
- [ ] **Templates are minimal**: uart_test_minimal.rs should be ~73 lines
- [ ] **Scripts are robust**: Handle errors, provide clear output
- [ ] **CLAUDE.md is authoritative**: Guidelines match reality, no fluff

**Questions to answer:**
- Does the .claude/ setup demonstrate value without being overwhelming?
- Are commands documented clearly (purpose, usage, examples)?
- Can someone new to Claude Code understand what each command does?

**Action**: Trim commands, merge duplicates, remove unused infrastructure.

---

## Phase 5: Documentation Audit

### Step 5.1: Check Documentation Completeness

**Review all markdown files:**

```bash
find . -name "*.md" -type f | grep -v target | grep -v node_modules
```

**For each document:**
- [ ] Grammar and spelling correct
- [ ] Technical accuracy verified
- [ ] Links work (no broken references)
- [ ] Code examples match current esp-hal 1.0.0
- [ ] Tone appropriate for target audience

### Step 5.2: Validate Inline Documentation

**Check code comments across all lessons:**

- [ ] Comments explain "why" not "what"
- [ ] Complex sections have explanatory comments
- [ ] No outdated or misleading comments
- [ ] Function/struct docs present where needed
- [ ] **No LLM-style comments**: Remove conversational or apologetic comments in code

---

## Phase 6: Simplification & Clarity Pass

**Note:** Much of this is covered in Phase 4. Focus on code/pattern simplification here.

### Step 6.1: Identify Over-Complexity

**Review each lesson for unnecessary complexity:**

**Questions:**
- Can code be simplified without losing educational value?
- Are there abstractions that obscure learning?
- Is language in docs too technical/verbose?
- Are there "clever" patterns that should be straightforward?

**Actions:**
- [ ] List lessons that need simplification
- [ ] Identify verbose documentation to trim
- [ ] Find over-engineered code to refactor

### Step 6.2: Enhance Clarity

**Review for clarity improvements:**

**Questions:**
- Are learning objectives explicit?
- Do examples clearly demonstrate concepts?
- Are error messages helpful?
- Is troubleshooting guidance comprehensive?

**Actions:**
- [ ] List areas needing clearer explanations
- [ ] Identify confusing code patterns
- [ ] Note missing examples or diagrams

---

## Phase 7: Firmware & Software Gaps Analysis

### Step 7.1: Firmware Side Review

**Questions:**
- **Peripheral Coverage**: Are key ESP32-C6 peripherals covered?
  - GPIO ✓ (Lesson 01)
  - UART ✓ (Lessons 06, 08)
  - I2C ✓ (Lesson 03)
  - SPI? (missing?)
  - Timers? (in Lesson 02, but dedicated lesson?)
  - ADC? (missing?)
  - PWM? (missing?)
  - WiFi? (missing?)
  - BLE? (missing?)

- **Advanced Concepts**: What's missing?
  - DMA? (planned in Lesson 08 v2?)
  - Interrupts? (touched in Lesson 01, but dedicated?)
  - Sleep modes?
  - RTC?
  - Flash storage?

- **Debugging Techniques**: Fully covered?
  - esp-println ✓
  - GDB ✓ (Lesson 07)
  - UART streaming ✓ (Lesson 08)
  - probe-rs?
  - RTT? (replaced with UART)

**Output**: List of firmware topics to add

---

### Step 7.2: Software Side Review

**Questions:**
- **Tooling**: What tools are missing?
  - Python UART reader ✓
  - GDB scripts? (Python API usage?)
  - Log analyzers?
  - Test automation?
  - CI/CD? (GitHub Actions?)

- **Scripts**: What utilities would help?
  - Port detection ✓ (find-esp32-ports.sh)
  - Automated testing?
  - Log parsing/visualization?
  - Performance profiling?

- **Documentation Tools**:
  - API docs generation? (rustdoc)
  - Lesson dependency graph?
  - Hardware setup diagrams?

**Output**: List of software/tooling to add

---

## Phase 8: Community Readiness

### Step 8.1: Contribution Guidelines

**Check if repo has:**
- [ ] LICENSE file
- [ ] Issue templates (optional)
- [ ] PR templates (optional)

**Note:** CONTRIBUTING.md and CODE_OF_CONDUCT.md are intentionally NOT included in this project.

### Step 8.2: Example Projects

**Check if there are:**
- [ ] Real-world example projects
- [ ] Reference implementations
- [ ] Common use-case templates

### Step 8.3: External Dependencies

**Audit external dependencies:**
- [ ] All crates are from crates.io (no git dependencies)
- [ ] Version pins are appropriate
- [ ] No unnecessary dependencies

---

## Phase 9: Final Cleanup & Organization

### Step 9.1: Delete Excess Files

**Identify files to remove:**
```bash
# Find potential cruft
find . -name "*.bak" -o -name "*.tmp" -o -name ".DS_Store"
find . -type d -name "target" -o -name "node_modules"
```

**Check for:**
- [ ] Orphaned test files
- [ ] Unused scripts
- [ ] Old/deprecated code
- [ ] Backup files
- [ ] Build artifacts in git

### Step 9.2: Git Hygiene

**Review git status:**
- [ ] No uncommitted changes
- [ ] .gitignore comprehensive
- [ ] No large binaries in history
- [ ] Commit messages descriptive
- [ ] Branches organized

### Step 9.3: Final Structure Validation

**Ensure consistent structure across all lessons:**

```
lessons/XX-name/
├── src/
│   ├── bin/
│   │   └── main.rs
│   └── lib.rs (optional)
├── .cargo/
│   └── config.toml
├── Cargo.toml
├── rust-toolchain.toml
├── build.rs (if needed)
├── README.md
└── TEST.md (if applicable)
```

---

## Phase 10: Generate Review Report

### Step 10.1: Summarize Findings

**Create a comprehensive report with:**

#### Section 1: Lessons Status
- Table showing each lesson's build/test status
- List of lessons needing fixes
- Recommendations for lesson order/numbering

#### Section 2: AI-Generated Content Issues
- LLM artifacts found and removed:
  - Exaggerated claims count
  - Excessive emojis removed
  - Divider overuse cleaned up
  - Over-complicated patterns simplified
  - Conversational fluff eliminated
- File consolidation recommendations
- .claude/ command consolidation plan

#### Section 3: Documentation Quality
- CLAUDE.md improvements needed
- README.md status (exists? complete?)
- Lesson docs that need updates
- New documentation needed

#### Section 4: Code Quality
- Lessons following best practices
- Code needing refactoring
- Warnings to address
- Performance concerns

#### Section 5: Missing Features
- Firmware side gaps (peripherals, concepts)
- Software side gaps (tools, scripts)
- Infrastructure improvements

#### Section 6: Community Readiness
- Contribution guidelines status
- License/legal compliance
- Example projects needed
- Issue/PR templates

#### Section 7: Cleanup Actions
- Files to delete
- Directories to reorganize
- Git cleanup needed

#### Section 8: Priority Recommendations
1. **Critical** (must fix before sharing)
   - Missing LICENSE
   - Inaccurate README
   - LLM artifacts that hurt professionalism
2. **Important** (should fix soon)
   - Command consolidation
   - File cleanup
   - Documentation polish
3. **Nice-to-have** (future improvements)
   - Additional peripherals
   - CI/CD setup
   - Visual aids

---

## Execution Instructions

**When this command is run, Claude should:**

1. **Create todo list** with all phases (use TodoWrite)
2. **Work systematically** through each phase
3. **Document findings** as you go (create `/tmp/review-findings.md`)
4. **Ask questions** when decisions needed (use AskUserQuestion)
5. **Test on hardware** where possible (use /test-lesson)
6. **Summarize at end** with actionable recommendations

**Estimated time**: 2-4 hours for thorough review

---

## Output Deliverables

At completion, provide:

1. **Review Report** (`/tmp/esp32-c6-repo-review-YYYY-MM-DD.md`)
   - Executive summary
   - Detailed findings by phase
   - Prioritized action items

2. **Lesson Status Matrix**
   ```
   | Lesson | Build | Flash | Run | Docs | Tests | Status |
   |--------|-------|-------|-----|------|-------|--------|
   | 01     | ✓     | ✓     | ✓   | ✓    | -     | PASS   |
   | 02     | ✓     | ✗     | -   | ✓    | -     | FAIL   |
   ...
   ```

3. **Action Plan** (prioritized list of fixes/improvements)

4. **Questions for User** (decisions needed before proceeding)

---

## Notes

- **Be thorough but efficient** - don't get stuck on minor details
- **Test on actual hardware** - this is critical for validation
- **Ask questions early** - don't guess at user intent
- **Document everything** - findings should be actionable
- **Prioritize ruthlessly** - separate critical from nice-to-have

---

**Usage**:
```bash
/review-repo
```

**Follow-up commands after review:**
```bash
# Fix specific lesson
/gen-lesson "Fix Lesson 02 based on review findings"

# Update documentation
# (manual editing of CLAUDE.md, README.md)

# Re-test after fixes
/test-lesson 02
```
