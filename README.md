# Simulação de Tráfego com Semáforos usando Threads

Projeto da disciplina **DCC 403 – Programação Concorrente com Threads**.

## Visão geral

Este projeto implementa uma simulação de tráfego em um cruzamento com semáforos, utilizando programação concorrente com threads e mecanismos de sincronização para garantir consistência no estado do sistema.

A solução foi desenvolvida em:

- C++
- Rust

## Objetivo

Demonstrar, na prática:

- criação de threads
- sincronização com mutex
- proteção de região crítica
- prevenção de race condition
- análise de deadlock, desempenho e desafios

## Descrição do problema

O cruzamento possui quatro direções:

- Norte
- Sul
- Leste
- Oeste

O tráfego é controlado por fases:

### Fase 1
- Norte e Sul: verde
- Leste e Oeste: vermelho

### Transição
- Norte e Sul: amarelo
- Leste e Oeste: vermelho

### Fase 2
- Norte e Sul: vermelho
- Leste e Oeste: verde

### Transição
- Norte e Sul: vermelho
- Leste e Oeste: amarelo

## Modelagem concorrente

A solução foi organizada da seguinte forma:

- cada semáforo é uma thread
- existe um estado compartilhado do cruzamento
- um controlador define a fase e a etapa atuais
- as threads atualizam seus estados com base nessas informações

## Sincronização

Como várias threads acessam e modificam o mesmo estado, foi necessário utilizar mecanismos de sincronização.

### Em C++
A implementação usa:

- `std::mutex`
- `std::condition_variable`

### Em Rust
A implementação usa:

- `Mutex`
- `Condvar`
- `Arc`

Esses mecanismos garantem exclusão mútua e sincronização correta entre as threads.

## Região crítica

A região crítica do sistema corresponde ao trecho em que as threads acessam ou alteram:

- a fase atual
- a etapa atual
- o estado dos semáforos
- o contador de atualizações

Sem proteção adequada, o sistema poderia apresentar inconsistência.

## Race condition

Sem sincronização, duas ou mais threads poderiam alterar o estado compartilhado ao mesmo tempo, causando uma condição de corrida.

Exemplo de erro possível sem mutex:

- Norte e Sul ficando verdes
- ao mesmo tempo Leste e Oeste também ficando verdes

A solução evita esse problema usando mutex e variáveis de condição.

## Estratégia adotada

Para tornar a simulação mais consistente, foi utilizada uma abordagem em que:

1. o controlador define a nova fase e etapa
2. todas as threads dos semáforos são notificadas
3. cada thread atualiza seu estado
4. o controlador espera até que todas terminem
5. só então o estado do cruzamento é exibido

Essa abordagem evita exibir estados parciais ou inconsistentes.

## Estrutura do projeto

```text
simulacao-semaforos-threads/
├── README.md
├── .gitignore
├── RELATORIO.md
├── cpp/
│   └── trafego.cpp
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── slides/
