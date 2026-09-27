# Implementation Tracker

## Completed in this continuation slice

- [x] Added task tracking doc for ongoing plan execution.
- [x] Implemented learning units for **Variables** and **Functions**.
- [x] Added runnable exercises for Variables and Functions.
- [x] Updated web unit/exercise pages to load by unit slug.
- [x] Updated API unit endpoint to load unit/exercise content by slug.
- [x] Extended executor exercise mapping for Variables and Functions.
- [x] Expanded content validation across all roadmap-referenced units/exercises.
- [x] Added learning units and exercises for Control Flow, Borrowing, References, and Slices.
- [x] Added a foundations roadmap API endpoint and switched the roadmap page to server-driven content.
- [x] Expanded progress APIs/views to show per-unit status and resume the next incomplete unit.
- [x] Added multi-unit API integration coverage for submissions and progress updates.
- [x] Extended content validation for missing roadmap-linked references and prerequisite cycles.

## Next planned tasks

- [ ] **MVP completion pass:** verify the full login → roadmap → unit → exercise → result → progress flow against the checklist in `docs/mvp-done-checklist.md`.
- [ ] **Progress lifecycle polish:** record `STARTED` progress when a submission is queued/running so dashboard state changes immediately.
- [ ] **Result UX follow-up:** link unit and progress views directly to each unit's latest attempt/result details.
- [ ] **Auth coverage:** add integration tests that exercise roadmap/progress endpoints with authenticated learner roles alongside the existing submission tests.
- [ ] **Next curriculum slice:** add the next Rust fundamentals after Slices (Types, Pattern Matching, and Modules) once the MVP loop is fully checked off.
