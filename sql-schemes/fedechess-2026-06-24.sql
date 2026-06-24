/*M!999999\- enable the sandbox mode */ 
-- MariaDB dump 10.19-11.8.8-MariaDB, for Linux (x86_64)
--
-- Host: localhost    Database: fedechess
-- ------------------------------------------------------
-- Server version	11.8.6-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*M!100616 SET @OLD_NOTE_VERBOSITY=@@NOTE_VERBOSITY, NOTE_VERBOSITY=0 */;

--
-- Table structure for table `categorias`
--

DROP TABLE IF EXISTS `categorias`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `categorias` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `nombre` varchar(100) NOT NULL,
  `descripcion` varchar(255) DEFAULT NULL,
  `orden` int(11) NOT NULL DEFAULT 0,
  `activo` tinyint(1) NOT NULL DEFAULT 1,
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `nombre` (`nombre`)
) ENGINE=InnoDB AUTO_INCREMENT=12 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `clubes`
--

DROP TABLE IF EXISTS `clubes`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `clubes` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `nombre` varchar(255) NOT NULL,
  `provincia` varchar(100) NOT NULL,
  `localidad` varchar(255) NOT NULL,
  `delegado` varchar(255) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=133 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `elo_fedechess`
--

DROP TABLE IF EXISTS `elo_fedechess`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `elo_fedechess` (
  `id_fada` int(11) NOT NULL,
  `id_fide` int(10) NOT NULL,
  `nombre` varchar(100) NOT NULL,
  `apellidos` varchar(100) DEFAULT NULL,
  `fnac` int(4) unsigned DEFAULT NULL,
  `titulo` varchar(5) DEFAULT NULL,
  `elo_fada` int(11) DEFAULT NULL,
  `standard` int(11) DEFAULT NULL,
  `rapid` int(11) DEFAULT NULL,
  `blitz` int(11) DEFAULT NULL,
  `elo_previo` int(11) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `elo_fide_fada`
--

DROP TABLE IF EXISTS `elo_fide_fada`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `elo_fide_fada` (
  `NAME` varchar(50) DEFAULT NULL,
  `COUNTRY` varchar(50) DEFAULT NULL,
  `BIRTHDAY` int(11) DEFAULT NULL,
  `G` varchar(50) DEFAULT NULL,
  `TITLE` varchar(50) DEFAULT NULL,
  `IDFIDE` int(11) DEFAULT NULL,
  `ELOFIDE` int(11) DEFAULT NULL,
  `KFIDE` varchar(50) DEFAULT NULL,
  `IDNAT` int(11) DEFAULT NULL,
  `ELONAT` int(11) DEFAULT NULL,
  `KNAT` varchar(50) DEFAULT NULL,
  `CLUB` varchar(50) DEFAULT NULL,
  `INFO` varchar(50) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `equipos`
--

DROP TABLE IF EXISTS `equipos`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `equipos` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `club_id` int(10) unsigned NOT NULL,
  `categoria_id` int(10) unsigned NOT NULL,
  `nombre` varchar(255) NOT NULL COMMENT 'Nombre del equipo (ej: "Club X - Equipo A")',
  `temporada` varchar(20) NOT NULL COMMENT 'Ej: "2025-2026"',
  `provincia` varchar(100) DEFAULT NULL,
  `localidad` varchar(100) DEFAULT NULL,
  `lugar_juego` varchar(255) DEFAULT NULL COMMENT 'Dirección del local de juego',
  `dia_juego` varchar(50) DEFAULT NULL COMMENT 'Ej: "Sábados 17:00"',
  `activo` tinyint(1) NOT NULL DEFAULT 1,
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `uk_equipos_nombre_temporada` (`nombre`,`temporada`),
  KEY `fk_equipos_categoria` (`categoria_id`),
  KEY `idx_equipos_club` (`club_id`),
  KEY `idx_equipos_temporada` (`temporada`),
  KEY `idx_equipos_activo` (`activo`),
  CONSTRAINT `fk_equipos_categoria` FOREIGN KEY (`categoria_id`) REFERENCES `categorias` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `federados`
--

DROP TABLE IF EXISTS `federados`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `federados` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `id_fada` varchar(20) NOT NULL COMMENT 'Código federativo (ej: CAT-12345)',
  `id_fide` int(10) unsigned DEFAULT NULL COMMENT 'ID FIDE internacional',
  `nombre` varchar(100) NOT NULL,
  `apellidos` varchar(100) NOT NULL,
  `fnac` int(4) unsigned DEFAULT NULL,
  `tipo_documento` enum('DNI','NIE','PASSPORT') NOT NULL,
  `numero_documento` varchar(20) NOT NULL,
  `genero` enum('M','F') DEFAULT NULL COMMENT 'Género para categorías',
  `email` varchar(150) DEFAULT NULL,
  `telefono` varchar(20) DEFAULT NULL,
  `direccion` varchar(255) DEFAULT NULL,
  `cp` varchar(10) DEFAULT NULL,
  `localidad` varchar(100) DEFAULT NULL,
  `provincia` varchar(100) DEFAULT NULL,
  `elo_standard` int(10) unsigned DEFAULT 0 COMMENT 'Elo estándar FIDE',
  `elo_rapid` int(10) unsigned DEFAULT 0 COMMENT 'Elo rápido FIDE',
  `elo_blitz` int(10) unsigned DEFAULT 0 COMMENT 'Elo blitz FIDE',
  `titulo_fide` enum('GM','IM','FM','CM','WGM','WIM','WFM','WCM','NONE') DEFAULT 'NONE',
  `titulo_nacional` enum('MN','MA','NONE') DEFAULT 'NONE' COMMENT 'Título nacional',
  `id_club` int(10) unsigned DEFAULT NULL COMMENT 'FK a tabla clubs (pendiente)',
  `categoria` enum('SUB-8','SUB-10','SUB-12','SUB-14','SUB-16','SUB-18','ABSOLUTO','VETERANO') DEFAULT NULL COMMENT 'Categoría por edad',
  `alta_federativa` int(10) unsigned DEFAULT NULL COMMENT 'Año de primera federación',
  `activo` tinyint(1) DEFAULT 1,
  `created_at` timestamp NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  `elo_fada` int(10) unsigned DEFAULT 1000 COMMENT 'Elo estándar FIDE',
  PRIMARY KEY (`id`),
  UNIQUE KEY `federation_code` (`id_fada`),
  KEY `idx_name` (`apellidos`,`nombre`),
  KEY `idx_federation_code` (`id_fada`),
  KEY `idx_fide_id` (`id_fide`),
  KEY `idx_active` (`activo`),
  KEY `idx_elo` (`elo_standard` DESC),
  KEY `idx_club` (`id_club`)
) ENGINE=InnoDB AUTO_INCREMENT=4101 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `fide_players`
--

DROP TABLE IF EXISTS `fide_players`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `fide_players` (
  `id_fide` int(10) unsigned NOT NULL,
  `nombre` varchar(200) DEFAULT NULL,
  `federacion` char(3) DEFAULT NULL,
  `sexo` char(1) DEFAULT NULL,
  `titulo` varchar(5) DEFAULT NULL,
  `titulo_femenino` varchar(5) DEFAULT NULL,
  `titulo_arbitro` varchar(20) DEFAULT NULL,
  `elo_standard` int(11) DEFAULT 0,
  `partidas_standard` int(11) DEFAULT 0,
  `k_standard` int(11) DEFAULT 0,
  `elo_rapid` int(11) DEFAULT 0,
  `partidas_rapid` int(11) DEFAULT 0,
  `k_rapid` int(11) DEFAULT 0,
  `elo_blitz` int(11) DEFAULT 0,
  `partidas_blitz` int(11) DEFAULT 0,
  `k_blitz` int(11) DEFAULT 0,
  `anio_nacimiento` int(11) DEFAULT NULL,
  `flag` varchar(10) DEFAULT NULL,
  `fecha_actualizacion` date DEFAULT curdate(),
  PRIMARY KEY (`id_fide`),
  KEY `idx_nombre` (`nombre`),
  KEY `idx_federacion` (`federacion`),
  KEY `idx_elo_std` (`elo_standard` DESC),
  KEY `idx_titulo` (`titulo`),
  KEY `idx_anio` (`anio_nacimiento`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `jugadores`
--

DROP TABLE IF EXISTS `jugadores`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `jugadores` (
  `id` int(11) NOT NULL AUTO_INCREMENT,
  `id_provincia` int(10) unsigned DEFAULT NULL,
  `provincia` varchar(100) NOT NULL,
  `club` varchar(150) NOT NULL,
  `apellidos` varchar(100) NOT NULL,
  `nombre` varchar(100) NOT NULL,
  `ano_nacimiento` int(11) NOT NULL,
  `sexo` enum('Masculino','Femenino') DEFAULT NULL,
  `fecha_registro` timestamp NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  KEY `idx_provincia` (`provincia`),
  KEY `idx_club` (`club`),
  KEY `idx_apellidos` (`apellidos`),
  KEY `idx_ano_nacimiento` (`ano_nacimiento`),
  KEY `idx_sexo` (`sexo`)
) ENGINE=InnoDB AUTO_INCREMENT=3903 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_uca1400_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `municipios`
--

DROP TABLE IF EXISTS `municipios`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `municipios` (
  `id` mediumint(8) unsigned NOT NULL COMMENT 'Concatenación de ID CCAA (1-19) + Cod Prov INE (PP) + Cod Muni INE (MMM)',
  `id_provincia` tinyint(3) unsigned NOT NULL COMMENT 'Cod Provincia del INE, sin cero',
  `nombre` varchar(64) NOT NULL,
  `cp` mediumint(8) unsigned DEFAULT NULL,
  `urlized` varchar(64) NOT NULL COMMENT 'Para URLs. Únicos cada columna',
  PRIMARY KEY (`id`),
  UNIQUE KEY `urlized` (`urlized`),
  KEY `id_provincia` (`id_provincia`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb3 COLLATE=utf8mb3_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `provincias`
--

DROP TABLE IF EXISTS `provincias`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `provincias` (
  `id` tinyint(3) unsigned NOT NULL AUTO_INCREMENT,
  `nombre` varchar(50) NOT NULL COMMENT 'Nombre oficial (ej: Sevilla)',
  `nombre_gefe` varchar(50) NOT NULL COMMENT 'Nombre como aparece en GEFE (ej: SEVILLA)',
  `activo` tinyint(1) NOT NULL DEFAULT 1,
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `nombre` (`nombre`),
  UNIQUE KEY `nombre_gefe` (`nombre_gefe`),
  KEY `idx_provincias_nombre` (`nombre`),
  KEY `idx_provincias_gefe` (`nombre_gefe`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `roles`
--

DROP TABLE IF EXISTS `roles`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `roles` (
  `id` tinyint(3) unsigned NOT NULL AUTO_INCREMENT,
  `nombre` varchar(50) NOT NULL,
  `descripcion` varchar(255) NOT NULL DEFAULT '',
  `nivel` tinyint(3) unsigned NOT NULL DEFAULT 0 COMMENT 'Jerarquía: 0=máximo (Federativo), mayor=número=menos privilegios',
  `activo` tinyint(1) NOT NULL DEFAULT 1,
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `nombre` (`nombre`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `torneos`
--

DROP TABLE IF EXISTS `torneos`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `torneos` (
  `id` int(11) NOT NULL,
  `fecha` varchar(10) NOT NULL,
  `hora` varchar(5) NOT NULL,
  `torneo` varchar(255) NOT NULL,
  `municipio` varchar(100) NOT NULL,
  `direccion` varchar(100) NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `user_logs`
--

DROP TABLE IF EXISTS `user_logs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `user_logs` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `user_id` int(10) unsigned DEFAULT NULL COMMENT 'NULL si es acción del sistema',
  `action` enum('LOGIN_SUCCESS','LOGIN_FAILED','LOGOUT','CREATE','UPDATE','DELETE','AUTH_GRANT','AUTH_REVOKE','PASSWORD_CHANGE','ACCOUNT_LOCK','ACCOUNT_UNLOCK') NOT NULL,
  `target_table` varchar(64) DEFAULT NULL COMMENT 'Tabla afectada (ej: jugadores, equipos)',
  `target_id` int(10) unsigned DEFAULT NULL COMMENT 'ID del registro afectado',
  `details` longtext CHARACTER SET utf8mb4 COLLATE utf8mb4_bin DEFAULT NULL COMMENT 'Detalles de los cambios en formato JSON' CHECK (json_valid(`details`)),
  `ip_address` varchar(45) DEFAULT NULL COMMENT 'Soporta IPv4 e IPv6',
  `user_agent` varchar(512) DEFAULT NULL,
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`id`),
  KEY `idx_logs_user` (`user_id`),
  KEY `idx_logs_action` (`action`),
  KEY `idx_logs_target` (`target_table`,`target_id`),
  KEY `idx_logs_created` (`created_at`),
  CONSTRAINT `fk_logs_user` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Table structure for table `users`
--

DROP TABLE IF EXISTS `users`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8mb4 */;
CREATE TABLE `users` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(50) NOT NULL,
  `password_hash` varchar(255) NOT NULL COMMENT 'Hash bcrypt/argon2',
  `email` varchar(255) DEFAULT NULL,
  `role_id` tinyint(3) unsigned NOT NULL,
  `club_id` int(10) unsigned DEFAULT NULL COMMENT 'Si rol=club, el club que gestiona',
  `equipo_id` int(10) unsigned DEFAULT NULL COMMENT 'Si rol=capitan, el equipo que gestiona',
  `jugador_id` int(10) unsigned DEFAULT NULL COMMENT 'Si rol=federado, su propia ficha',
  `activo` tinyint(1) NOT NULL DEFAULT 1,
  `ultimo_login` timestamp NULL DEFAULT NULL,
  `intentos_fallidos` tinyint(3) unsigned NOT NULL DEFAULT 0 COMMENT 'Para bloqueo tras N intentos',
  `bloqueado_hasta` timestamp NULL DEFAULT NULL,
  `creado_por` int(10) unsigned DEFAULT NULL COMMENT 'FK a users.id',
  `created_at` timestamp NOT NULL DEFAULT current_timestamp(),
  `updated_at` timestamp NOT NULL DEFAULT current_timestamp() ON UPDATE current_timestamp(),
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`),
  KEY `fk_users_equipo` (`equipo_id`),
  KEY `fk_users_jugador` (`jugador_id`),
  KEY `fk_users_creado` (`creado_por`),
  KEY `idx_users_username` (`username`),
  KEY `idx_users_role` (`role_id`),
  KEY `idx_users_club` (`club_id`),
  KEY `idx_users_activo` (`activo`),
  CONSTRAINT `fk_users_creado` FOREIGN KEY (`creado_por`) REFERENCES `users` (`id`) ON DELETE SET NULL,
  CONSTRAINT `fk_users_equipo` FOREIGN KEY (`equipo_id`) REFERENCES `equipos` (`id`) ON DELETE SET NULL,
  CONSTRAINT `fk_users_jugador` FOREIGN KEY (`jugador_id`) REFERENCES `jugadores` (`id`) ON DELETE SET NULL,
  CONSTRAINT `fk_users_role` FOREIGN KEY (`role_id`) REFERENCES `roles` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*M!100616 SET NOTE_VERBOSITY=@OLD_NOTE_VERBOSITY */;

-- Dump completed on 2026-06-24  8:31:44
