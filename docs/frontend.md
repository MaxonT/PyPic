# Frontend del compilador (`pypic-frontend`)

**Estado: implementado (primer corte).** Parseo + validacion del subconjunto + tipos + constantes.
No genera IR ni codigo todavia. Escrito en Rust; Python es solo la sintaxis del lenguaje fuente.
Todo corre en el host: no hay interprete ni VM de Python en el MCU
(ver `docs/spec/00_PRINCIPIOS_NO_INTERPRETE.md`).

```
fuente .py -> lexer -> parser (AST propio) -> check (tipos, constantes, reglas) -> Program tipado
```

## Independencia del ancho del MCU

PyPIC no es "solo 8 bits": cubre cada MCU PIC. El lenguaje tiene enteros de ancho fijo
(`u8 i8 u16 i16 u32 i32`) y `bool`; **no hay `int` nativo ni ancho de palabra asumido**.
Que tipos admite un destino lo decide `TargetProfile` (hoy `max_int_bits`; mas adelante lo
llenara el Device Pack). Sin perfil, el destino es `generic` (hasta 32 bits).

## Subconjunto soportado

- Nivel de modulo: `x: u8 = 0`, `K: Final[u8] = 5` (constante), funciones, `from pypic import ...`.
- Funciones: parametros y retorno tipados (`-> None` si no devuelve), `@interrupt` (una sola, sin parametros).
- Sentencias: asignacion, `+= -= *= //= %= &= |= ^= <<= >>=`, `if/elif/else`, `while`,
  `for i in range(...)`, `return`, `break`, `continue`, `pass`, `global`, llamadas.
- Expresiones: enteros (dec/hex/bin/octal), `True/False`, aritmetica, bit a bit, desplazamientos,
  comparaciones, `and/or/not`, llamadas, conversiones `u8(x)`.
- `u8 i8 u16 i16 u32 i32 bool Final interrupt` estan predeclarados; `from pypic import ...` es opcional
  (mantiene el archivo valido para otros editores).

Todo lo demas se rechaza con un diagnostico especifico: clases, listas/tuplas, cadenas, floats, `/`, `**`,
lambdas, `try`, `import` general, atributos, indexado, argumentos con nombre, comparaciones encadenadas, etc.

## Semantica fijada (decisiones a revisar)

| Tema | Regla |
|---|---|
| Literales | Sin tipo propio; toman el del contexto si su valor cabe (`PYP2040` si no). |
| Conversiones | Siempre explicitas: `u16(a) + b`. Mezclar tipos es `PYP2047`. |
| Overflow en ejecucion | Modular (wrap-around) en el ancho del tipo. |
| Overflow constante | Error (`PYP2045`/`PYP2040`), nunca truncado en silencio. |
| Cast explicito | Trunca a proposito: `u8(u16(300)) == 44`. |
| `//` y `%` | Truncan hacia cero (como C), **no** como Python. `/` no existe. |
| Condiciones | Deben ser `bool` (sin truthiness): `if x != 0`. |
| Variables | Siempre inicializadas al declararse; las globales con constante. Declarar con tipo antes de asignar. |
| Globales | Modificarlas en una funcion exige `global x`. Un local no puede ocultar una global. |
| `for` | Solo `range(...)`; el paso debe ser constante distinto de 0; si todo es literal, el tipo es el menor que cabe. |
| Recursion | Prohibida (memoria estatica, pila de hardware limitada): `PYP2013`. |
| Desplazamientos | Cantidad literal `0..bits-1` o entero sin signo; el operando izquierdo necesita tipo. |

## Diagnosticos

Formato: `archivo:linea:col: error[PYPnnnn]: mensaje`. Rangos: `PYP0xxx` lexer, `PYP1xxx` sintaxis/subconjunto,
`PYP2xxx` tipos y semantica. Si hay errores de sintaxis no se ejecuta el verificador de tipos.

## Uso

```
cargo run -p pypic-cli -- check examples/counter.py [--max-int-bits 16]
cargo test
```

## Criterios de aceptacion

- [x] Parser con errores con ubicacion y recuperacion por sentencia (`syntax_errors_report_all_statements`).
- [x] Tipado y constantes (literales, casts, overflow, `//` y `%`).
- [x] Reglas hardware-conscientes: sin recursion, una ISR, tipos limitados por el destino.
- [x] Cada construccion rechazada tiene test con su codigo.
- [ ] Pendiente: resolucion de modulos del SDK (`machine`, `gpio`...), arreglos estaticos, IR tipada,
      validacion contra Device Pack, advertencias (codigo inalcanzable, variables sin usar).
