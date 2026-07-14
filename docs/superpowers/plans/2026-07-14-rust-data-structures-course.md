# Rust Data Structures Course Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `rust-data-structures` as the complete Semestre 1 data structures course for Jeresoft Academy, aligned with RFC-0001.

**Architecture:** One Rust crate, one public module per data structure, one `docs/NN-*.md` chapter per module, and parallel `examples/`, `tests/`, `benches/`, `diagrams/`, and `assets/` material. The course is built from fundamental to specialized structures: `vector`, `linked_list`, `stack`, `queue`, `deque`, `heap`, `trie`, `graph`, `btree`, `hashmap`, `bloom_filter`, `skip_list`.

**Tech Stack:** Rust 2021 or newer, Cargo, `cargo fmt`, `cargo clippy`, unit tests, integration tests, doctests, Cargo benches, Markdown compatible with mdBook, Mermaid diagrams.

---

## Source Decisions

- [x] Re-read RFC-0001 sections before execution: `§1`, `§2`, `§10`, `§12`, `§13`, `§14`, `§15`, `§16`, `§17`, `§20`, `§21`.
- [x] Treat `rust-data-structures` as a course repository, not a generic crate.
- [x] Keep the course sequence fixed unless a future RFC changes it: vector, linked list, stack, queue, deque, heap, trie, graph, btree, hashmap, bloom filter, skip list.
- [x] Keep structure representation and operations canonical here; algorithms over those structures belong in `rust-algorithms` unless they are necessary to explain the structure.
- [x] Avoid creating content for future courses inside this repo.

## Repository Foundation

### Task 1: Establish Repository Identity

**Files:**
- Create: `README.md`
- Create: `AGENTS.md`
- Create: `ROADMAP.md`
- Create: `LICENSE-MIT`
- Create: `LICENSE-APACHE`
- Create: `LICENSE-CC-BY-SA-4.0.md`

- [x] Create `README.md` with the course purpose, its place in Semestre 1, how to navigate `docs/`, `src/`, `examples/`, `tests/`, `benches/`, and `diagrams/`.
- [x] State in `README.md` that this repo teaches data structures in Rust as part of Jeresoft Academy RFC-0001.
- [x] Add the planned chapter table in `README.md` with status columns: `planned`, `draft`, `implemented`, `tested`, `benchmarked`, `reviewed`, `published`.
- [x] Create `AGENTS.md` from RFC-0001 §20, instantiated for `colección = camino troncal / Semestre 1` and `tema = estructuras de datos en Rust`.
- [x] Create `ROADMAP.md` with the twelve chapters and the no-deadlines project philosophy from RFC-0001 §1.
- [x] Add dual licensing: MIT OR Apache-2.0 for code, CC BY-SA 4.0 for educational content.
- [x] Run `git status --short` and confirm only intentional foundation files are pending.
- [x] Commit: `chore: establish data structures repository foundation`.

### Task 2: Create Cargo Crate Skeleton

**Files:**
- Create: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `src/vector.rs`
- Create: `src/linked_list.rs`
- Create: `src/stack.rs`
- Create: `src/queue.rs`
- Create: `src/deque.rs`
- Create: `src/heap.rs`
- Create: `src/trie.rs`
- Create: `src/graph.rs`
- Create: `src/btree.rs`
- Create: `src/hashmap.rs`
- Create: `src/bloom_filter.rs`
- Create: `src/skip_list.rs`

- [x] Create `Cargo.toml` with package name `rust-data-structures`, edition, license expression `MIT OR Apache-2.0`, repository URL, and no unnecessary dependencies.
- [x] Create `src/lib.rs` with crate-level documentation explaining the course and public `pub mod` declarations for all twelve structures.
- [x] Create one empty module file per structure with a module-level doc-comment describing the learning goal.
- [x] Run `cargo fmt`.
- [x] Run `cargo check`.
- [x] Run `cargo test`.
- [x] Commit: `chore: scaffold data structures crate`.

### Task 3: Create Course Directory Layout

**Files and directories:**
- Create: `docs/`
- Create: `examples/`
- Create: `examples/soluciones/`
- Create: `tests/`
- Create: `benches/`
- Create: `diagrams/`
- Create: `assets/`

- [x] Create the standard RFC-0001 §15 directories.
- [x] Add `docs/SUMMARY.md` listing the twelve chapters in order.
- [x] Add `diagrams/README.md` explaining Mermaid-first diagram policy.
- [x] Add `examples/README.md` explaining basic, intermediate, advanced, and real-case examples.
- [x] Add `tests/README.md` explaining integration-test naming by structure.
- [x] Add `benches/README.md` explaining that benchmarks validate complexity claims, not vanity performance.
- [x] Commit: `chore: add course content directories`.

### Task 4: Add CI and Quality Gates

**Files:**
- Create: `.github/workflows/ci.yml`
- Create: `.github/workflows/docs.yml`

- [x] Add CI jobs for `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, `cargo test --all-targets`, and `cargo test --doc`.
- [x] Add a docs workflow that validates Markdown links and builds mdBook-compatible docs once the docs tool is chosen for this repo.
- [x] Keep the initial workflow dependency-free unless a tool is justified in `README.md`.
- [x] Commit: `ci: add rust quality gates`.

## Chapter Production Pipeline

For each chapter, complete the following checklist before moving to the next structure.

- [ ] Create `docs/NN-title.md` using RFC-0001 §16 exactly.
- [ ] Fill metadata: course, chapter number, prerequisites, code path, video status, site lesson status.
- [ ] Write `Introducción`: what the reader learns and what they already need.
- [ ] Write `Motivación`: real problem first, formalism second.
- [ ] Write `Teoría / Historia`: origin, historical problem, and why the structure exists.
- [ ] Write `Teoría / Fundamentos`: representation, invariants, operations, and mental model.
- [ ] Write `Teoría / Casos de uso`: real systems where the structure is useful.
- [ ] Write `Teoría / Ventajas y limitaciones`: honest tradeoffs.
- [ ] Write `Teoría / Comparación con alternativas`: when not to use it.
- [ ] Add at least one Mermaid diagram in `diagrams/`.
- [ ] Write complexity table and reasoning for every public operation.
- [ ] State whether interactive visualization applies; if not, justify in one line.
- [ ] Implement the public API in `src/<module>.rs`.
- [ ] Add doc-comments with examples and complexity notes for all public items.
- [ ] Add unit tests inside the module for invariants and edge cases.
- [ ] Add integration tests in `tests/<module>_test.rs`.
- [ ] Add doctests through examples in public documentation.
- [ ] Add benchmarks in `benches/<module>_bench.rs` when the structure has meaningful complexity claims to measure.
- [ ] Add examples in `examples/<module>_basic.rs`, `examples/<module>_intermediate.rs`, `examples/<module>_advanced.rs`, and `examples/<module>_real_case.rs` when applicable.
- [ ] Add four to eight exercises in the chapter, spanning levels 1 to 4.
- [ ] Add solutions for levels 1 to 3 in `examples/soluciones/`.
- [ ] Add a level 4 discussion of approaches and tradeoffs.
- [ ] Add references: primary source where possible, canonical book, Rust documentation when relevant.
- [ ] Run `cargo fmt`.
- [ ] Run `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] Run `cargo test --all-targets`.
- [ ] Run doc build or Markdown validation.
- [ ] Update `README.md` and `ROADMAP.md` status for the chapter.
- [ ] Commit the chapter with `feat: add <structure> chapter`.

## Course Chapters

### Task 5: Vector

**Files:**
- Create: `docs/01-vector.md`
- Modify: `src/vector.rs`
- Create: `tests/vector_test.rs`
- Create: `benches/vector_bench.rs`
- Create: `diagrams/01-vector.mmd`
- Create: `examples/vector_basic.rs`
- Create: `examples/vector_intermediate.rs`
- Create: `examples/vector_advanced.rs`
- Create: `examples/vector_real_case.rs`

- [x] Teach contiguous memory, capacity, length, growth, indexing, push, pop, insert, remove, iteration, and amortized complexity.
- [x] Explain why vector comes first: it anchors ownership, allocation, cache locality, and the rest of the course.
- [x] Compare against arrays, linked lists, deques, and slices.
- [x] Include tests for empty vector, growth, indexing bounds, insertion at beginning/middle/end, removal, iteration, and drop behavior where observable.
- [x] Include benchmarks for push growth, random access, insertion at front, and insertion at end.

### Task 6: Linked List

**Files:**
- Create: `docs/02-linked-list.md`
- Modify: `src/linked_list.rs`
- Create: `tests/linked_list_test.rs`
- Create: `benches/linked_list_bench.rs`
- Create: `diagrams/02-linked-list.mmd`
- Create: `examples/linked_list_basic.rs`
- Create: `examples/linked_list_intermediate.rs`
- Create: `examples/linked_list_advanced.rs`
- Create: `examples/linked_list_real_case.rs`

- [x] Teach nodes, ownership, pointers, head/tail, singly vs doubly linked lists, traversal, insertion, removal, and iterator design.
- [x] Prefer safe Rust. Any `unsafe` requires a `// SAFETY:` invariant and a written justification in the chapter.
- [x] Compare against vector, deque, and intrusive lists.
- [x] Include tests for empty list, push front/back, pop front/back, remove, iteration, and single-element transitions.
- [x] Include benchmarks that show where linked lists lose to vectors because of locality.

### Task 7: Stack

**Files:**
- Create: `docs/03-stack.md`
- Modify: `src/stack.rs`
- Create: `tests/stack_test.rs`
- Create: `benches/stack_bench.rs`
- Create: `diagrams/03-stack.mmd`
- Create: `examples/stack_basic.rs`
- Create: `examples/stack_intermediate.rs`
- Create: `examples/stack_advanced.rs`
- Create: `examples/stack_real_case.rs`

- [x] Teach LIFO, push, pop, peek, underflow, implementation over vector, and why stack is an interface more than a storage strategy.
- [x] Compare vector-backed stack against linked-list-backed stack.
- [x] Include real cases: call stack, undo history, expression parsing.
- [x] Include tests for empty pop, peek, LIFO order, clear, and mixed operations.
- [x] Include benchmarks for vector-backed operations and explain why most are O(1).

### Task 8: Queue

**Files:**
- Create: `docs/04-queue.md`
- Modify: `src/queue.rs`
- Create: `tests/queue_test.rs`
- Create: `benches/queue_bench.rs`
- Create: `diagrams/04-queue.mmd`
- Create: `examples/queue_basic.rs`
- Create: `examples/queue_intermediate.rs`
- Create: `examples/queue_advanced.rs`
- Create: `examples/queue_real_case.rs`

- [x] Teach FIFO, enqueue, dequeue, front, back, circular buffers, and why naive vector removal is costly.
- [x] Compare linked-list queue, circular-buffer queue, and standard `VecDeque`.
- [x] Include real cases: schedulers, BFS frontier, request processing.
- [x] Include tests for empty dequeue, FIFO order, wraparound behavior, growth, and mixed operations.
- [x] Include benchmarks contrasting naive front removal and circular-buffer behavior.

### Task 9: Deque

**Files:**
- Create: `docs/05-deque.md`
- Modify: `src/deque.rs`
- Create: `tests/deque_test.rs`
- Create: `benches/deque_bench.rs`
- Create: `diagrams/05-deque.mmd`
- Create: `examples/deque_basic.rs`
- Create: `examples/deque_intermediate.rs`
- Create: `examples/deque_advanced.rs`
- Create: `examples/deque_real_case.rs`

- [x] Teach double-ended queues, ring buffers, push/pop front/back, indexing tradeoffs, and growth.
- [x] Compare against stack, queue, vector, and linked list.
- [x] Include real cases: sliding-window algorithms, work queues, browser navigation.
- [x] Include tests for all four end operations, wraparound, growth, and order preservation.
- [x] Include benchmarks for front/back operations and indexed access where supported.

### Task 10: Heap

**Files:**
- Create: `docs/06-heap.md`
- Modify: `src/heap.rs`
- Create: `tests/heap_test.rs`
- Create: `benches/heap_bench.rs`
- Create: `diagrams/06-heap.mmd`
- Create: `examples/heap_basic.rs`
- Create: `examples/heap_intermediate.rs`
- Create: `examples/heap_advanced.rs`
- Create: `examples/heap_real_case.rs`

- [x] Teach binary heap representation, heap property, sift up, sift down, push, pop, peek, heapify, min-heap vs max-heap.
- [x] Compare heap against sorted vector, balanced tree, and priority queue APIs.
- [x] Include real cases: priority scheduling, Dijkstra frontier, top-k queries.
- [x] Include tests for ordering, duplicates, heapify, custom comparators if supported, and empty pop.
- [x] Include benchmarks for push/pop and heapify.

### Task 11: Trie

**Files:**
- Create: `docs/07-trie.md`
- Modify: `src/trie.rs`
- Create: `tests/trie_test.rs`
- Create: `benches/trie_bench.rs`
- Create: `diagrams/07-trie.mmd`
- Create: `examples/trie_basic.rs`
- Create: `examples/trie_intermediate.rs`
- Create: `examples/trie_advanced.rs`
- Create: `examples/trie_real_case.rs`

- [x] Teach prefix trees, terminal markers, insertion, lookup, deletion, prefix search, and memory tradeoffs.
- [x] Compare against hashmap, sorted vector, B-tree, radix tree, and finite automata.
- [x] Include real cases: autocomplete, dictionary lookup, routing prefixes.
- [x] Include tests for empty strings, shared prefixes, deletion without breaking siblings, prefix queries, and Unicode policy.
- [x] Include benchmarks for lookup and prefix search.

### Task 12: Graph

**Files:**
- Create: `docs/08-graph.md`
- Modify: `src/graph.rs`
- Create: `tests/graph_test.rs`
- Create: `benches/graph_bench.rs`
- Create: `diagrams/08-graph.mmd`
- Create: `examples/graph_basic.rs`
- Create: `examples/graph_intermediate.rs`
- Create: `examples/graph_advanced.rs`
- Create: `examples/graph_real_case.rs`

- [x] Teach graph representation, directed vs undirected, weighted vs unweighted, adjacency list, adjacency matrix, edge list, and traversal API boundaries.
- [x] Keep algorithmic deep dives in `rust-algorithms`; this chapter owns representation and operations.
- [x] Compare representations by density, memory, and update/query patterns.
- [x] Include real cases: social graph, dependency graph, route map.
- [x] Include tests for node insertion, edge insertion, removal, directed edges, undirected edges, self-loops, and missing nodes.
- [x] Include benchmarks comparing adjacency list and adjacency matrix operations where implemented.

### Task 13: B-Tree

**Files:**
- Create: `docs/09-btree.md`
- Modify: `src/btree.rs`
- Create: `tests/btree_test.rs`
- Create: `benches/btree_bench.rs`
- Create: `diagrams/09-btree.mmd`
- Create: `examples/btree_basic.rs`
- Create: `examples/btree_intermediate.rs`
- Create: `examples/btree_advanced.rs`
- Create: `examples/btree_real_case.rs`

- [ ] Teach multiway search trees, node capacity, sorted keys, splitting, searching, insertion, deletion strategy, and cache/page locality.
- [ ] Explain why B-tree is canonical here and reused later by database internals.
- [ ] Compare against binary search tree, red-black tree, skip list, and hashmap.
- [ ] Include real cases: filesystems, databases, ordered maps.
- [ ] Include tests for insertion, search, split, sorted iteration, duplicate policy, and deletion if implemented.
- [ ] Include benchmarks for search and insertion under ordered and random input.

### Task 14: HashMap

**Files:**
- Create: `docs/10-hashmap.md`
- Modify: `src/hashmap.rs`
- Create: `tests/hashmap_test.rs`
- Create: `benches/hashmap_bench.rs`
- Create: `diagrams/10-hashmap.mmd`
- Create: `examples/hashmap_basic.rs`
- Create: `examples/hashmap_intermediate.rs`
- Create: `examples/hashmap_advanced.rs`
- Create: `examples/hashmap_real_case.rs`

- [ ] Teach hashing, buckets, collisions, load factor, resizing, separate chaining vs open addressing, lookup, insert, remove, and iteration tradeoffs.
- [ ] Compare against B-tree, vector search, trie, and perfect hashing.
- [ ] Include real cases: caches, indexes, deduplication, counting frequencies.
- [ ] Include tests for collision handling, overwrite, remove, resize, missing keys, and iteration consistency.
- [ ] Include benchmarks for inserts/lookups under normal and high-collision scenarios.

### Task 15: Bloom Filter

**Files:**
- Create: `docs/11-bloom-filter.md`
- Modify: `src/bloom_filter.rs`
- Create: `tests/bloom_filter_test.rs`
- Create: `benches/bloom_filter_bench.rs`
- Create: `diagrams/11-bloom-filter.mmd`
- Create: `examples/bloom_filter_basic.rs`
- Create: `examples/bloom_filter_intermediate.rs`
- Create: `examples/bloom_filter_advanced.rs`
- Create: `examples/bloom_filter_real_case.rs`

- [ ] Teach probabilistic membership, bit arrays, multiple hashes, false positives, no false negatives, sizing, and error probability.
- [ ] Compare against hashmap, set, counting Bloom filter, and Cuckoo filter.
- [ ] Include real cases: cache filtering, database reads, web crawlers.
- [ ] Include tests for inserted values, definitely-missing behavior under controlled inputs, false-positive-rate measurement, and parameter validation.
- [ ] Include benchmarks for insert and membership checks.

### Task 16: Skip List

**Files:**
- Create: `docs/12-skip-list.md`
- Modify: `src/skip_list.rs`
- Create: `tests/skip_list_test.rs`
- Create: `benches/skip_list_bench.rs`
- Create: `diagrams/12-skip-list.mmd`
- Create: `examples/skip_list_basic.rs`
- Create: `examples/skip_list_intermediate.rs`
- Create: `examples/skip_list_advanced.rs`
- Create: `examples/skip_list_real_case.rs`

- [ ] Teach probabilistic levels, expected logarithmic search, insertion, deletion, iteration, and randomization.
- [ ] Compare against B-tree, balanced binary trees, sorted vectors, and linked lists.
- [ ] Include real cases: ordered indexes, in-memory stores, concurrent-friendly ordered maps.
- [ ] Include tests for search, insert, delete, ordering, duplicate policy, seeded level generation, and edge cases.
- [ ] Include benchmarks for random inserts, ordered inserts, search, and iteration.

## Cross-Course Integration

- [ ] Add references from vector, stack, queue, deque, heap, trie, graph, and hashmap to future `rust-algorithms` chapters where algorithms use them.
- [ ] Add references from B-tree to future `rust-database-internals`.
- [ ] Add references from graph to future system design and distributed systems where graph modeling appears.
- [ ] Add references from Bloom filter and skip list to future database, cache, and distributed systems material.
- [ ] Keep every cross-course link as a citation or "later in the path" note, not a full re-explanation.

## Final Course Completion

- [ ] Every public item has doc-comments with examples and complexity notes.
- [ ] Every chapter has the eleven required RFC-0001 §14 sections; visualization is either present or explicitly justified as not applicable.
- [ ] Every chapter has four to eight exercises across levels 1 to 4.
- [ ] Every level 1 to 3 exercise has a solution in `examples/soluciones/`.
- [ ] Every level 4 exercise has a tradeoff discussion.
- [ ] Every structure has unit tests, integration tests, doctests, and benchmarks where meaningful.
- [ ] `cargo fmt --check` passes.
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes.
- [ ] `cargo test --all-targets` passes.
- [ ] `cargo test --doc` passes.
- [ ] Documentation builds or validates without broken links.
- [ ] README status table and ROADMAP match the actual repo state.
- [ ] GitHub topics and description match the course identity.
- [ ] The repo is ready for `academy-web` ingestion once the site content mechanism is decided.
- [ ] Final commit: `docs: mark data structures course checklist complete`.

## Execution Options

1. **Subagent-Driven:** dispatch a fresh worker per major task or per chapter, then review each result before continuing.
2. **Inline Execution:** execute the checklist in this session with checkpoints after foundation, CI, and each chapter.
