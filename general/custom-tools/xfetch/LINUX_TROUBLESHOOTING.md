# Linux Troubleshooting Guide

## Errores Comunes y Soluciones

### 1. Error de netstat2 (SOLUCIONADO)

**Error:**
```
error[E0609]: no field `state` on type `&libc::tcp_info`
 --> netstat2-0.9.1/src/integrations/linux/netlink_iterator.rs:193:40
```

**Solución implementada:**
- Movidas las dependencias problemáticas (`netstat2`, `wmi`, `local-ip-address`, etc.) a ser específicas de Windows
- Implementadas funciones nativas para Linux que no dependen de librerías externas
- Uso de comandos del sistema (`ip route`, `/proc/net/dev`, `/sys/class/net/`) para funcionalidad de red

### 2. Errores de Lifetime (SOLUCIONADO)

**Error:**
```
error[E0716]: temporary value dropped while borrowed
 --> src/system_info.rs:649:24
```

**Solución implementada:**
- Creadas variables intermedias para evitar que valores temporales se liberen prematuramente
- Separación de `String::from_utf8_lossy()` y `.trim()` en variables distintas

**Ejemplo de corrección:**
```rust
// Antes (problemático):
let temp = String::from_utf8_lossy(&output.stdout).trim();

// Después (corregido):
let temp_output = String::from_utf8_lossy(&output.stdout);
let temp = temp_output.trim();
```

## Compilación en Linux

### Requisitos
- Rust 1.70+ (recomendado 1.80+)
- Herramientas del sistema: `ip`, `cat`, `nvidia-smi` (opcional para GPU NVIDIA)

### Pasos de instalación

1. **Clonar el repositorio:**
   ```bash
   git clone <repository-url>
   cd xfetch
   ```

2. **Compilar:**
   ```bash
   cargo build --release
   ```

3. **Probar:**
   ```bash
   ./target/release/xfetch -X
   ```

4. **Instalar globalmente (opcional):**
   ```bash
   sudo cp target/release/xfetch /usr/local/bin/
   # O agregar target/release a tu PATH
   ```

### Script de prueba automática

Usa el script incluido para verificar la compilación:
```bash
chmod +x test-linux-build.sh
./test-linux-build.sh
```

## Funcionalidades específicas de Linux

### Información de red
- **Interfaz activa:** Detectada via `ip route show default`
- **Velocidad:** Leída desde `/sys/class/net/{interface}/speed`
- **Estadísticas:** Obtenidas de `/proc/net/dev`

### Información de batería
- **Capacidad:** `/sys/class/power_supply/BAT0/capacity`
- **Estado:** `/sys/class/power_supply/BAT0/status`

### GPU NVIDIA
- **Temperatura:** `nvidia-smi --query-gpu=temperature.gpu`
- **VRAM:** `nvidia-smi --query-gpu=memory.total`

## Limitaciones conocidas

1. **GPU AMD/Intel:** Información limitada en Linux (solo nombre básico)
2. **Información de red:** Requiere permisos de lectura en `/sys/class/net/`
3. **Batería:** Solo funciona con BAT0, algunos sistemas usan BAT1

## Solución de problemas adicionales

### Error de permisos
```bash
# Si hay problemas de permisos con /sys/class/net/
sudo chmod +r /sys/class/net/*/speed
```

### Dependencias faltantes
```bash
# Ubuntu/Debian
sudo apt update
sudo apt install build-essential

# Fedora/RHEL
sudo dnf groupinstall "Development Tools"

# Arch Linux
sudo pacman -S base-devel
```

### Verificar instalación de Rust
```bash
rustc --version
cargo --version

# Si no está instalado:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Reportar problemas

Si encuentras problemas adicionales:
1. Ejecuta `./test-linux-build.sh` y comparte la salida
2. Incluye información del sistema: `uname -a`
3. Versión de Rust: `rustc --version`
4. Distribución Linux: `cat /etc/os-release`

## Cambios implementados

### v0.1.0 - Correcciones Linux
- ✅ Solucionado error de netstat2
- ✅ Corregidos errores de lifetime
- ✅ Implementadas funciones nativas para Linux
- ✅ Dependencias específicas por plataforma
- ✅ Script de prueba automática
- ✅ Compatibilidad completa Linux/Windows