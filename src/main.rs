use projeto_generics;

enum ProcessamentoResultado<T>{
    Sucesso(T),
    Descartado,
    Erro(String),
}

struct Evento<T>{
    timestamp:u64,
    payload:T,


}

impl<T> Evento<T> {
    fn new(timestamp:u64,payload:T) -> Self{
        Self { timestamp, payload }
    }

    fn timestamp(&self) -> u64{
        self.timestamp

    }

    fn payload(&self) -> &T{
        &self.payload
    }

}

impl Evento<String> {
    fn muda_maiusculo(&mut self) {
        self.payload = self.payload.to_uppercase();
    }
}

impl Evento<f64> {
    fn payload_arredoando(&self) -> f64 {
        (self.payload * 100.0).round() / 100.0

    }
}

fn exbir_texto<T>(evnto: &Evento<T>) {
    println!("[{}] ",evnto.timestamp);
}
fn processar_evento<T>(evento: &Evento<T>) -> ProcessamentoResultado<u64> {
    if evento.timestamp == 0 {
        return ProcessamentoResultado::Erro(String::from("Timestamp error"))
    }
    ProcessamentoResultado::Sucesso(evento.timestamp)
}

fn main() {
    let log:Evento<String> = Evento { timestamp: (12345), payload: (String::from("Servidor iniciado")) };
    let metrica:Evento<f64> = Evento { timestamp: (242525), payload: (99.9)};
    let status:Evento<bool> = Evento { timestamp: (2323242), payload: (true)};
    let mut log_new = Evento::new((12323), (String::from("Servidor iniciado")));
    let status_new = Evento::new((1232323), (true));
    exbir_texto(&log);
    exbir_texto(&metrica);
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
