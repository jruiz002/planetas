# Laboratorio de Planetas con Shaders

Un proyecto de renderizado de planetas procedurales utilizando únicamente shaders en Rust con Raylib. Este laboratorio demuestra la creación de cuerpos celestes complejos sin texturas externas, usando solo matemáticas y algoritmos procedurales.

## 🌟 Características Principales

- **4 Planetas Únicos**: Cada uno con shaders procedurales complejos
- **Anillos Procedurales**: Sistema de anillos generados completamente con Vertex Shaders
- **Luna Orbital**: Luna procedural que orbita alrededor de los planetas
- **Efectos Visuales Avanzados**: Múltiples capas de color, gradientes, iluminación simulada
- **Controles Interactivos**: Navegación 3D completa y cambio de planetas
- **Rotación y Traslación**: Movimientos planetarios realistas

## 🪐 Planetas Implementados

### 1. Planeta Rocoso (Tecla 1)
**Características técnicas:**
- **Vertex Shader**: Deformación de superficie con múltiples capas de noise (simple_noise, fbm, voronoi_noise)
- **Fragment Shader**: 6+ capas de efectos visuales
  - Variaciones de altitud con colores de roca/arena
  - Simulación de temperatura basada en posición
  - Iluminación avanzada (difusa, especular, oclusión ambiental, rim lighting)
  - Efectos atmosféricos sutiles

**Parámetros del Shader:**
- `altitude_variation`: Controla la variación de color por altura
- `temperature_simulation`: Simula zonas calientes/frías
- `surface_roughness`: Rugosidad de la superficie rocosa
- `atmospheric_haze`: Efecto de atmósfera tenue

![Planeta Rocoso](images/planeta_1.png)

### 2. Gigante Gaseoso (Tecla 2)
**Características técnicas:**
- **Vertex Shader**: Ondas atmosféricas y turbulencia dinámica
- **Fragment Shader**: 8+ capas de efectos atmosféricos
  - Múltiples capas atmosféricas con colores distintos
  - Bandas complejas con turbulencia
  - Efectos de tormenta (Gran Mancha Roja simulada)
  - Rayos atmosféricos animados
  - Dispersión de luz y rim lighting avanzado

**Parámetros del Shader:**
- `atmospheric_layers`: 4 capas atmosféricas distintas
- `storm_intensity`: Intensidad de las tormentas
- `band_complexity`: Complejidad de las bandas atmosféricas
- `lightning_frequency`: Frecuencia de rayos atmosféricos
- `gas_density`: Densidad del gas con transparencia variable

![Gigante Gaseoso](images/planeta_2.png)

### 3. Planeta de Cristal (Tecla 3)
**Características técnicas:**
- **Vertex Shader**: Formaciones cristalinas multicapa con pulsaciones animadas
- **Fragment Shader**: 7+ capas de efectos cristalinos
  - Patrones de color complejos con transiciones suaves
  - Efectos de energía interna
  - Múltiples reflexiones especulares
  - Refracción simulada
  - Luz interna con cambios de color dinámicos

**Parámetros del Shader:**
- `crystal_formations`: Múltiples capas de cristales
- `energy_pulse`: Pulsaciones de energía animadas
- `internal_light`: Luz interna con variaciones de color
- `refraction_index`: Simulación de refracción
- `transparency_variation`: Variaciones de transparencia

![Planeta de Cristal](images/planeta_3.png)

### 4. Planeta de Lava (Tecla 4)
**Características técnicas:**
- **Vertex Shader**: Deformación volcánica con flujos de lava y actividad pulsante
- **Fragment Shader**: 6+ capas de efectos volcánicos
  - Transiciones de color basadas en temperatura
  - Patrones de flujo de lava animados
  - Grietas volcánicas brillantes
  - Emisión de calor simulada
  - Resplandor volcánico dinámico

**Parámetros del Shader:**
- `volcanic_activity`: Intensidad de la actividad volcánica
- `lava_flow_speed`: Velocidad de los flujos de lava
- `heat_emission`: Emisión de calor con colores cálidos
- `volcanic_glow`: Resplandor volcánico ambiental
- `magma_chambers`: Cámaras de magma internas

![Planeta de Lava](images/planeta_4.png)

## 🌙 Elementos Procedurales Adicionales

### Sistema de Anillos Procedurales (20 puntos)
**Implementación con Vertex Shader:**
- **Generación**: 8 anillos concéntricos con espaciado variable
- **Efectos Visuales**:
  - Variaciones de densidad usando noise procedural
  - Gaps naturales en los anillos
  - Rotación diferencial (cada anillo rota a velocidad distinta)
  - Partículas brillantes ocasionales
  - Bandas de color con patrones complejos
- **Parámetros**:
  - `ring_count`: Número de anillos (8)
  - `ring_spacing`: Espaciado entre anillos (0.3)
  - `rotation_speed`: Velocidad de rotación variable
  - `density_variation`: Variaciones de densidad
  - `sparkle_frequency`: Frecuencia de partículas brillantes

### Luna Procedural (20 puntos)
**Implementación con Vertex Shader:**
- **Órbita**: Movimiento orbital realista alrededor del planeta
- **Características**:
  - Escala reducida (30% del planeta)
  - Rotación propia sincronizada
  - Cráteres procedurales con deformación de superficie
  - Variaciones de superficie lunar realistas
- **Efectos Visuales**:
  - Múltiples patrones de cráteres
  - Detalles de superficie con fbm
  - Iluminación lunar con rim lighting
  - Variaciones de color gris lunar
- **Parámetros**:
  - `orbit_radius`: Radio orbital (3.0)
  - `orbit_speed`: Velocidad orbital (0.8)
  - `moon_scale`: Escala de la luna (0.3)
  - `crater_density`: Densidad de cráteres
  - `surface_roughness`: Rugosidad de la superficie

## 🎮 Controles Interactivos

- **1-4**: Cambiar entre planetas
- **Click izquierdo + arrastrar**: Rotar cámara
- **Rueda del ratón**: Zoom in/out
- **Click derecho + arrastrar**: Mover cámara

## 🛠️ Tecnologías Utilizadas

- **Rust**: Lenguaje de programación principal
- **Raylib**: Biblioteca de gráficos y ventanas
- **nalgebra**: Matemáticas vectoriales y matriciales
- **rand**: Generación de números aleatorios para efectos procedurales

## 📁 Estructura del Proyecto

```
src/
├── main.rs           # Bucle principal y lógica de renderizado
├── vector.rs         # Implementación de Vector3 personalizado
├── matrix.rs         # Operaciones matriciales para transformaciones
├── camera.rs         # Sistema de cámara 3D interactiva
├── sphere.rs         # Generación de malla esférica y renderizado
└── shaders.rs        # Todos los shaders planetarios y efectos
```

## 🚀 Instalación y Ejecución

### Prerrequisitos
- Rust (versión 1.70 o superior)
- Cargo (incluido con Rust)

### Pasos de instalación
```bash
# Clonar el repositorio
git clone https://github.com/jruiz002/planetas.git
cd planetas

# Compilar el proyecto
cargo build --release

# Ejecutar la aplicación
cargo run --release
```

## 🔧 Arquitectura Técnica

### Sistema de Shaders
- **Trait PlanetShader**: Interfaz común para todos los shaders planetarios
- **Vertex Shaders**: Deformación de geometría procedural
- **Fragment Shaders**: Cálculo de color por píxel con múltiples capas
- **Uniforms Compartidos**: `time`, `light_direction`, `camera_position`

### Pipeline de Renderizado
1. **Generación de Malla**: Esfera base con 32x32 subdivisiones
2. **Transformación de Vértices**: Aplicación de vertex shaders
3. **Rasterización**: Conversión a píxeles de pantalla
4. **Sombreado de Fragmentos**: Cálculo de color final por píxel
5. **Composición**: Renderizado final con efectos adicionales

### Efectos Procedurales
- **Simple Noise**: Ruido básico para variaciones
- **Fractional Brownian Motion (FBM)**: Ruido complejo multicapa
- **Voronoi Noise**: Patrones celulares
- **Ridge Noise**: Ruido con crestas para efectos montañosos

## 📊 Métricas de Rendimiento

- **FPS Target**: 60 FPS constantes
- **Resolución**: 1024x768 píxeles
- **Vértices por Esfera**: 2,048 vértices (32x32)
- **Shaders Activos**: 1 planeta + anillos + luna simultáneamente
- **Complejidad de Shader**: 6-8 capas de efectos por planeta

---

**Desarrollado para el curso de Gráficas por Computadora - Universidad del Valle de Guatemala**