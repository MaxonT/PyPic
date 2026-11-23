# I²C Scanner, terminal y herramientas de hardware

I²C Bus Explorer: escaneo de direcciones de 7 bits 0x08-0x77 mediante ACK; velocidad configurable y selección MSSP/software; resultados con dirección, timestamp, posibles chips (sin afirmar identificación inequívoca). Modos: firmware temporal ejecutado en PIC y salida por UART, o adaptador USB-I²C desde PC. Firmware de escaneo requiere canal de comunicación disponible; avisar sobre interferencia con firmware existente, cambios de pines y necesidad de reprogramar. Registro I²C: lectura/escritura explícita con advertencia de efectos laterales, manejo de NACK, timeout, bus ocupado y clock stretching.

Serial Monitor: selección de puerto COM/tty, baud, bits de datos, paridad, stop, flow control, encoding ASCII/UTF-8/HEX, timestamps, logs, filtros, envío ASCII/HEX/archivos, reconexión y plotter numérico. No presuponer UART nativo en todos los PIC; USB-UART o UART software cuando aplique. Simulador UART virtual separado de puerto físico.

Herramientas adicionales: SPI Inspector, EEPROM Explorer, registro de periféricos, pin conflict resolver, Clock Configurator, PWM Configurator, analizador de protocolos, comparación de firmware HEX, reportes de configuración y memoria. Para PWM calcular PR2/prescaler/CCP y error real de frecuencia según MCU/reloj; UI no debe prometer frecuencia exacta si no alcanzable.
