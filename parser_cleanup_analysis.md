# Parser Cleanup Analysis

## 🚨 Legacy/Outdated Code Found

### 1. **`clean_text()` function (lines 856-957)**
**Status**: Likely obsolete
**Purpose**: Removing spaces between numbers and CSS units (e.g., "2 em" → "2em")
**Why obsolete**: This was likely needed when CSS was being processed through the Rust tokenizer, which inserted spaces. With the current `<style src>` approach, CSS is read as raw files, so this cleanup shouldn't be needed.

**Evidence**: The function does complex character-by-character processing that seems unnecessary for the current CSS handling approach.

### 2. **Commented Debug Statements**
**Status**: Leftover debug code
**Examples**: 
- Line 237: `// println!("parse_script_nodes starting. Input len: {}", input.len());`
- Lines 244, 262, 270, etc.: Multiple commented debug prints
- Line 434: `// panic!("parse_call called with: {}", input);`

### 3. **`parse_component_var()` function (lines 473-480)**
**Status**: Possibly obsolete
**Purpose**: Parses component variables without parentheses
**Issue**: Modern Azumi uses `@ComponentName()` syntax, so this might be unused.

### 4. **Overly Complex Script/Style Parsing**
**Status**: Over-engineered for current use case
**Examples**:
- `parse_script_nodes()` handles loose closing tags (`</ script >` with spaces)
- Complex parsing for edge cases that may not occur in real usage
- Lines 242-259: Handling for "loose closing tags" with spaces

### 5. **Redundant Attribute Parsing Logic**
**Status**: Overly complex for current requirements
**Examples**:
- Extended identifier parsing with hyphens and colons
- Complex path parsing for module paths
- Multiple parsing strategies that may not all be needed

## ✅ Still Relevant Code

### 1. **Core Node Structure**
- Element, Text, Expression, Call, If, For, Match parsing
- Basic HTML element parsing
- Attribute parsing (though could be simplified)

### 2. **Control Flow Parsing**
- `@if`, `@for`, `@match` parsing logic
- These are core features that are actively used

### 3. **Style Source Handling**
- `<style src="file.css" />` parsing (lines 144-152)
- This is a current, important feature

## 🔧 Recommended Cleanup

### **High Priority (Remove)**
1. **`clean_text()` function** - Likely obsolete CSS cleanup code
2. **Commented debug statements** - Clean up debug code
3. **Over-complex script/style parsing** - Simplify for real-world usage

### **Medium Priority (Review)**
1. **`parse_component_var()`** - Check if still needed
2. **Extended identifier parsing** - May be overkill

### **Architecture Concern**
The parser seems to be a duplicate of functionality that might be better handled by the `token_parser.rs` file. The architecture might need consolidation.

## 🧪 Verification Needed

Before removing anything:
1. Test current functionality without these features
2. Check if `clean_text()` is actually called anywhere
3. Verify `parse_component_var()` usage
4. Test edge cases for script/style parsing