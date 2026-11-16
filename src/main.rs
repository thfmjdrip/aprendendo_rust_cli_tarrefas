use projeto_generics::{self, Evento, exbir,processar_evento,ProcessamentoResultado};



fn main() {
    let log:Evento<String> = Evento { timestamp: (12345), payload: (String::from("Servidor iniciado")) };
    let metrica:Evento<f64> = Evento{ timestamp: (242525), payload: (99.9)};
    let status:Evento<bool> = Evento  { timestamp: (2323242), payload: (true)};
    let mut log_new = Evento ::new((12323), (String::from("Servidor iniciado")));
    let status_new = Evento::new((1232323), (true));
    exbir(&log);
    exbir(&metrica);
    log_new.muda_maiusculo();
    println!("Payload = {}",log_new.payload);

    match  processar_evento(&log){
        ProcessamentoResultado::Sucesso(ts) => {
            println!("Processado com sucesso : {}",ts);
        }
        ProcessamentoResultado::Descartado => {
            println!("Evento descartado")
        }
        ProcessamentoResultado::Erro(er) => {
            println!("Error : {}",er)
        }

    }

}
