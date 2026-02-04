
# Introducción 
Rust es un lenguaje poderoso, seguro y moderno, pero también tiene una curva de aprendizaje particular, especialmente en temas como `ownership`, `borrowing` y `lifetimes`. En ocasiones suele compararse con lenguajes populares como C, C++ o Python, pero su filosofía es bastante distinta. Mientras que en C o C++ el programador es responsable de gestionar manualmente la memoria, Rust introduce un modelo de memoria seguro que evita este tipo de errores en tiempo de compilación, sin necesidad de un `garbage collector`. El `garbage collector` gestiona automáticamente la asignación y liberación de memoria.  Su función principal es identificar y eliminar objetos que ya no están siendo referenciados por el programa, liberando así la memoria que ocupan para que pueda reutilizarse.

Por otro lado, lenguajes como Python o JavaScript priorizan la facilidad de uso y la rapidez de desarrollo mediante un recolector de basura. Esto simplifica mucho la escritura de programas, pero introduce costos en rendimiento, consumo de memoria y falta de control preciso sobre cuándo se libera dicha memoria. Rust busca un punto intermedio poco común: ofrecer seguridad y ergonomía, pero con rendimiento comparable al de C y C++.

Rust se diferencia a otros lenguajes por su modelo de `ownership` (propiedad). Donde cada valor tiene un único dueño, y el compilador se encarga de verificar reglas estrictas sobre cómo los datos se mueven, se prestan o se modifican. Esto permite detectar errores de memoria antes de ejecutar el programa, algo que en otros lenguajes suele descubrirse recién en producción. Aunque este modelo introduce una curva de aprendizaje más pronunciada, es precisamente lo que hace a Rust tan confiable.

Entre sus principales ventajas se encuentran la seguridad de memoria sin garbage collection, un rendimiento predecible, un sistema de tipos muy expresivo y un compilador que actúa como un asistente que guía al programador hacia código correcto. Además, Rust cuenta con herramientas oficiales de alta calidad como cargo, su gestor de paquetes y sistema de construcción, que facilitan enormemente el desarrollo y mantenimiento de proyectos.

Rust es usado principalmente en contextos donde el rendimiento y la seguridad son críticos. Es común encontrarlo en el desarrollo de sistemas operativos, motores de bases de datos, navegadores web, software embebido, herramientas de línea de comandos, servicios backend de alto rendimiento e incluso en blockchain y videojuegos. También está ganando popularidad como alternativa segura para reemplazar código legado escrito en C o C++.

---
## Primeros pasos: ¿Que es cargo?
Cargo es el sistema oficial de compilación y administrador de paquetes del lenguaje de programación Rust. La mayoría de los desarrolladores Rust lo utilizan para gestionar sus proyectos porque automatiza tareas como compilar el código, descargar y compilar bibliotecas externas (llamadas crates) y gestionar dependencias. 

Pero... Exactamente, ¿Qué y como lo hace?: 

* **Gestión de dependencias**: Permite declarar y gestionar fácilmente las bibliotecas externas que necesita un proyecto, descargándolas automáticamente desde `crates.io`. 

* **Compilación y ejecución**: Usa comandos simples como `cargo build` y `cargo run` para compilar y ejecutar proyectos. 

* **Estructura de proyectos**: Crea una estructura de directorios estándar con `cargo new`, separando el código fuente en la carpeta `src/`. 

* **Archivo de configuración**: Utiliza `Cargo.toml` para definir metadatos del proyecto, dependencias y configuraciones. 

* **Reproducibilidad**: Genera un archivo `Cargo.lock` que fija las versiones exactas de las dependencias para garantizar compilaciones consistentes.

---

## **¿Como crear un proyecto nuevo?**
Empezar es realmente sencillo, para ello es necesario ubicarnos en la términal de vscode (si no la visualizas puedes pulsar `ctrl + j` para abrirla manualmente):
![terminal](/images/terminal.png)

donde se escribira el siguiente comando:
```bash
cargo new hola_mundo
```
que creará una nueva carpeta de nombre `hola_mundo` que contiene todos los elementos necesarios para el desarrollo y posterior ejecución del código

![crear_hola_mundo](/images/p_hola_mundo.png)

## **¿Como correr el código?**
Para empezar es necesario posicionarnos en la carpeta del proyecto, lo que hacemos escribiendo el comando:
```bash
cd hola_mundo
```

y posteriormente:
```bash
cargo run
```
lo que comenzará con el proceso de compilación y verificación del código. En este caso, al crear el proyecto con cargo, este crea un archivo .rs (rust) por defecto que contiene una función simple, imprimir en terminal un "Hello World!":

![correr_hola_mundo](/images/cargo_run_hw.png)

## **Continuación: ¿Qué sigue?**
