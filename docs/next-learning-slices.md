# Next Learning Slices

This plan records the Rust Foundations sequence through **Closures and Iterators** and the next units after it. Each slice should be delivered end to end: curriculum content, runnable exercise, roadmap integration, executor support, and learner-flow verification.

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

## 4. Structs

- **Slug:** `structs`
- **Prerequisite:** `unit.rust.modules.v1`
- **Content IDs:** `unit.rust.structs.v1`; `exercise.rust.structs.build-profile.v1`

### Learning outcomes

- Define a struct with named fields and choose appropriate field types.
- Initialize structs and access or update their fields.
- Add and call methods that operate on struct instances.
- Distinguish public and private struct fields when designing a small API.

### Lesson and exercise scope

Introduce named-field structs and struct literals, then demonstrate field access, mutable updates, and methods. Explain how field visibility affects callers while keeping tuple structs, unit-like structs, and advanced trait implementations out of scope.

The exercise should build a small profile struct, expose a method that derives or formats profile information, and keep at least one implementation detail private. Tests should verify construction, field access through the supported API, method behavior, and a boundary case such as an empty profile field.

### Completion checks

- The learner can define, initialize, and access a struct without confusing fields with local variables.
- The exercise demonstrates methods and a meaningful public/private field boundary.
- The expected solution passes its tests, including the boundary case, and the starter provides a clear compiling scaffold.

## 5. Collections (Implemented)

- **Slug:** `collections`
- **Prerequisite:** `unit.rust.structs.v1`
- **Content IDs:** `unit.rust.collections.v1`; `exercise.rust.collections.record-index.v1`

### Learning outcomes

- Distinguish owned `String` values from string slices and choose between them for common tasks.
- Create, update, and read values from a `Vec`.
- Store and look up keyed values with a `HashMap`.
- Choose an appropriate collection for a small data-management task.

### Lesson and exercise scope

Introduce `String` and `Vec` as owned, growable collections, then demonstrate the basic insert, access, and iteration operations for vectors and maps. Explain lookup behavior for missing map keys and keep collection internals, advanced iterator patterns, and performance analysis out of scope.

The exercise should build and query a small collection of records, using `Vec` to hold the records and `HashMap` to index them, with strings for record data. Tests should cover ordinary insert/query behavior, multiple records, and a missing key or empty collection.

### Completion checks

- The learner can explain when to use `String`, `Vec`, and `HashMap` in the exercise.
- The exercise builds and queries more than one record and tests a missing or empty case.
- The starter compiles, and the expected solution passes the exercise tests.

## 6. Error Handling (Implemented)

- **Slug:** `error-handling`
- **Prerequisite:** `unit.rust.collections.v1`
- **Content IDs:** `unit.rust.error-handling.v1`; `exercise.rust.error-handling.parse-record.v1`

### Learning outcomes

- Represent an optional value with `Option` and handle both `Some` and `None`.
- Represent success or failure with `Result` and provide useful error information.
- Propagate compatible errors with the `?` operator.
- Distinguish recoverable errors from situations that should cause a panic.

### Lesson and exercise scope

Introduce `Option` and `Result` by examining operations that may have no value or may fail, then demonstrate matching, returning an error, and propagating it with `?`. Keep custom error types and broader error-handling libraries out of scope.

The exercise should parse a small record from input that may be absent or invalid, returning a `Result` and using `?` to propagate a parsing failure. Tests should cover valid input, missing input, and malformed input without panicking.

### Completion checks

- The learner can choose between `Option` and `Result` based on whether there is an error to report.
- The exercise handles missing and invalid input as explicit outcomes and demonstrates `?`.
- The starter compiles, and the expected solution passes all success and failure cases.

## 7. Generics and Traits (Implemented)

- **Slug:** `generics-traits`
- **Prerequisite:** `unit.rust.error-handling.v1`
- **Content IDs:** `unit.rust.generics-traits.v1`; `exercise.rust.generics-traits.generic-largest.v1`

### Learning outcomes

- Write a generic function or type that works with more than one concrete type.
- Define and implement a trait to describe shared behavior.
- Use a trait bound to require behavior from a generic parameter.
- Recognize how generics and traits enable reuse while preserving compile-time checks.

### Lesson and exercise scope

Introduce a generic function and a simple generic type, then define a trait, implement it for multiple types, and use a trait bound in a reusable function. Keep lifetimes, associated types, blanket implementations, and advanced trait features out of scope.

The exercise should define a small trait, implement it for multiple numeric types, and use its bound in a generic function. Tests should verify the same operation for at least two distinct types and include a simple boundary case.

### Completion checks

- The learner can identify the generic parameter and explain what behavior the trait bound requires.
- The exercise demonstrates one shared operation with at least two type implementations.
- The starter compiles, and the expected solution passes all type-specific tests.

## 8. Closures and Iterators (Implemented)

- **Slug:** `closures-iterators`
- **Prerequisite:** `unit.rust.generics-traits.v1`
- **Content IDs:** `unit.rust.closures-iterators.v1`; `exercise.rust.closures-iterators.filter-transform.v1`

### Learning outcomes

- Recognize closure syntax and use a closure where a function-like value is expected.
- Create an iterator from a collection and apply common transformations.
- Filter and transform values with iterator adapters, then collect or aggregate the result.
- Explain that iterator adapters are lazy until consumed.

### Lesson and exercise scope

Introduce closure parameters and return expressions, then demonstrate `iter`, `filter`, `map`, and a consuming operation such as `collect` or `sum`. Keep custom iterator implementations, complex closure capture, and advanced iterator combinators out of scope.

The exercise should filter a collection of records or values and transform the matching items into a result collection or aggregate. Tests should cover matching and non-matching values, an empty input, and preservation of the expected order where relevant.

### Completion checks

- The learner can explain which operations filter values, transform values, and consume an iterator.
- The exercise demonstrates a closure-based filter and transformation over a collection.
- The starter compiles, and the expected solution passes ordinary and empty-input tests.

## Integration and release of the sequence

For each unit, verify that it appears after its prerequisite in the Foundations roadmap, loads through the existing unit/exercise flow, and can be submitted and reflected in learner progress. Add the exercise ID to the executor template mapping and cover submission/progress behavior with the existing API integration-test patterns. Check the full roadmap ordering and prerequisite graph, then run `npm run content:validate` and `npm run validate`; complete the MVP smoke-flow checks when the test database is available.

## Next Rust Book topics

- Lifetimes and borrowing annotations.
- Automated testing with unit, integration, and documentation tests.
- Smart pointers, including `Box`, `Rc`, and `RefCell`.
- Concurrency with threads, message passing, and shared state.
