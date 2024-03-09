--
-- Table structure for table `EloFada`
--

DROP TABLE IF EXISTS `EloFada`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!40101 SET character_set_client = utf8 */;
CREATE TABLE `EloFada` (
  `Id_Fada` int(5) NOT NULL,
  `Categoria` varchar(10) DEFAULT NULL,
  `Id_Fide` int(8) DEFAULT NULL,
  `Jugador` varchar(50) DEFAULT NULL,
  `Elo_Feda` int(4) DEFAULT NULL,
  `Elo_Fide` int(4) DEFAULT NULL,
  `Elo_Fada` varchar(100) DEFAULT NULL,
  `Elo_Fada_Old` int(4) DEFAULT NULL,
  `Id_Feda` int(5) DEFAULT NULL,
  `Titulo` varchar(4) DEFAULT NULL,
  `FNac` int(4) DEFAULT NULL,
  `Partidas_Totales` int(4) DEFAULT NULL,
  `Provincia` varchar(3) DEFAULT NULL,
  `Edad` int(3) DEFAULT NULL,
  `Sexo` char(1) DEFAULT NULL,
  `Estado` varchar(1) DEFAULT NULL,
  `Lista_Elo` varchar(14) DEFAULT NULL,
  `Club` varchar(40) DEFAULT NULL,
  `Federado` varchar(3) DEFAULT NULL,
  `Lista` int(3) DEFAULT NULL,
  `Torneos` int(3) DEFAULT NULL,
  `Partidas` int(3) DEFAULT NULL,
  `Z_Elo` int(4) DEFAULT NULL,
  `RC` int(4) DEFAULT NULL
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

