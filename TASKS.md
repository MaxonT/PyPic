# Backlog de implementación

## Fase 0 — Diseño
- [ ] Especificar gramática del subconjunto Python y sistema de tipos
- [ ] Definir IR y ABI PIC14
- [ ] Definir schema y validación Device Pack
- [ ] Elegir licencia, estructura monorepo y políticas de contribución
## Fase 1 — Compilador nativo
- [ ] Parser y errores con ubicación
- [ ] Tipado y constantes
- [ ] Instrucciones PIC14 y encoder
- [ ] Bancos, saltos, llamadas y linker
- [ ] Intel HEX y config words
- [ ] Blink real PIC16F887 + pruebas
## Fase 2 — Hardware-aware
- [ ] Definición PIC16F887 validada
- [ ] Diagnósticos LSP y pinout
- [ ] SDK GPIO, timers, ADC, PWM, I²C, UART
## Fase 3 — Herramientas
- [ ] VS Code y Project Wizard
- [ ] Simulador y DAP
- [ ] Terminal serial y plotter
- [ ] I²C Scanner y explorador de registros
## Fase 4 — Extensibilidad
- [ ] PIC12F625
- [ ] Backend ASM PIC-AS y C XC8/SDCC
- [ ] Adaptadores de toolchains y programadores
- [ ] Blockly y ecosistema de paquetes

## Criterio transversal de arquitectura
- [ ] Confirmar que cada backend produce firmware nativo autónomo, sin intérprete/VM Python, REPL ni bytecode Python embebido.
- [ ] Mantener la separación host (parseo/IR/transpilación) vs dispositivo (ejecución de instrucciones nativas).
