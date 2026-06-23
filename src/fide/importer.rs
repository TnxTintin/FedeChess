use sqlx::MySqlPool;
use crate::error::AppResult;
use super::parser::FidePlayer;

pub struct FideImporter<'a> {
    pool: &'a MySqlPool,
}

impl<'a> FideImporter<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }

    /// Importa una lista de jugadores FIDE a la BBDD
    /// Usa INSERT ... ON DUPLICATE KEY UPDATE para actualizar existentes
    pub async fn import(&self, players: &[FidePlayer]) -> AppResult<usize> {
        let mut imported = 0;
        
        // Procesar en lotes de 100 para mejor rendimiento
        for chunk in players.chunks(100) {
            let mut tx = self.pool.begin().await?;
            
            for player in chunk {
                sqlx::query(
                    r#"
                    INSERT INTO fide_players (
                        id_fide, nombre, federacion, sexo, titulo, titulo_femenino,
                        titulo_arbitro, elo_standard, partidas_standard, k_standard,
                        elo_rapid, partidas_rapid, k_rapid,
                        elo_blitz, partidas_blitz, k_blitz,
                        anio_nacimiento, flag, fecha_actualizacion
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, CURDATE())
                    ON DUPLICATE KEY UPDATE
                        nombre = VALUES(nombre),
                        federacion = VALUES(federacion),
                        sexo = VALUES(sexo),
                        titulo = VALUES(titulo),
                        titulo_femenino = VALUES(titulo_femenino),
                        titulo_arbitro = VALUES(titulo_arbitro),
                        elo_standard = VALUES(elo_standard),
                        partidas_standard = VALUES(partidas_standard),
                        k_standard = VALUES(k_standard),
                        elo_rapid = VALUES(elo_rapid),
                        partidas_rapid = VALUES(partidas_rapid),
                        k_rapid = VALUES(k_rapid),
                        elo_blitz = VALUES(elo_blitz),
                        partidas_blitz = VALUES(partidas_blitz),
                        k_blitz = VALUES(k_blitz),
                        anio_nacimiento = VALUES(anio_nacimiento),
                        flag = VALUES(flag),
                        fecha_actualizacion = CURDATE()
                    "#
                )
                .bind(player.id_fide)
                .bind(&player.nombre)
                .bind(&player.federacion)
                .bind(&player.sexo)
                .bind(&player.titulo)
                .bind(&player.titulo_femenino)
                .bind(&player.titulo_arbitro)
                .bind(player.elo_standard)
                .bind(player.partidas_standard)
                .bind(player.k_standard)
                .bind(player.elo_rapid)
                .bind(player.partidas_rapid)
                .bind(player.k_rapid)
                .bind(player.elo_blitz)
                .bind(player.partidas_blitz)
                .bind(player.k_blitz)
                .bind(player.anio_nacimiento)
                .bind(&player.flag)
                .execute(&mut *tx)
                .await?;
                
                imported += 1;
            }
            
            tx.commit().await?;
            tracing::info!("Importados {} jugadores", imported);
        }
        
        Ok(imported)
    }

    /// Sincroniza Elos de FIDE con la tabla federados
    /// Actualiza solo los federados que tienen id_fide y cuyo elo FIDE es mayor
    pub async fn sync_elos(&self) -> AppResult<usize> {
        let result = sqlx::query(
            r#"
            UPDATE federados f
            INNER JOIN fide_players fp ON f.id_fide = fp.id_fide
            SET 
                f.elo_standard = CASE WHEN fp.elo_standard > 0 THEN fp.elo_standard ELSE f.elo_standard END,
                f.elo_rapid = CASE WHEN fp.elo_rapid > 0 THEN fp.elo_rapid ELSE f.elo_rapid END,
                f.elo_blitz = CASE WHEN fp.elo_blitz > 0 THEN fp.elo_blitz ELSE f.elo_blitz END,
                f.titulo_fide = CASE 
                    WHEN fp.titulo != '' THEN fp.titulo 
                    WHEN fp.titulo_femenino != '' THEN fp.titulo_femenino
                    ELSE f.titulo_fide 
                END
            WHERE f.id_fide IS NOT NULL
              AND (
                  (fp.elo_standard > 0 AND fp.elo_standard != f.elo_standard) OR
                  (fp.elo_rapid > 0 AND fp.elo_rapid != f.elo_rapid) OR
                  (fp.elo_blitz > 0 AND fp.elo_blitz != f.elo_blitz)
              )
            "#
        )
        .execute(self.pool)
        .await?;
        
        Ok(result.rows_affected() as usize)
    }
}
