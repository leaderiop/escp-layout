# Fuzzing Targets for ESC/P Layout Engine

This directory contains fuzzing targets for the escp-layout library using `libfuzzer-sys`.

## Prerequisites

1. **Nightly Rust** (required for fuzzing):
   ```bash
   rustup install nightly
   ```

2. **cargo-fuzz** (fuzzing tool):
   ```bash
   cargo install cargo-fuzz
   ```

## Available Fuzz Targets

### 1. `fuzz_region` (T040)

Fuzzes the `Region` type and all its operations:
- Region creation with arbitrary x, y, width, height values
- `split_vertical()` with arbitrary split heights
- `split_horizontal()` with arbitrary split widths
- `with_padding()` with arbitrary padding values
- Accessor methods

**Purpose**: Verify no panics occur with any input combination, and all geometry validation works correctly.

### 2. `fuzz_render` (T085)

Fuzzes the complete rendering pipeline:
- Page creation with arbitrary write operations
- Multi-page documents (0-5 pages)
- Arbitrary cell positions and characters
- Random style combinations (NONE, BOLD, UNDERLINE, BOLD+UNDERLINE)
- ESC/P byte stream generation and validation

**Purpose**: Verify deterministic rendering, proper ESC/P output format, and no panics during rendering.

## Running Fuzz Tests

### Quick Test (1000 iterations, 5 seconds)

```bash
# From repository root
cd /Users/mohammadalmechkor/Projects/matrix

# Fuzz Region operations
cargo +nightly fuzz run fuzz_region -- -runs=1000 -max_total_time=5

# Fuzz rendering pipeline
cargo +nightly fuzz run fuzz_render -- -runs=1000 -max_total_time=5
```

### Comprehensive Test (1M+ iterations, recommended)

```bash
# Region fuzzing (10 minutes)
cargo +nightly fuzz run fuzz_region -- -runs=1000000 -max_total_time=600

# Render fuzzing (10 minutes)
cargo +nightly fuzz run fuzz_render -- -runs=1000000 -max_total_time=600
```

### Continuous Fuzzing (for CI)

```bash
# Run until crash found or manually stopped
cargo +nightly fuzz run fuzz_region

# Or with timeout
cargo +nightly fuzz run fuzz_region -- -max_total_time=3600  # 1 hour
```

## Expected Results

Both fuzz targets should run without finding any crashes:

```
INFO: -max_total_time: 5
INFO: Running fuzz target 'fuzz_region'...
#1000	DONE   cov: 234 ft: 456 corp: 123 exec/s: 200 rss: 45Mb
```

**Success Criteria**:
- ✅ No panics or crashes
- ✅ All operations return Result or silently truncate as designed
- ✅ ESC/P output always has valid structure (initialization + content + form-feeds)

## Understanding Fuzzing Output

- **cov**: Code coverage (higher is better)
- **ft**: Features covered (unique code paths)
- **corp**: Corpus size (interesting test cases saved)
- **exec/s**: Executions per second (performance)
- **rss**: Memory usage

## Corpus Storage

Interesting test cases are saved in:
```
fuzz/corpus/fuzz_region/
fuzz/corpus/fuzz_render/
```

These can be used for regression testing.

## Debugging Crashes

If a crash is found:

1. **Reproduce the crash**:
   ```bash
   cargo +nightly fuzz run fuzz_region fuzz/artifacts/fuzz_region/crash-abc123
   ```

2. **Minimize the input**:
   ```bash
   cargo +nightly fuzz tmin fuzz_region fuzz/artifacts/fuzz_region/crash-abc123
   ```

3. **Debug with coverage**:
   ```bash
   cargo +nightly fuzz coverage fuzz_region
   ```

## Integration with CI

Add to CI pipeline:

```yaml
- name: Run fuzz tests
  run: |
    rustup install nightly
    cargo install cargo-fuzz
    cargo +nightly fuzz run fuzz_region -- -runs=100000 -max_total_time=300
    cargo +nightly fuzz run fuzz_render -- -runs=100000 -max_total_time=300
```

## Constitution Compliance

These fuzz targets verify:
- **Principle IX: Zero-Panic Guarantee** - No panics with any input
- **Principle I: Deterministic Behavior** - Same input always produces same output
- **Principle III: Strict Truncation** - Silent truncation, never errors

## Performance Notes

Fuzzing performance on typical hardware:
- **fuzz_region**: ~10,000-50,000 exec/s
- **fuzz_render**: ~5,000-20,000 exec/s (more complex)

Reaching 1M iterations:
- **fuzz_region**: ~30-60 seconds
- **fuzz_render**: ~1-3 minutes

---

**Status**: ✅ Fuzz targets implemented and compiling successfully
**Last Updated**: 2025-11-18
