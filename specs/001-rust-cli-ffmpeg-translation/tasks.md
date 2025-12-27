---

description: "Task list for Media Conversion CLI Tool implementation"
---

# Tasks: Media Conversion CLI Tool

**Input**: Design documents from `/specs/001-rust-cli-ffmpeg-translation/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: The examples below include test tasks. Tests are OPTIONAL - only include them if explicitly requested in the feature specification.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- **Single project**: `src/`, `tests/` at repository root
- **Web app**: `backend/src/`, `frontend/src/`
- **Mobile**: `api/src/`, `ios/src/` or `android/src/`
- Paths shown below assume single project - adjust based on plan.md structure

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create Rust project structure with cargo new ff
- [x] T002 [P] Add clap, regex, subprocess dependencies to Cargo.toml
- [x] T003 [P] Configure Rust formatting and linting tools (rustfmt, clippy)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 [P] Create src/main.rs with basic CLI entry point using clap
- [x] T005 [P] Create src/grammar/mod.rs module structure
- [x] T006 [P] Create src/intent/mod.rs module structure
- [x] T007 [P] Create src/command_builder/mod.rs module structure
- [x] T008 [P] Create src/executor/mod.rs module structure
- [x] T009 [P] Create src/utils/mod.rs module structure
- [x] T010 [P] Create tests/ directory structure for grammar, command_builder, integration, snapshots

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Interactive Mode Translation (Priority: P1) 🎯 MVP

**Goal**: Implement interactive mode that allows users to enter plain English commands to convert media files

**Independent Test**: Can be fully tested by launching the interactive mode, entering a plain English command, and verifying it translates to the correct conversion command and executes properly.

### Implementation for User Story 1

- [x] T011 [P] [US1] Create src/grammar/tokenizer.rs to tokenize English commands
- [x] T012 [P] [US1] Create src/grammar/parser.rs to parse tokens into intent structs with manual implementation, no heuristics (constitution requirement)
- [x] T013 [P] [US1] Create src/intent/types.rs with Intent struct and OperationType enum
- [x] T014 [US1] Implement basic interactive mode in src/main.rs (depends on T011, T012, T013)
- [x] T015 [US1] Create src/command_builder/builder.rs to convert intents to ffmpeg commands
- [x] T016 [US1] Create src/executor/runner.rs to execute ffmpeg commands using std::process::Command (constitution requirement)
- [x] T017 [US1] Implement error handling with detailed messages and specific guidance on how to fix issues (depends on constitution clarification)

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Direct Command Translation (Priority: P1)

**Goal**: Implement direct command translation from command line with dry-run support

**Independent Test**: Can be fully tested by running the tool with a plain English command and verifying it translates to the correct conversion command and executes it.

### Implementation for User Story 2

- [x] T018 [US2] Enhance clap configuration in src/main.rs to support direct mode and options
- [x] T019 [US2] Add --dry-run flag functionality as execution control to print command without executing (depends on T015, T016)
- [x] T020 [US2] Add --output flag to specify output directory (depends on T013, T015)
- [x] T021 [US2] Implement command validation and error handling for direct mode (depends on T017)

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Predictable Output Naming (Priority: P2)

**Goal**: Ensure output files are named predictably based on input files

**Independent Test**: Can be tested by running conversion commands and verifying the output file naming follows predictable patterns.

### Implementation for User Story 3

- [x] T022 [P] [US3] Create src/utils/file_utils.rs for file path manipulation
- [x] T023 [US3] Implement predictable output naming logic in src/utils/file_utils.rs (depends on T022)
- [x] T024 [US3] Integrate output naming logic with command builder (depends on T015, T023)
- [x] T025 [US3] Update executor to respect output directory specification (depends on T016, T020)

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T026 [P] Add comprehensive unit tests for grammar parsing in tests/grammar/
- [x] T034 [P] Add deterministic behavior tests to verify identical input produces identical output (constitution requirement)
- [x] T027 [P] Add snapshot tests for command generation in tests/snapshots/ (constitution requirement)
- [x] T035 [P] Add error handling for input file existence/readability in src/utils/file_utils.rs
- [x] T036 [P] Add error handling for unsupported file formats in src/grammar/parser.rs
- [x] T037 [P] Add error handling for ffmpeg availability in src/executor/runner.rs
- [x] T038 [P] Add error handling for insufficient disk space in src/executor/runner.rs
- [x] T028 Add integration tests for complete workflows in tests/integration/
- [x] T029 [P] Add documentation updates in README.md
- [x] T030 [P] Add help and version functionality to CLI
- [x] T031 Add proper exit codes (0, 1, 2, >2) as specified in contracts
- [x] T032 [P] Add support for common media formats (MP4, AVI, MOV, etc.) as specified in clarifications
- [x] T039 Add cross-platform build configuration for Linux, macOS, Windows (constitution requirement)
- [x] T033 Run validation against quickstart guide scenarios
- [x] T040 Update Cargo.toml to use only lightweight dependencies as required by constitution

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all models for User Story 1 together:
T011 [US1] Create src/grammar/tokenizer.rs to tokenize English commands
T012 [US1] Create src/grammar/parser.rs to parse tokens into intent structs
T013 [US1] Create src/intent/types.rs with Intent struct and OperationType enum
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence