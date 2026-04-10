use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
enum Cor {
    Vermelho,
    Amarelo,
    Verde,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Fase {
    NorteSul,
    LesteOeste,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Etapa {
    Verde,
    Amarelo,
}

#[derive(Clone, Debug)]
struct Semaforo {
    nome: String,
    cor_atual: Cor,
}

#[derive(Debug)]
struct Estado {
    fase: Fase,
    etapa: Etapa,
    encerrado: bool,
    tick: u64,
    atualizados: usize,
    semaforos: Vec<Semaforo>,
}

fn cor_para_texto(cor: Cor) -> &'static str {
    match cor {
        Cor::Verde => "VERDE",
        Cor::Amarelo => "AMARELO",
        Cor::Vermelho => "VERMELHO",
    }
}

fn pertence_a_fase(nome: &str, fase: Fase) -> bool {
    match fase {
        Fase::NorteSul => nome == "Norte" || nome == "Sul",
        Fase::LesteOeste => nome == "Leste" || nome == "Oeste",
    }
}

fn imprimir_estado(estado: &Estado) {
    println!("\n===== ESTADO DO CRUZAMENTO =====");
    for s in &estado.semaforos {
        println!("{} -> {}", s.nome, cor_para_texto(s.cor_atual));
    }
    println!(
        "Fase: {}",
        match estado.fase {
            Fase::NorteSul => "Norte/Sul",
            Fase::LesteOeste => "Leste/Oeste",
        }
    );
    println!(
        "Etapa: {}",
        match estado.etapa {
            Etapa::Verde => "VERDE",
            Etapa::Amarelo => "AMARELO",
        }
    );
    println!("================================");
}

fn main() {
    let estado = Arc::new((
        Mutex::new(Estado {
            fase: Fase::NorteSul,
            etapa: Etapa::Verde,
            encerrado: false,
            tick: 0,
            atualizados: 0,
            semaforos: vec![
                Semaforo {
                    nome: "Norte".to_string(),
                    cor_atual: Cor::Vermelho,
                },
                Semaforo {
                    nome: "Sul".to_string(),
                    cor_atual: Cor::Vermelho,
                },
                Semaforo {
                    nome: "Leste".to_string(),
                    cor_atual: Cor::Vermelho,
                },
                Semaforo {
                    nome: "Oeste".to_string(),
                    cor_atual: Cor::Vermelho,
                },
            ],
        }),
        Condvar::new(),
        Condvar::new(),
    ));

    let total_semaforos = {
        let (mutex_estado, _, _) = &*estado;
        let guard = mutex_estado.lock().unwrap();
        guard.semaforos.len()
    };

    let mut handles = Vec::new();

    for indice in 0..total_semaforos {
        let estado_clone = Arc::clone(&estado);
        let handle = thread::spawn(move || {
            let (mutex_estado, cv_semaforos, cv_controlador) = &*estado_clone;
            let mut ultimo_tick_processado = 0_u64;

            loop {
                let mut guard = mutex_estado.lock().unwrap();
                while !guard.encerrado && guard.tick <= ultimo_tick_processado {
                    guard = cv_semaforos.wait(guard).unwrap();
                }

                if guard.encerrado {
                    break;
                }

                let nome = guard.semaforos[indice].nome.clone();
                let cor = if pertence_a_fase(&nome, guard.fase) {
                    match guard.etapa {
                        Etapa::Verde => Cor::Verde,
                        Etapa::Amarelo => Cor::Amarelo,
                    }
                } else {
                    Cor::Vermelho
                };

                guard.semaforos[indice].cor_atual = cor;
                ultimo_tick_processado = guard.tick;
                guard.atualizados += 1;

                if guard.atualizados == guard.semaforos.len() {
                    cv_controlador.notify_one();
                }
            }
        });
        handles.push(handle);
    }

    let aplicar_etapa = |estado_comp: &Arc<(Mutex<Estado>, Condvar, Condvar)>,
                         fase: Fase,
                         etapa: Etapa| {
        let (mutex_estado, cv_semaforos, cv_controlador) = &**estado_comp;
        let mut guard = mutex_estado.lock().unwrap();
        guard.fase = fase;
        guard.etapa = etapa;
        guard.atualizados = 0;
        guard.tick += 1;

        cv_semaforos.notify_all();

        while guard.atualizados < guard.semaforos.len() {
            guard = cv_controlador.wait(guard).unwrap();
        }

        imprimir_estado(&guard);
    };

    let ciclos = 3;
    for _ in 0..ciclos {
        aplicar_etapa(&estado, Fase::NorteSul, Etapa::Verde);
        thread::sleep(Duration::from_secs(2));

        aplicar_etapa(&estado, Fase::NorteSul, Etapa::Amarelo);
        thread::sleep(Duration::from_secs(1));

        aplicar_etapa(&estado, Fase::LesteOeste, Etapa::Verde);
        thread::sleep(Duration::from_secs(2));

        aplicar_etapa(&estado, Fase::LesteOeste, Etapa::Amarelo);
        thread::sleep(Duration::from_secs(1));
    }

    {
        let (mutex_estado, cv_semaforos, _) = &*estado;
        let mut guard = mutex_estado.lock().unwrap();
        guard.encerrado = true;
        cv_semaforos.notify_all();
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("\nSimulacao encerrada com sucesso.");
}
