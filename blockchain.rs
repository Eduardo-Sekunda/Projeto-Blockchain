use crate::block::Block;
use sha2::Digest;

//Estrutura que armazena todos os blocos
pub struct Blockchain {
    pub chain: Vec<Block>,
}

/*
Primeiramente, inicializamos a blockchain vazia e criamos o bloco gênese, que possui índice 0 e hash anterior "0".

Criamos o método add_block, que obtém o último hash da cadeia, calcula o novo índice com base no tamanho atual,
usa os dados passados para criar o novo bloco e, por fim, insere esse bloco no final da cadeia.

O método is_chain_valid percorre a cadeia a partir do segundo bloco (afinal, o gênese sempre é válido), verifica
se o hash_anterior de cada bloco bate com o hash do bloco anterior, recalcula o hash atual e compara com o armazenado,
retornando false caso alguma inconsistência seja encontrada.
*/
impl Blockchain {
    pub fn new() -> Self {
        let mut blockchain = Blockchain {chain: Vec::new()};
        let genesis_block = Block::new(0, "Bloco Gênese".to_string(), "0".to_string());
        blockchain.chain.push(genesis_block);
        blockchain
    }

    pub fn adiciona_block(&mut self, dados: String) {
        let ultimo_hash = self.chain.last().unwrap().hash.clone();
        let index = self.chain.len() as u32;
        let novo_bloco = Block::new(index, dados, ultimo_hash);
        self.chain.push(novo_bloco);
    }

    pub fn chain_valido(&self) -> bool {
        for i in 1..self.chain.len() {
            let bloco_atual = &self.chain[i];
            let bloco_anterior = &self.chain[i - 1];

            if bloco_atual.hash_anterior != bloco_anterior.hash {
                return false;
            }

            let conteudo = format!("{}{}{}{}", bloco_atual.index, bloco_atual.timestamp, 
            bloco_atual.dados, bloco_atual.hash_anterior);
            let mut hasher = sha2::Sha256::new();
            hasher.update(conteudo);
            let recalculado_hash = format!("{:x}", hasher.finalize());

            if bloco_atual.hash != recalculado_hash {
                return false;
            }
        }
        true
    }
}