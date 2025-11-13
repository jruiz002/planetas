# Laboratorio de Planetas - Shaders Procedurales

Este proyecto implementa un **software renderer** que visualiza diferentes tipos de cuerpos celestes utilizando únicamente **shaders procedurales**. Todos los efectos visuales son generados mediante cálculos matemáticos en vertex y fragment shaders, sin uso de texturas o materiales externos.

## � Descripción del Proyecto

El laboratorio cumple con todos los requisitos establecidos:
- ✅ **Tres tipos de planetas**: Rocoso, Gaseoso, Cristal, y Lava (4 implementados)
- ✅ **Solo esferas base**: Usa el archivo `sphere.obj` proporcionado
- ✅ **Sin texturas**: Todo es procedural con shaders
- ✅ **Software renderer**: Pipeline de renderizado personalizado
- ✅ **Características extras**: Anillos y luna procedurales

## 🪐 Planetas Implementados

### 1. Planeta Rocoso (Tecla 1)
- **Superficie rugosa** con montañas y cráteres
- **4 capas de efectos**: Montañas (ridge noise), cráteres (Voronoi), rugosidad (fbm), minerales
- **Incluye luna orbital** procedural

### 2. Gigante Gaseoso (Tecla 2)  
- **Bandas atmosféricas** dinámicas que cambian con el tiempo
- **4 capas de efectos**: Bandas base, turbulencia, vórtices, brillos atmosféricos
- **Incluye sistema de anillos** procedurales

### 3. Planeta de Cristal (Tecla 3)
- **Efectos cristalinos** con refracción y brillos especulares
- **4 capas de efectos**: Cristales base, refracción, especular, patrones de energía
- **Incluye sistema de anillos** procedurales

### 4. Planeta de Lava (Tecla 4)
- **Mundo volcánico** con lava fundida y actividad geotérmica  
- **4 capas de efectos**: Roca volcánica, lava fundida, emisión de calor, resplandor

## 🛠️ Librerías Utilizadas

### **Raylib** - Biblioteca gráfica principal
- **Por qué**: Proporciona una API simple para crear ventanas, manejar entrada y renderizar primitivas básicas
- **Uso**: Creación de ventana, manejo de eventos de teclado/mouse, funciones de dibujo (triangulos, líneas, píxeles)
- **Ventaja**: Permite enfocarse en los shaders sin lidiar con OpenGL directamente

### **Rust Standard Library**
- **std::f32::consts::PI**: Constantes matemáticas para cálculos trigonométricos
- **std::fs::File, std::io**: Para cargar el archivo sphere.obj

### Librerías **NO** utilizadas intencionalmente:
- **No nalgebra/glam**: Implementé mi propio sistema de vectores y matrices para entender la matemática 3D
- **No image/texture loading**: Cumple con la restricción de no usar texturas externas
- **No OpenGL directo**: Raylib abstrae la complejidad del renderizado de hardware

## 🏗️ Arquitectura del Proyecto

### **Estructura Modular**
```
src/
├── main.rs           # Loop principal y coordinación
├── vector.rs         # Matemática vectorial personalizada  
├── matrix.rs         # Transformaciones 3D (view, projection, viewport)
├── camera.rs         # Sistema de cámara orbital
├── sphere.rs         # Estructura de mesh y vértices
├── shaders.rs        # Todos los shaders planetarios
└── obj_loader.rs     # Cargador del archivo sphere.obj
```

### **Pipeline de Renderizado (Software)**
1. **Carga de Geometría**: `obj_loader.rs` parsea sphere.obj
2. **Transformaciones**: `matrix.rs` aplica model-view-projection
3. **Vertex Shader**: `shaders.rs` deforma la geometría
4. **Proyección**: Convierte 3D a coordenadas de pantalla
5. **Fragment Shader**: `shaders.rs` calcula el color final
6. **Rasterización**: Raylib dibuja los triángulos resultantes

### **Sistema de Shaders**
```rust
pub trait PlanetShader {
    fn vertex_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> (Vector3, Vector3);
    fn fragment_shader(&self, position: Vector3, normal: Vector3, uv: (f32, f32), uniforms: &ShaderUniforms) -> ShaderColor;
}
```

- **Vertex Shader**: Modifica la posición de los vértices (deformación del terreno)
- **Fragment Shader**: Calcula el color final usando múltiples capas de ruido
- **Uniforms**: Parámetros globales (tiempo, posición de luz, cámara)

### **Algoritmos de Ruido Procedural**
- **Simple Noise**: Ruido básico pseudo-aleatorio
- **FBM (Fractal Brownian Motion)**: Múltiples octavas de ruido para patrones complejos
- **Voronoi Noise**: Patrones celulares para cráteres
- **Ridge Noise**: Ruido con crestas para montañas

## 🎮 Controles

- **1-4**: Cambiar entre planetas
- **WASD**: Rotar cámara
- **Flechas**: Zoom y paneo horizontal  
- **Q/E**: Paneo horizontal
- **R/F**: Paneo vertical

## 🚀 Compilación y Ejecución

```bash
# Compilar
cargo build --release

# Ejecutar  
cargo run --release
```

## ⭐ Características Técnicas Destacadas

### **Múltiples Capas por Shader**
Cada planeta implementa **4+ capas** de efectos que se combinan:
- Colores base del material
- Efectos de ruido procedural
- Iluminación avanzada (difusa, especular, rim lighting)
- Efectos temporales animados

### **Elementos Procedurales Adicionales**
- **Anillos**: 8 anillos concéntricos con rotación diferencial
- **Luna**: Órbita realista con cráteres procedurales  
- **Rotación planetaria**: Cada planeta rota a velocidad diferente

### **Matemática 3D Personalizada**
- Sistema completo de vectores y matrices implementado desde cero
- Transformaciones model-view-projection manuales
- Cámara orbital con controles intuitivos

---

**Desarrollado para Gráficas por Computadora - Universidad del Valle de Guatemala**  
**Por: José Ruiz**