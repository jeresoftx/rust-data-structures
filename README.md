# Rust Data Structures

Repositorio del camino troncal de Jeresoft Academy para estudiar estructuras de
datos en Rust. Pertenece al Semestre 1 del plan de estudios junto con
`rust-algorithms` (RFC-0001 §10).

El objetivo no es solo implementar estructuras que funcionen. El objetivo es
crear un recurso educativo completo: cada estructura debe explicar por que
existe, que problema resuelve, que alternativas tiene, como se implementa, como
se prueba y como se mide.

## Que Contiene

- Capitulos en Markdown compatibles con mdBook.
- Implementaciones Rust idiomaticas, una estructura por modulo.
- Ejemplos progresivos: basico, intermedio, avanzado y caso real.
- Tests unitarios, tests de integracion y doctests.
- Benchmarks que confrontan el analisis teorico con mediciones.
- Diagramas Mermaid y recursos visuales.
- Ejercicios graduados con soluciones para niveles 1 a 3.

## Lugar En El Camino

Este curso abre el Semestre 1 del camino troncal. Es el curso canonico para la
representacion, invariantes y operaciones de cada estructura de datos. Los
algoritmos que usan esas estructuras viven en `rust-algorithms`, salvo cuando
un algoritmo sea necesario para explicar la propia estructura.

## Capitulos

| # | Capitulo | Modulo | Estado |
|---|----------|--------|--------|
| 01 | Vector | `src/vector.rs` | benchmarked |
| 02 | Linked List | `src/linked_list.rs` | benchmarked |
| 03 | Stack | `src/stack.rs` | benchmarked |
| 04 | Queue | `src/queue.rs` | benchmarked |
| 05 | Deque | `src/deque.rs` | benchmarked |
| 06 | Heap | `src/heap.rs` | planned |
| 07 | Trie | `src/trie.rs` | planned |
| 08 | Graph | `src/graph.rs` | planned |
| 09 | B-Tree | `src/btree.rs` | planned |
| 10 | HashMap | `src/hashmap.rs` | planned |
| 11 | Bloom Filter | `src/bloom_filter.rs` | planned |
| 12 | Skip List | `src/skip_list.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

## Estructura Esperada

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
docs/
  SUMMARY.md
  01-vector.md
src/
  lib.rs
  vector.rs
examples/
  soluciones/
tests/
benches/
diagrams/
assets/
```

## Como Usarlo

Ejecutar tests:

```bash
cargo test
```

Formatear:

```bash
cargo fmt
```

Verificacion completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

Ejecutar benchmarks:

```bash
cargo bench
```

## Gobernanza

- `AGENTS.md` es la guia de arranque para humanos e IA en este repositorio.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha limite.
- `docs/superpowers/plans/2026-07-14-rust-data-structures-course.md` contiene
  el checklist de implementacion inicial.
- `LICENSE.md` resume la doble licencia: codigo bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofia

Este repositorio debe poder leerse como un libro de ingenieria. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningun capitulo se
considera publicable hasta cumplir la anatomia completa de RFC-0001 §14.
