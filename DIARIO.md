# Diario Fedechess

23-06-2026

- Cerrado el importador de la Base de datos de la FIDE. Al final se tuvo que recurrir al XML.
- Modificado campo de jugadores para que se el id de provincia en vez de provincia.
- Generado los Clubes y descapitalizandolos de Origen GEFE ( Incorporar mas adelante los historicos).
- Reorganizar los TODO.
- Crear Tunnel para la base de datos. srv.tinux.net
- Migrar la base de datos de localhost a srv.tinux.net
  - Creado comando para historial de mariadb:
    Para Shell: mariadb-history='cat ~/.mariadb_history | sed "s|\\\\040| |g"'
    Para Mariadb CLI: system cat ~/.mariadb_history | sed "s|\\040| |g"

22-06-2026

- Conseguido primer TUI Funcional
- Comprimido los sql a tar.gz
- Reorganizando la estructura de los branchs

05-05-2026

- Creado github publico para desarrollo.
- Creado estructura en Local para soporta github.

2025-11-06

    -Acualizando los los gits, y posible movimiento a nuevo servidor.

2025-10-18

    - Ir pensando en la variante de emparejaminentos Tnx. Partir el numero de jugadores en bloques respecto al numero de rondas e ir reducciendo las distancias de fuerza de juego. Hacer un README o Milestone para describir cuales son los objetivos y que metodos para llevarlo a cabo.

2025-10-07

    - Cambiado el nombre de tablas y campos a minusculas en la base de datos.
    - Se van a poner los branchs a ingles. y crear una tabla para no estar consultandolo permanentemente.

2025-08-30

    - Habilitando Lapce como IDE de edicion. Creado precisamente en Rust

2025-04-17

    - Acceso a traves de tunel a la base de datos en el servidor.
        ssh -i ~/.ssh/id_ed25519 -f  -L 3310:127.0.0.1:3306 tinux@tinux.net -N
        mariadb -h localhost -P 3310 fedechess -u fedechess -p --ssl=OFF

2025-04-13

    Poniendo a punto los branchs:
    actas, arbitros,  circulares,  clubes,  commons,  delegaciones,  divisiones,  documentaciones,  elo,  federados,  habilitados,  inscripciones,  jugadores,  logins, main,  monitores,  reglamentos,  torneos,  transeuntes,  web
    Creado Subdominio en tinux.net como fedechess.tinux.net
    Crear le fedechess en Local para las pruebas. local.fedechess.tinux.net
    Limpieza y bifurcaciones de tamas e independizadas.

2025-04-12

    Retomando la Programacion
    Crear base de Datos de Jugadores segun GEFE.
        Tipo Identificador (NIF/CIF/Passport),  NIF/CIF Nombre, Apellidos, Fecha Nacimiento, Sexo, email, email2, CP, Localidade, Direecion, Telefono, Movil, Nacionalidad, Codigo Regional, Elo Regional, Codigo Feda, Elo Feda, Codigo Fide, Elo Fide
    Crear Base de Datos de Equipos por Temporadas. DH Andaluza, Primera Andaluza, Preferentes Provinciales, Primeras Provinciales, Segunda Provinciales.

2024-03-30

- Disgregado entorno Web con su rama especifica:

  To gitlab.com:Tinotin/fedechess.git
  - [new branch] web -> web

2024-03-09

    Poniendo al dia, y poniendo objetivos inmediatos
    Creada la estructura de Fedechess para el Elo, Jugadores, Fecha de nacimiento y sus Id's de Federacion Internacional y Andaluza

2024-03-09

    Acondicionando lugar de trabajo

2023-05-23

- Agregado README Licences
- Redactando Documentacion de Federated Members
- Seleccionando los campos globales del federado.
- Agregado los ficheros de TODO y Monitors
- Preparado para continuar con _Rust_
