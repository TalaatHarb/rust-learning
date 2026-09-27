# Implementation Tracker

## Completed in this continuation slice

- [x] Added task tracking doc for ongoing plan execution.
- [x] Implemented learning units for **Variables** and **Functions**.
- [x] Added runnable exercises for Variables and Functions.
- [x] Updated web unit/exercise pages to load by unit slug.
- [x] Updated API unit endpoint to load unit/exercise content by slug.
- [x] Extended executor exercise mapping for Variables and Functions.
- [x] Expanded content validation across all roadmap-referenced units/exercises.

## Next planned tasks

- [ ] **Curriculum expansion:** add the next four fundamentals units and exercises — Control Flow, Borrowing, References, and Slices — so the current foundations roadmap goes beyond Variables, Functions, and Ownership.
- [ ] **Roadmap API slice:** add a roadmap/content endpoint that returns module and unit metadata from `content/roadmaps/foundations.json` for the web app instead of maintaining planned units inline in the UI.
- [ ] **Progress tracking follow-up:** expand dashboard and progress views to show per-unit status across all implemented units, plus resume-learning behavior based on the next incomplete unit.
- [ ] **Integration coverage:** add end-to-end API/executor tests that submit solutions for multiple units and verify attempt persistence plus progress transitions.
- [ ] **Content graph validation:** extend `npm run content:validate` to detect missing references and prerequisite dependency cycles across all roadmap-linked units before adding more curriculum content.
