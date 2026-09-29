# Next Learning Slices

This plan sequences the next three Rust Foundations units after **Slices**. Each slice should be delivered end to end before starting the next: curriculum content, runnable exercise, roadmap integration, executor support, and learner-flow verification.

## Shared implementation requirements

- Use the stable, versioned content ID convention (`*.v1`) and the existing unit/exercise JSON formats.
- Add each new unit to `content/roadmaps/foundations.json` in the order below. Make its prerequisite IDs point to the preceding unit so roadmap validation and learner progression remain coherent.
- Give each unit focused learning objectives, an explanation, runnable examples, relevant compiler-error examples, and completion criteria.
- For each exercise, provide a starter template, tests, expected solution, and progressive hints. Ensure the starter compiles before learner edits and the expected solution passes its tests.
- Add each exercise ID to the executor's template mapping and cover submission/progress behavior using the existing integration-test patterns.
- Run `npm run content:validate` while iterating and `npm run validate` before considering the slices complete; integration validation requires `TEST_DATABASE_URL`.

## 1. Types

- **Slug:** `types`
- **Prerequisite:** `unit.rust.slices.v1`
- **Content IDs:** `unit.rust.types.v1`; `exercise.rust.types.tuple-basics.v1`

### Learning outcomes

- Recognize Rust's common scalar types, including booleans, characters, and numeric types.
- Use tuples and arrays as fixed-size compound types and access their elements.
- Apply type annotations when inference is insufficient and explain basic numeric type mismatches.

### Lesson and exercise scope

Introduce scalar types and inference first, then tuples and arrays, distinguishing fixed-size arrays from dynamically sized collections without teaching `Vec` in depth. Include examples of tuple destructuring, array indexing, and a compiler diagnostic caused by an incompatible type.

The exercise should construct a small typed record from a tuple and fixed-size array, then return or calculate a result that requires choosing the right type. Tests should cover ordinary values and a boundary case such as an empty/zero value or maximum supported index.

### Completion checks

- The learner can identify the types used in the exercise and explain where an explicit annotation is useful.
- The learner can access tuple and array elements without confusing their fixed-size behavior.
- The exercise's starter and expected solution satisfy the shared implementation requirements.

## 2. Pattern Matching

- **Slug:** `pattern-matching`
- **Prerequisite:** `unit.rust.types.v1`
- **Content IDs:** `unit.rust.pattern-matching.v1`; `exercise.rust.pattern-matching.classify-value.v1`

### Learning outcomes

- Use `match` to handle every case of an enum or other matched value.
- Define and construct a simple enum, including variants that carry data.
- Destructure values in match arms and use a guarded or catch-all arm where appropriate.
- Recognize why exhaustive matching prevents unhandled cases.

### Lesson and exercise scope

Build from the existing introductory `match` example in Control Flow, then introduce custom enums and data-bearing variants. Focus on exhaustive matching and destructuring; keep `Option`/`Result` error-handling patterns and advanced pattern syntax for later units.

The exercise should classify a domain value represented by an enum, including at least one data-bearing variant. Tests should cover every variant and confirm that the learner extracts or uses the carried value correctly.

### Completion checks

- The learner can explain why each enum variant must be handled.
- The exercise tests exercise all variants and include a carried-data case.
- The expected solution passes, and the starter gives a clear, compiling task scaffold.

## 3. Modules

- **Slug:** `modules`
- **Prerequisite:** `unit.rust.pattern-matching.v1`
- **Content IDs:** `unit.rust.modules.v1`; `exercise.rust.modules.public-api.v1`

### Learning outcomes

- Organize related Rust items in a module.
- Distinguish public and private items and use `pub` to expose a supported API.
- Refer to items with paths and bring names into scope with `use`.
- Interpret a visibility or unresolved-path compiler error.

### Lesson and exercise scope

Introduce inline modules and module paths within a crate, then demonstrate visibility and `use`. Keep filesystem-based module layouts and multi-crate work out of scope for this first module unit unless the exercise template makes them necessary.

The exercise should complete a small public API over an internal module, requiring the learner to expose the intended function and call it through a path or import. Tests should verify the public behavior without relying on private implementation details.

### Completion checks

- The learner can distinguish an item's definition path from the path used to access it.
- The exercise demonstrates a meaningful public/private boundary and passes its tests.
- The learner-facing compiler feedback identifies a visibility or path issue relevant to the lesson.

## Integration and release of the sequence

After each unit is implemented, verify that it appears after its prerequisite in the Foundations roadmap, loads through the existing unit/exercise flow, and can be submitted and reflected in learner progress. Once all three are integrated, check the full roadmap ordering and prerequisite graph, then complete the existing validation and MVP smoke-flow checks.
