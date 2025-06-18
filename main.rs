mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();

    blockchain.adiciona_block("Transação A".to_string());
    blockchain.adiciona_block("Transação B".to_string());

    for bloco in &blockchain.chain {
        bloco.exibir();
    }

    println!("A cadeia é válida? {}", blockchain.chain_valido());

}
