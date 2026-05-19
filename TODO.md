# ruslink - TODO & Future Roadmap

## Overview

This document outlines planned features, improvements, and security enhancements for ruslink. Items are prioritized by impact and implementation complexity.

---

## 📋 Current Backlog

### Phase 1: Security Enhancements ⚠️ [HIGH PRIORITY]

#### 1.1 Add Security Section to README

**Description:** Create comprehensive security documentation covering validation, backup strategies, and safety guarantees.

**Items:**
- [ ] **Symlink Validation**
  - [ ] Validate symlink targets before creation
  - [ ] Detect circular symlinks
  - [ ] Prevent symlinks pointing outside target directory (if desired)
  - [ ] Document symlink safety checks
  - [ ] Add `--validate` flag to verify existing symlinks

- [ ] **Backup Strategy Documentation** ✅ Decided: Complementary to Git
  - [ ] Document when to use `--backup` vs `--git`
  - [ ] Explain backup location and recovery procedures
  - [ ] Create section: "Backup vs Git Versioning: Defense in Depth"
  - [ ] Add examples combining `--backup` and `--git`
  - [ ] Document backup file naming convention (.bak, .bak1, .bak2)
  - [ ] Add recovery workflows (CLI commands to restore from backups)
  - [ ] Clarify: backup is fallback for non-git environments, not replacement

- [ ] **Safety Guarantees**
  - [ ] Document what ruslink guarantees (reversibility, atomicity)
  - [ ] Explain what ruslink does NOT guarantee
  - [ ] Add section: "Recovery Procedures"
    - [ ] Recovering from failed stow
    - [ ] Recovering from `--adopt` with `--backup`
    - [ ] Recovering from `--merge` operations
    - [ ] Restoring from git history
    - [ ] Restoring from local backups
  - [ ] Add section: "Known Limitations"
  - [ ] Document race conditions (if any) in concurrent usage

**Rationale:**
- Backup is **NOT a replacement for git**, but a **complementary safety measure**
- Backup works in non-git environments (production systems, non-dotfiles packages)
- Combined `--backup` + `--git` provides "defense in depth"
- Users need clear guidance on when to use each strategy
- Recovery documentation reduces support requests

**Acceptance Criteria:**
- [ ] Security section added to README
- [ ] Backup vs Git comparison table created
- [ ] 3+ recovery workflow examples documented
- [ ] All safety guarantees clearly stated
- [ ] All known limitations documented

---

### Phase 2: Additional Security Features ⚠️ [MEDIUM PRIORITY]

#### 2.1 Enhanced Validation

**Description:** Implement additional validation mechanisms beyond current symlink checks.

**Items:**
- [ ] **Pre-stow Validation**
  - [ ] Check for sufficient disk space
  - [ ] Verify write permissions on all target paths
  - [ ] Detect permission changes after symlink creation
  - [ ] Validate package directory integrity

- [ ] **Post-stow Verification**
  - [ ] Verify all symlinks created correctly
  - [ ] Verify file integrity (checksums if `--backup`)
  - [ ] Generate verification report
  - [ ] Add `--verify` flag to check existing installations

- [ ] **Symlink Integrity Checks**
  - [ ] Detect broken symlinks early
  - [ ] Detect symlinks pointing to wrong locations
  - [ ] Detect and warn about circular dependencies
  - [ ] Validate symlink permissions

**Acceptance Criteria:**
- [ ] `--validate` flag implemented
- [ ] `--verify` flag implemented
- [ ] All validations tested on Windows/Linux/macOS
- [ ] Performance impact < 5% on typical packages

---

#### 2.2 Atomic Operations

**Description:** Ensure stow operations are atomic (all-or-nothing).

**Items:**
- [ ] **Transactional Stowing**
  - [ ] Implement rollback mechanism on error
  - [ ] Use temporary staging directory for symlinks
  - [ ] Only commit changes if all operations succeed
  - [ ] Document atomicity guarantees

- [ ] **Partial Recovery**
  - [ ] Track which files were successfully stowed
  - [ ] Provide recovery command for partial failures
  - [ ] Log all operations with timestamps

**Acceptance Criteria:**
- [ ] All stow operations atomic or documented as non-atomic
- [ ] Rollback tested for 10+ failure scenarios
- [ ] Recovery procedures documented

---

#### 2.3 Audit Logging

**Description:** Comprehensive audit trail for compliance and troubleshooting.

**Items:**
- [ ] **Structured Logging**
  - [ ] Log all file operations (create, modify, delete)
  - [ ] Include timestamps, user, package, operation
  - [ ] Configurable log levels and output formats
  - [ ] JSON export for analysis

- [ ] **Audit Log File**
  - [ ] Create `.ruslink-audit.log` (optional, flag-controlled)
  - [ ] Include all destructive operations
  - [ ] Maintain history across runs
  - [ ] Add `--audit-log` flag

**Acceptance Criteria:**
- [ ] Audit logs created with `--audit-log` flag
- [ ] JSON format supported
- [ ] Rotation/retention policies documented

---

### Phase 3: User Experience & Documentation

#### 3.1 Configuration File Support

**Description:** Allow persistent configuration via `.ruslink.toml` or similar.

**Items:**
- [ ] **Configuration Format**
  - [ ] Design `.ruslink.toml` schema
  - [ ] Support per-package overrides
  - [ ] Support global defaults

- [ ] **Configuration Loading**
  - [ ] Auto-detect and load configuration
  - [ ] CLI flags override config file
  - [ ] Validate configuration on startup

**Acceptance Criteria:**
- [ ] `.ruslink.toml` schema defined
- [ ] Configuration loading implemented
- [ ] Examples documented

---

#### 3.2 Interactive Mode

**Description:** Interactive prompts for conflicting operations.

**Items:**
- [ ] **Interactive Conflicts**
  - [ ] Show side-by-side diff of conflicting files
  - [ ] Allow choosing action per-file
  - [ ] Remember choices for batch operations

- [ ] **Interactive Package Selection**
  - [ ] Multi-select packages
  - [ ] Filter/search packages
  - [ ] Preview changes before confirmation

**Acceptance Criteria:**
- [ ] Interactive mode works on all platforms
- [ ] 3+ test scenarios pass

---

### Phase 4: Advanced Features

#### 4.1 Hooks & Scripts

**Description:** Allow custom pre/post operation hooks.

**Items:**
- [ ] **Hook System**
  - [ ] Define hook types (pre-stow, post-stow, pre-unstow, post-unstow)
  - [ ] Support shell scripts and binaries
  - [ ] Hook failure handling options (fail/warn/ignore)

- [ ] **Hook Configuration**
  - [ ] `.ruslink-hooks/` directory
  - [ ] `hooks.toml` configuration
  - [ ] Environment variables passed to hooks

**Acceptance Criteria:**
- [ ] 3+ hook types working
- [ ] Hooks tested on Linux/macOS/Windows
- [ ] Documentation with examples

---

#### 4.2 Template Support

**Description:** Variable substitution in files during stowing.

**Items:**
- [ ] **Template Engine**
  - [ ] Support simple variable syntax (e.g., `${VARIABLE}`)
  - [ ] Built-in variables: `${HOME}`, `${USER}`, `${HOSTNAME}`, `${DATE}`
  - [ ] Custom variable definition per-package

- [ ] **Template Processing**
  - [ ] Mark files as templates (`.template` suffix)
  - [ ] Process templates during stowing
  - [ ] Cache processed files

**Acceptance Criteria:**
- [ ] Basic variable substitution working
- [ ] 5+ built-in variables
- [ ] Performance acceptable (< 10ms overhead per file)

---

#### 4.3 Encrypted Secrets

**Description:** Support for encrypted files in packages.

**Items:**
- [ ] **Encryption Support**
  - [ ] Detect encrypted files (`.enc` suffix)
  - [ ] Decrypt during stow (requires key)
  - [ ] Support multiple encryption formats

- [ ] **Key Management**
  - [ ] Load keys from file/environment/prompt
  - [ ] Support key rotation

**Acceptance Criteria:**
- [ ] One encryption format supported
- [ ] Key management functional
- [ ] Security audit passed

---

### Phase 5: Performance & Optimization

#### 5.1 Caching & Incremental Updates

**Items:**
- [ ] **Operation Cache**
  - [ ] Cache ignore patterns per-package
  - [ ] Cache symlink targets
  - [ ] Implement cache invalidation

- [ ] **Incremental Stowing**
  - [ ] Skip unchanged files
  - [ ] Use file hashes/timestamps to detect changes
  - [ ] Benchmark: `ruslink` vs `GNU Stow` on large packages

**Acceptance Criteria:**
- [ ] 30% performance improvement on large packages
- [ ] Cache hits logged in verbose mode
- [ ] No correctness regressions

---

#### 5.2 Parallel Processing

**Items:**
- [ ] **Concurrent Operations**
  - [ ] Use Rayon for parallel file traversal
  - [ ] Parallel symlink creation
  - [ ] Thread-safe state management

**Acceptance Criteria:**
- [ ] 40% faster on multi-core systems
- [ ] Tested with 4/8/16 cores
- [ ] No ordering dependencies broken

---

### Phase 6: Platform Support

#### 6.1 Improved Windows Support

**Items:**
- [ ] **Junction Links**
  - [ ] Use `mklink /J` for directory symlinks on Windows
  - [ ] Document limitations on Windows 10 < Build 14972
  - [ ] Test on Windows Server

- [ ] **Permission Handling**
  - [ ] Handle Windows ACLs
  - [ ] Deal with read-only files
  - [ ] Test with UAC enabled/disabled

**Acceptance Criteria:**
- [ ] Full feature parity with Linux/macOS on Windows 11
- [ ] Tested on Windows 10 and Windows Server

---

#### 6.2 macOS Specific Features

**Items:**
- [ ] **Extended Attributes**
  - [ ] Preserve `.DS_Store` handling
  - [ ] Preserve extended attributes (metadata)
  - [ ] Handle quarantine bits

**Acceptance Criteria:**
- [ ] Extended attributes preserved
- [ ] No Gatekeeper issues

---

### Phase 7: Testing & Quality

#### 7.1 Test Coverage

**Items:**
- [ ] Achieve 80%+ code coverage
- [ ] Add integration tests for all conflict scenarios
- [ ] Add stress tests (large packages, deep trees)
- [ ] Add regression tests for reported bugs

**Acceptance Criteria:**
- [ ] Code coverage ≥ 80%
- [ ] CI passing on all platforms

---

#### 7.2 Performance Benchmarks

**Items:**
- [ ] Create benchmark suite
- [ ] Compare against `GNU Stow` and `Chezmoi`
- [ ] Document performance characteristics
- [ ] Track performance across versions

**Acceptance Criteria:**
- [ ] Benchmark suite functional
- [ ] Comparison with GNU Stow completed
- [ ] Results published in README

---

## 🔍 Known Limitations & Workarounds

### Current Limitations

1. **No Configuration File Support** — Use environment variables or shell aliases
2. **No Hook/Script Support** — Use external wrapper scripts
3. **No Template Variables** — Copy files with correct values
4. **No Encryption** — Use `git-crypt` or `transcrypt` for git repos
5. **Limited Windows Support** — Requires Windows 10 Build 14972+

---

## 📅 Release Timeline (Estimated)

- **v0.6.x** — Security Documentation (Phase 1) — Q3 2025
- **v0.7.x** — Enhanced Validation (Phase 2.1) — Q4 2025
- **v0.8.x** — Configuration File Support (Phase 3.1) — Q2 2026
- **v0.9.x** — Hook System (Phase 4.1) — Q4 2026
- **v1.0.0** — Stable Release with Core Features — Q2 2027
- **v1.1.x** — Advanced Features (Phase 4.2-4.3) — Q4 2027
- **v1.2.x** — Performance Optimization (Phase 5) — Q2 2028

---

## 🤝 Contributing

Help us implement these features! Areas needing contribution:

- **Documentation** — Improve guides and examples
- **Testing** — Write tests for edge cases
- **Performance** — Optimize hot paths
- **Platforms** — Test and fix platform-specific issues
- **UX** — Improve error messages and prompts

See [CONTRIBUTING.md](CONTRIBUTING.md) for details.

---

## 📝 Notes

- Items in Phase 1 (Security) are critical for v0.5.x
- Items in Phase 2-3 are planned for v0.6-0.8
- Items in Phase 4+ are optional for v1.0 (nice-to-have)
- Community feedback may reprioritize items
- Marked items (✅) indicate decisions already made

---

**Last Updated:** May 2024  
**Status:** DRAFT  
**Owner:** ruslink maintainers
