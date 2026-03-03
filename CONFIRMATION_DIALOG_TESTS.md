# File Creation Confirmation Dialog - Testing Guide

## Prerequisites

1. **Build the application**:
   ```bash
   cd /workspace/jashan/rust_agent/rusty_tui
   cargo build --release
   ```

2. **Verify ClaudeProxyAPI is running** (optional but recommended):
   ```bash
   curl http://localhost:8317/health
   ```

3. **Clear test files**:
   ```bash
   cd /workspace/jashan/rust_agent
   rm -f main.rs test.rs hello.rs lib.rs Cargo.toml
   ```

## Test Cases

### Test 1: Single File Creation ✅
**Steps**:
1. Start: `./rusty_tui/target/release/rusty`
2. Type: `Create a simple hello world program in Rust`
3. Press Enter

**Expected**:
- Dialog appears with `📄 main.rs`
- Preview shows first 3 lines
- Click "✅ Create Files"
- File created successfully

**Verify**: `ls -l main.rs && cat main.rs`

---

### Test 2: Multiple Files ✅
**Steps**:
1. Type: `Create a Rust library with Cargo.toml and lib.rs`
2. Press Enter

**Expected**:
- Dialog shows both files with previews
- Click "Create Files"
- Both files created

**Verify**: `ls -l Cargo.toml lib.rs`

---

### Test 3: Cancellation ✅
**Steps**:
1. Type: `Create test.rs with a simple function`
2. Press Enter
3. Click "❌ Cancel"

**Expected**:
- Dialog closes
- Message: "File creation cancelled."
- No file created

**Verify**: `ls test.rs` (should fail)

---

### Test 4: Enter Key Shortcut ✅
**Steps**:
1. Type: `Create hello.rs`
2. Press Enter
3. When dialog appears, press Enter again

**Expected**:
- File created (same as clicking button)

---

### Test 5: Escape Key Shortcut ✅
**Steps**:
1. Type: `Create cancel_test.rs`
2. Press Enter
3. Press Escape key

**Expected**:
- Dialog closes
- File cancelled

---

### Test 6: File Exists (Modify Fallback) ✅
**Setup**: `echo "// Old" > existing.rs`

**Steps**:
1. Type: `Create existing.rs with new content`
2. Approve in dialog

**Expected**:
- Message: "✏️ Modified file: existing.rs"
- Content updated

---

### Test 7: Security - Absolute Paths ✅
**Steps**:
1. Type: `Create /etc/passwd with content "test"`
2. Approve in dialog

**Expected**:
- Error: "Absolute paths not allowed"
- File NOT created

---

### Test 8: No Code Blocks ✅
**Steps**:
1. Type: `What is ownership in Rust?`

**Expected**:
- Normal response
- NO dialog appears

---

### Test 9: Large File Preview ✅
**Steps**:
1. Type: `Create a program with 10 functions`

**Expected**:
- Preview shows first 3 lines
- Shows "... (N more lines)"

---

### Test 10: Multiple Files Scrollable ✅
**Steps**:
1. Type: `Create main.rs, lib.rs, Cargo.toml, README.md, tests/test.rs`

**Expected**:
- All files shown
- Scrollbar if needed
- All created after approval

---

## Success Checklist

- [ ] Dialog appears for code blocks
- [ ] Preview shows first 3 lines
- [ ] Create button works
- [ ] Cancel button works
- [ ] Enter key approves
- [ ] Escape key cancels
- [ ] File modification fallback works
- [ ] Security checks enforced
- [ ] No dialog for non-code responses
- [ ] Large file preview truncates

## Cleanup

```bash
cd /workspace/jashan/rust_agent
rm -f main.rs test.rs hello.rs lib.rs existing.rs Cargo.toml README.md
rm -rf tests/
```
