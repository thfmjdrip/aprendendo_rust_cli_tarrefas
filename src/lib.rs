
pub enum ProcessamentoResultado<T>{
    Sucesso(T),
    Descartado,
    Erro(String),
}

pub struct Evento<T>{
    timestamp:u64,
    payload:T,


}

impl<T> Evento<T> {
    pub fn new(timestamp:u64,payload:T) -> Self{
        Self { timestamp, payload }
    }

    pub fn timestamp(&self) -> u64{
        self.timestamp

    }

    pub fn payload(&self) -> &T{
        &self.payload
    }

}

impl Evento<String> {
    pub fn muda_maiusculo(&mut self) {
        self.payload = self.payload.to_uppercase();
    }
}

impl Evento<f64> {
    pub fn payload_arredoando(&self) -> f64 {
        (self.payload * 100.0).round() / 100.0

    }
}

pub fn exbir_texto<T>(evnto: &Evento<T>) {
    println!("[{}] ",evnto.timestamp);
}
pub fn processar_evento<T>(evento: &Evento<T>) -> ProcessamentoResultado<u64> {
    if evento.timestamp == 0 {
        return ProcessamentoResultado::Erro(String::from("Timestamp error"))
    }
    ProcessamentoResultado::Sucesso(evento.timestamp)
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn evento_string_sendo_criado_corretamente(){
        let evento = Evento::new(100,String::from("Test"));
        assert_eq!(evento.timestamp,100);
    }
    
}




