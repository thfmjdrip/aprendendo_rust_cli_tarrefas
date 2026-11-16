use core::fmt;
use std::fmt::Display;


pub enum ProcessamentoResultado<T>{
    Sucesso(T),
    Descartado,
    Erro(String),
}

trait Processavel{
    fn processar(&self);
    fn nome_do_tipo(&self) -> &str;
    fn resumo(&self) -> String {
        format!("[{}] processado",self.nome_do_tipo())
    }

}

struct LogPayLoad{
    msg:String,
    nivel:String,
}

struct MetricaPayLoad{
    nome:String,
    valor:f64,
}
struct AlertaPayLoad{
    msg:String,
    severidade:u8,
}

impl Processavel for LogPayLoad {
    fn processar(&self) {
        println!("[LOG] [{}] {}",self.nivel,self.msg);
    }

    fn nome_do_tipo(&self) -> &str {
        "LOG"
    }

    
}

impl Processavel for MetricaPayLoad {
    fn processar(&self) {
        println!("[METRICA] [{}] {}",self.nome,self.valor);
    }

    fn nome_do_tipo(&self) -> &str {
       "METRICA"
    }

    
}

impl Processavel for AlertaPayLoad {
    fn processar(&self) {
        let icone = if self.severidade >= 5 {"[🔴}"} else{"[🟡]"};
        println!("[ALERTA] [{}] {}",icone,self.msg);
    }

    fn nome_do_tipo(&self) -> &str {
        "ALERTA"
    }

    fn resumo(&self) -> String {
        format!("[{}] {}",self.severidade, self.msg)
    }
}


impl <T:Processavel> Evento<T>{ // so pode cair no T se o T implementar o tipo [Processavel]
    pub fn processar_evento(self){
        println!("----- Evento em {} -----",self.timestamp);
        self.payload.processar();
        println!("Resumo: {}",self.payload.resumo());
    }

}



pub struct Evento<T>{
    pub timestamp:u64,
    pub payload:T,
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

impl std::fmt::Display for LogPayLoad {
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"[{}] {}",self.nivel,self.msg)
    }
}


pub fn exbir<T:Display>(evnto: &Evento<T>) {
    println!("[{}] PayLoad:{}",evnto.timestamp,evnto.payload);
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

    #[test]
    fn playload_processavel(){
        let evento = Evento::new(132141414, LogPayLoad{
            msg:"Servidor iniciado".to_string(),
            nivel:"INFO".to_string()
        });
        evento.processar_evento();
    }
    
}




