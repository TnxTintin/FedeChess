# Gestión de Federados
Esta es la primera fase de programacion. 
Poder introducir los federados.

1. Alta Nuevo Federado
2. Modificación Federado
3. Baja o Eliminación Federado.

Es importante, generar un archivo de registro, para poder ver cuando y quien dio de alta. 

Así como las modificaciones de su ficha y/o baja o archivo.
La eliminación de los federados totalmente, solo lo podrán hacer los administradores, que podran eliminara totalmente al federado.

Dependerá de la federación, si no se quiere que se produzca una eliminación y se optara por su archivado (preferible)
Como experiencia, la mejor solución es la baja y archivado.

También se puede producir la desactivación de federado por diferentes motivos.

Lo primero que se tiene que hacer es crear el administrador:
Nombre de usuario Administrador:
root
Nombre, Apellidos, e-mail, Movil. 

Crear grupos de usuarios. Pueden ser de la organización federativa, capitanes de clubes, árbitros, federado u otro tipo de usuario mo especificado.

Cada grupo tendra sus respectivos permisos para cada grupo.

Grupos: 

* Federativos (Secretarios y habilitados)
* Clubes
* Equipos
* Capitanes habilitados
* Árbitros
* Federados

Al ser administrador puede crear otros usuarios para diferentes tareas, y a lo largo del desarrollo se ira indicando que funciones pueden tener cada grupo de usuarios.

Usuarios de alta completos [CRUD]: Crear, Visualizar, Actualizar y Eliminar.

Grupo Federativos: 
> * Podrán de dar de alta a todo tipo de federados y arbitros, asi como su modificación y eliminación. Tambien tendran la facultad de dar autorizaciones a los usuarios de los otros grupos

Grupo Clubes: 
> * Podrán de dar de alta a todo tipo de federado asignado a ese Club, así como su modificación y eliminación estrictamente del responsable de ese club. Podrán asimismo, dar autorizaciones a sus capitanes

Grupo Capitanes: 
> * Podrán dar de alta, modificar y eliminación estrictamente del responsable de ese equipo.

Grupo Árbitros: 
> * Podrán actualizar la situación del federado en lo relativo a competición. Asimismo podrán dar autorizaciones a otros árbitros nuevos. Seria como un superarbitro.

Grupo Monitores: 
> * Podrán dar de alta, modificar y eliminación estrictamente del grupo de monitores. Ver quienes pueden darse de alta en el grupo de monitores.

Grupo Federados: 
> * Solo podra visualizar su ficha personal. En todo caso, se le puede habilitar ocultar algunos campos personales del público general.

La ficha de un federado completo ():

1. Id_ Autonomico: El Id de la federacion autonoma.
2. Id_Nacional: El Id nacional de España
3. Id_ Fide: El Id Internacional de la Fide
4. Id_FOA: El Id del Fide Online Arena (Algunas veces no coincide con el de FIDE)

5. Nombre autonomía: El nombre de la Autonomía. 
6. Provincia: La provincia con la que se federa. La provincia activa parta las competiciones.
7. Nombre: Nombre del federado.
8. Apellidos del federado.
9. Sexo.
10. Fecha de Nacimiento: La fecha de nacimiento que determinará en tiempo real a que categoría pertenece.
11. Discapacidad: Si el federado sufre algún tipo de discapacidad (solo lo pueden ver los administradores y árbitros.)
12. Activo: Si está activo o inactivo para las competiciones.
13. NIF-NIE: El NIF o NIE del federado.
14. Direccion: La dirección de residencia del federado.
15. Municipio: Municipio donde reside. 
16. e-mail: Email de contacto. 
17. Movil: El móvil de contacto. (Solo lo pueden ver los autorizados)
18. Nacionalidad.
19. Elo Autonómico
20. Elo Nacional
21. Elo FIDE Standard
22. Elo FIDE Rapid
23. Elo FIDE Blitz
24. ELO FIDE Arena Standard
25. ELO FIDE Arena Rapid
26. ELO FIDE Arena Bullet
27. TituloPlayer: Título obtenido como jugador (GM, IM, FM, CM, WGM, WIM, WFM, WCM). (Ver como implementar normas y títulos especiales, como campeones del mundo, continentales, etc)
28. TituloMonitor: Titulación conseguida como monitor o entrenador.
29. Titulo Arbitro: Tipo de Título arbitral y ámbito de actuación.

Hay que definir cuáles son los campos fijos y variables, y que tratamiento darles, para separar los datos en tablas diferentes. Así como crear el historial, para cada uno de los campos. 



