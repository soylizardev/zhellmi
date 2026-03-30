// Declaramos los subarchivos de la carpeta 'core'
pub mod env; // (Opcional por ahora) Para gestión de variables de entorno
pub mod environ;
pub mod executor; // Contendrá la lógica de BusyBox y el PATH dinámico
pub mod parser; // Contendrá la lógica de procesar la entrada y '#'
