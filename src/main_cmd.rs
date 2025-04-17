use std::io;

fn main() {
    loop {
        // Mostrar el menú principal
        println!("=== MENÚ PRINCIPAL ===");
        println!("1. Jugadores");
        println!("2. Torneos");
        println!("3. Ayuda");
        println!("4. Salir");

        // Leer la opción del usuario
        let opcion = leer_opcion();

        match opcion {
            1 => menu_jugadores(),
            2 => menu_torneos(),
            3 => mostrar_ayuda(),
            4 => {
                println!("Saliendo del programa...");
                break;
            }
            _ => println!("Opción inválida. Por favor, intenta de nuevo."),
        }
    }
}

fn menu_jugadores() {
    loop {
        // Mostrar el menú de jugadores
        println!("\n=== MENÚ JUGADORES ===");
        println!("1. Nuevo");
        println!("2. Listado");
        println!("3. Buscar");
        println!("4. Eliminar");
        println!("5. Volver al menú principal");

        // Leer la opción del usuario
        let opcion = leer_opcion();

        match opcion {
            1 => println!("Seleccionaste: Nuevo jugador"),
            2 => println!("Seleccionaste: Listado de jugadores"),
            3 => println!("Seleccionaste: Buscar jugador"),
            4 => println!("Seleccionaste: Eliminar jugador"),
            5 => break,
            _ => println!("Opción inválida. Por favor, intenta de nuevo."),
        }
    }
}

fn menu_torneos() {
    loop {
        // Mostrar el menú de torneos
        println!("\n=== MENÚ TORNEOS ===");
        println!("1. Nuevo");
        println!("2. Listado");
        println!("3. Buscar");
        println!("4. Eliminar");
        println!("5. Volver al menú principal");

        // Leer la opción del usuario
        let opcion = leer_opcion();

        match opcion {
            1 => println!("Seleccionaste: Nuevo torneo"),
            2 => println!("Seleccionaste: Listado de torneos"),
            3 => println!("Seleccionaste: Buscar torneo"),
            4 => println!("Seleccionaste: Eliminar torneo"),
            5 => break,
            _ => println!("Opción inválida. Por favor, intenta de nuevo."),
        }
    }
}

fn mostrar_ayuda() {
    println!("\n=== AYUDA ===");
    println!("Este programa permite gestionar jugadores y torneos.");
    println!("Desde el menú principal, selecciona una opción para acceder a sus submenús.");
    println!("En cada submenú, puedes realizar acciones específicas como crear, listar, buscar o eliminar registros.");
    println!("Para regresar al menú principal, selecciona la opción 'Volver'.");
    println!("Para salir del programa, selecciona la opción 'Salir' desde el menú principal.");
}

fn leer_opcion() -> u32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada");

    input.trim().parse::<u32>().unwrap_or(0)
}
