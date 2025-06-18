/* 
sha2::Sha256 = Algoritmo de hash criptográfico que gera uma saída única de 64 caracteres hexadecimais (256 bits)
a partir de dados de entrada arbitrários.
sha2::Digest = Resultado do processamento do SHA-256, representando o hash em formato bruto ou hexadecimal.
Chrono::prelude = traz para o escopo atual os principais tipos e traits da crate chrono
serde::Serialize = transforma o meu tipo de dado (a struct Block) em um item serializável (ou seja é convertido 
em um formato armazenável/transmissível)
serde::Deserialize = faz a reconstrução do meu tipo serializado
*/

use sha2::{Sha256, Digest};
use chrono::Utc;
use serde::{Serialize, Deserialize};

/*
Estrutura básica de um bloco na blockchain
index: posição do meu bloco
timestamp: hora exata que ele foi criado
dados: conteúdo (transações e mensagens)
hash_anterior: hash do bloco anterior, criando vínculo
hash: meu hash atual, garantindo integridade
*/

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: String,
    pub dados: String,
    pub hash_anterior: String,
    pub hash: String,
}

/*
Método new => cria um novo bloco (captura timestamp atual, 
gera o hash a partir dos campos relevantes, calcula SHA256, retorna o bloco com os campos preenchidos)
*/

impl Block {
    pub fn new(index: u32, dados: String, hash_anterior: String) -> Self {
        let timestamp = Utc::now().to_rfc3339();
        let conteudo = format!("{}{}{}{}", index, &timestamp, &dados, &hash_anterior);
        let mut hasher = Sha256::new();
        hasher.update(conteudo);
        let hash_resultante = format!("{:x}", hasher.finalize());

        Block {
            index,
            timestamp,
            dados,
            hash_anterior,
            hash: hash_resultante,
        }
    }

     pub fn exibir(&self) {
        println!("Bloco #{}", self.index);
        println!("Timestamp: {}", self.timestamp);
        println!("Dados: {}", self.dados);
        println!("Hash Anterior: {}", self.hash_anterior);
        println!("Hash: {}", self.hash);
        println!(); // linha em branco entre blocos
    }
}