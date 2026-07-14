# Tests De Integracion

Los tests de integracion validan el comportamiento publico de cada estructura.
Los detalles internos se cubren con tests unitarios dentro de `src/`.

Convencion de nombres:

```text
<modulo>_test.rs
```

Cada test debe corresponder a un caso descrito en el capitulo: si la teoria
identifica un borde, el repo debe tener una prueba para ese borde.
