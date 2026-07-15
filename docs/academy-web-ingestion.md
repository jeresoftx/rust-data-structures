# Academy Web Ingestion

> **Estado:** preparado, mecanismo pendiente.
> **Repositorio destino:** `academy-web`.
> **Fuente canonica:** este repositorio, no una copia editada en el sitio.

Este documento prepara la ingestion del curso `rust-data-structures` hacia
`academy-web` sin decidir todavia el mecanismo tecnico. La decision final puede
ser submodule, subtree, fetch desde GitHub, paquete versionado, generacion en CI
o una tuberia propia del sitio. Este repo solo fija el contrato que cualquier
mecanismo debe respetar.

## Principio Rector

El curso vive aqui. `academy-web` puede renderizar, ordenar, buscar, enlazar y
enriquecer la experiencia, pero no debe convertirse en la fuente de verdad del
contenido educativo ni del codigo Rust.

Eso protege tres invariantes de RFC-0001:

- El curso se lee como libro de ingenieria.
- El codigo, tests, ejemplos, benches y diagramas evolucionan juntos.
- Nada se publica como completo si el repo fuente no lo sostiene.

## Inventario Canonico

La ingestion debe partir de:

- [`docs/course-manifest.json`](./course-manifest.json): inventario estable para
  automatizacion.
- [`docs/SUMMARY.md`](./SUMMARY.md): orden humano de capitulos.
- `docs/NN-title.md`: capitulo canonico compatible con mdBook.
- `src/<module>.rs`: implementacion y documentacion publica.
- `examples/<module>_*.rs`: ejemplos progresivos.
- `examples/soluciones/*.rs`: soluciones de ejercicios nivel 1 a 3.
- `tests/<module>_test.rs`: pruebas de integracion.
- `benches/<module>_bench.rs`: mediciones de complejidad.
- `diagrams/NN-title.mmd`: diagrama Mermaid del capitulo.

Si dos fuentes parecen contradecirse, gana este orden:

1. `docs/course-manifest.json` para rutas e IDs consumibles por maquina.
2. `docs/SUMMARY.md` para el orden editorial.
3. `README.md` y `ROADMAP.md` para estado publico del repositorio.
4. El checklist en `docs/superpowers/plans/` para trazabilidad historica.

## Contrato De Capitulo

Cada capitulo expone estos campos minimos para el sitio:

| Campo | Fuente | Regla |
|---|---|---|
| `courseId` | manifiesto | Siempre `rust-data-structures`. |
| `number` | manifiesto y nombre de archivo | Entero estable de 1 a 12. |
| `slug` | manifiesto | Formato `NN-title`, estable para URL. |
| `title` | manifiesto y H1 | Nombre visible del capitulo. |
| `chapterPath` | manifiesto | Markdown canonico. |
| `modulePath` | manifiesto | Codigo Rust asociado. |
| `diagramPath` | manifiesto | Mermaid asociado. |
| `examples` | manifiesto | Basico, intermedio, avanzado y caso real. |
| `solutions` | manifiesto | Solo niveles 1 a 3. |
| `status` | manifiesto y README | Estado editorial actual. |

El sitio puede derivar metadatos adicionales del bloque inicial del Markdown
(`Curso`, `Capitulo`, `Prerequisitos`, `Codigo`, `Video`, `Leccion en el
sitio`), pero no debe depender solo de texto libre para construir rutas.

## Reglas De Transformacion

- Mantener los slugs del manifiesto como URLs canonicas.
- Renderizar Mermaid desde `diagrams/`, no desde una copia pegada a mano.
- Enlazar codigo, ejemplos, tests y benches como artefactos del mismo capitulo.
- Mantener los ejercicios de nivel 4 como discusion, no como solucion cerrada.
- Marcar video y leccion de sitio como `pendiente` hasta que existan artefactos
  reales.
- No publicar automaticamente capitulos por estar en el manifiesto; el estado
  `published` requiere decision editorial humana.
- No reescribir referencias a cursos futuros como si esos cursos ya existieran.
  Deben seguir siendo notas de "mas adelante en el camino".

## Decisiones Diferidas

Estas decisiones pertenecen a `academy-web` o a una RFC futura:

- Mecanismo fisico de ingestion.
- Formato final de rutas publicas del sitio.
- Motor de busqueda e indexacion.
- Soporte de playground Rust en navegador.
- Render de benchmarks como tablas, graficas o anexos.
- Politica de versionado visible para lectores.
- Pipeline de publicacion y revision humana.

## Checklist Para Activar La Ingestion

- [ ] Elegir el mecanismo de contenido en `academy-web`.
- [ ] Leer `docs/course-manifest.json` desde el sitio sin rutas hardcodeadas.
- [ ] Renderizar un capitulo piloto con Markdown, Mermaid, codigo y ejemplos.
- [ ] Validar enlaces a codigo, tests, benches y soluciones.
- [ ] Marcar en el sitio que video y leccion estan pendientes cuando aplique.
- [ ] Ejecutar revision humana antes de declarar cualquier capitulo `published`.
- [ ] Registrar cualquier cambio de contrato en este documento y en el
      manifiesto.

## Criterio De Preparacion

Este repo esta preparado para ingestion cuando:

- El manifiesto JSON es valido.
- Todas las rutas declaradas en el manifiesto existen.
- `cargo fmt --check` pasa.
- `cargo clippy --all-targets --all-features -- -D warnings` pasa.
- `cargo test --all-targets` pasa.
- `cargo test --doc` pasa.

La ingestion puede esperar. Lo importante es que, cuando `academy-web` decida su
mecanismo, no tenga que adivinar que representa cada archivo del curso.
