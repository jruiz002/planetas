# 🪐 Laboratorio de Planetas con Shaders

Un proyecto de renderizado 3D en Rust que utiliza únicamente shaders para crear planetas procedurales sin texturas externas.

## 📋 Descripción

Este laboratorio implementa un sistema de renderizado por software que genera tres tipos únicos de planetas celestiales usando solo geometría esférica y shaders procedurales. Cada planeta tiene características visuales distintivas creadas completamente mediante código, sin usar texturas o materiales pre-cargados.

## 🎯 Características Principales

- **Renderizado por Software**: Pipeline completo implementado desde cero
- **Shaders Procedurales**: Efectos visuales generados algorítmicamente
- **Tres Planetas Únicos**: Cada uno con propiedades físicas y visuales distintas
- **Controles Interactivos**: Cámara orbital con zoom y rotación
- **Sin Texturas Externas**: Todo generado proceduralmente

## 🌍 Planetas Implementados

### 1. Planeta Rocoso
![Planeta Rocoso](images/planeta_1.png)

- **Características**: Superficie rugosa con cráteres y montañas
- **Shader**: Desplazamiento de vértices con ruido Perlin
- **Colores**: Tonos tierra (marrón, gris, naranja)
- **Rotación**: Lenta (0.5 rad/s)

### 2. Gigante Gaseoso
![Gigante Gaseoso](images/planeta_2.png)

- **Características**: Bandas atmosféricas dinámicas con turbulencia
- **Shader**: Efectos de flujo y remolinos atmosféricos
- **Colores**: Azul-púrpura con variaciones atmosféricas
- **Rotación**: Rápida (1.2 rad/s)

### 3. Planeta de Cristal
![Planeta de Cristal](images/planeta_3.png)

- **Características**: Superficie cristalina con efectos de refracción
- **Shader**: Efectos metálicos y cristalinos
- **Colores**: Cian-magenta con brillo especular
- **Rotación**: Media (0.8 rad/s)

## 🎮 Controles

| Control | Acción |
|---------|--------|
| **1, 2, 3** | Cambiar entre planetas |
| **Click Izquierdo + Arrastrar** | Rotar cámara |
| **Rueda del Ratón** | Zoom in/out |
| **Click Derecho + Arrastrar** | Mover cámara |
| **ESC** | Salir de la aplicación |

## 🛠️ Tecnologías Utilizadas

- **Lenguaje**: Rust 2021
- **Gráficos**: Raylib 5.0
- **Matemáticas**: nalgebra 0.32
- **Generación Procedural**: rand 0.8

## 📁 Estructura del Proyecto

```
src/
├── main.rs          # Loop principal y lógica de renderizado
├── vector.rs        # Operaciones matemáticas 3D
├── matrix.rs        # Transformaciones matriciales
├── camera.rs        # Sistema de cámara orbital
├── sphere.rs        # Generación de geometría esférica
└── shaders.rs       # Sistema de shaders procedurales
```

## 🚀 Instalación y Ejecución

### Prerrequisitos

- Rust 1.70+ instalado
- Sistema operativo compatible con Raylib (Windows, macOS, Linux)

### Pasos

1. **Clonar el repositorio**:
   ```bash
   git clone <url-del-repositorio>
   cd planetas
   ```

2. **Compilar el proyecto**:
   ```bash
   cargo build --release
   ```

3. **Ejecutar la aplicación**:
   ```bash
   cargo run --release
   ```

## 🔧 Arquitectura Técnica

### Sistema de Shaders

El proyecto implementa un trait `PlanetShader` que define la interfaz para los shaders:

```rust
pub trait PlanetShader {
    fn vertex_shader(&self, vertex: &Vertex, uniforms: &ShaderUniforms) -> (Vector3, Vector3);
    fn fragment_shader(&self, position: &Vector3, normal: &Vector3, uniforms: &ShaderUniforms) -> ShaderColor;
}
```

### Pipeline de Renderizado

1. **Generación de Geometría**: Creación procedural de esferas
2. **Vertex Shader**: Transformación de vértices y normales
3. **Transformaciones**: Aplicación de matrices de vista, proyección y viewport
4. **Fragment Shader**: Cálculo de colores por píxel
5. **Rasterización**: Dibujo de triángulos en pantalla

### Efectos Procedurales

- **Ruido Perlin**: Para superficies rugosas y variaciones naturales
- **Funciones de Mezcla**: Para transiciones suaves de colores
- **Desplazamiento de Vértices**: Para crear relieve en las superficies
- **Iluminación Direccional**: Para efectos de sombreado realistas

## 📊 Rendimiento

- **FPS Target**: 60 FPS
- **Resolución**: 1024x768
- **Geometría**: ~2048 triángulos por esfera (32x32 subdivisiones)
- **Renderizado**: Software rendering optimizado

## 🎓 Propósito Educativo

Este proyecto fue desarrollado como parte del curso de Gráficas por Computadora para demostrar:

- Implementación de pipelines de renderizado desde cero
- Creación de efectos visuales procedurales
- Uso de shaders sin hardware acelerado
- Matemáticas 3D aplicadas (vectores, matrices, transformaciones)
- Arquitectura de software para gráficos en tiempo real

---

**Desarrollado en Rust**