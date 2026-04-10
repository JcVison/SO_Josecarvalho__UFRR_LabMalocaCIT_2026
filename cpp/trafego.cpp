#include <chrono>
#include <condition_variable>
#include <iostream>
#include <mutex>
#include <string>
#include <thread>
#include <vector>

using namespace std;
using namespace std::chrono_literals;

enum class Cor { Vermelho, Amarelo, Verde };
enum class Fase { NorteSul, LesteOeste };
enum class Etapa { Verde, Amarelo };

struct Semaforo {
    string nome;
    Cor corAtual;
};

struct EstadoCompartilhado {
    Fase fase = Fase::NorteSul;
    Etapa etapa = Etapa::Verde;
    bool encerrado = false;
    unsigned long tick = 0;
    size_t atualizados = 0;
    vector<Semaforo> semaforos = {
        {"Norte", Cor::Vermelho},
        {"Sul", Cor::Vermelho},
        {"Leste", Cor::Vermelho},
        {"Oeste", Cor::Vermelho},
    };
};

mutex mtx;
condition_variable cvSemaforos;
condition_variable cvControlador;

string corParaTexto(Cor cor) {
    switch (cor) {
        case Cor::Verde:
            return "VERDE";
        case Cor::Amarelo:
            return "AMARELO";
        default:
            return "VERMELHO";
    }
}

bool pertenceAFase(const string& nome, Fase fase) {
    if (fase == Fase::NorteSul) {
        return nome == "Norte" || nome == "Sul";
    }
    return nome == "Leste" || nome == "Oeste";
}

void imprimirEstado(const EstadoCompartilhado& estado) {
    cout << "\n===== ESTADO DO CRUZAMENTO =====\n";
    for (const auto& s : estado.semaforos) {
        cout << s.nome << " -> " << corParaTexto(s.corAtual) << '\n';
    }
    cout << "Fase: " << (estado.fase == Fase::NorteSul ? "Norte/Sul" : "Leste/Oeste") << '\n';
    cout << "Etapa: " << (estado.etapa == Etapa::Verde ? "VERDE" : "AMARELO") << '\n';
    cout << "================================\n";
}

void threadSemaforo(size_t indice, EstadoCompartilhado& estado) {
    unsigned long ultimoTickProcessado = 0;

    unique_lock<mutex> lock(mtx);
    while (true) {
        cvSemaforos.wait(lock, [&]() {
            return estado.encerrado || estado.tick > ultimoTickProcessado;
        });

        if (estado.encerrado) {
            break;
        }

        const bool ativo = pertenceAFase(estado.semaforos[indice].nome, estado.fase);
        if (ativo) {
            estado.semaforos[indice].corAtual =
                (estado.etapa == Etapa::Verde) ? Cor::Verde : Cor::Amarelo;
        } else {
            estado.semaforos[indice].corAtual = Cor::Vermelho;
        }

        ultimoTickProcessado = estado.tick;
        estado.atualizados++;

        if (estado.atualizados == estado.semaforos.size()) {
            cvControlador.notify_one();
        }
    }
}

void aplicarEtapa(EstadoCompartilhado& estado, Fase fase, Etapa etapa) {
    unique_lock<mutex> lock(mtx);
    estado.fase = fase;
    estado.etapa = etapa;
    estado.atualizados = 0;
    estado.tick++;

    cvSemaforos.notify_all();
    cvControlador.wait(lock, [&]() {
        return estado.atualizados == estado.semaforos.size();
    });

    imprimirEstado(estado);
}

int main() {
    EstadoCompartilhado estado;
    vector<thread> threads;
    threads.reserve(estado.semaforos.size());

    for (size_t i = 0; i < estado.semaforos.size(); ++i) {
        threads.emplace_back(threadSemaforo, i, ref(estado));
    }

    const int ciclos = 3;
    for (int i = 0; i < ciclos; ++i) {
        aplicarEtapa(estado, Fase::NorteSul, Etapa::Verde);
        this_thread::sleep_for(2s);

        aplicarEtapa(estado, Fase::NorteSul, Etapa::Amarelo);
        this_thread::sleep_for(1s);

        aplicarEtapa(estado, Fase::LesteOeste, Etapa::Verde);
        this_thread::sleep_for(2s);

        aplicarEtapa(estado, Fase::LesteOeste, Etapa::Amarelo);
        this_thread::sleep_for(1s);
    }

    {
        lock_guard<mutex> lock(mtx);
        estado.encerrado = true;
    }
    cvSemaforos.notify_all();

    for (auto& t : threads) {
        t.join();
    }

    cout << "\nSimulacao encerrada com sucesso.\n";
    return 0;
}
