# Relatório – Simulação de Tráfego com Semáforos usando Threads

**Disciplina:** DCC 403 – Programação Concorrente com Threads  
**Aluno:** José Carvalho Neto 2023010235
**Ano:** 2026  

## 1. Introdução

A programação concorrente é utilizada quando múltiplas tarefas precisam ser executadas de forma simultânea ou sobreposta, compartilhando recursos e exigindo coordenação. Um exemplo clássico de aplicação é o controle de tráfego em cruzamentos, onde diferentes semáforos devem operar de forma sincronizada para evitar conflitos entre vias.

Neste trabalho, foi desenvolvida uma simulação de tráfego com semáforos utilizando threads, com implementação em C++ e Rust. A proposta foi modelar um cruzamento com quatro direções e aplicar mecanismos de sincronização para garantir consistência no estado compartilhado do sistema.

## 2. Objetivo

O objetivo do trabalho é desenvolver uma solução concorrente que simule o funcionamento de um cruzamento com semáforos, demonstrando o uso de threads, sincronização com mutex e controle de região crítica, além de discutir desafios como condição de corrida, deadlock e desempenho.

## 3. Descrição do problema

O problema consiste em um cruzamento com quatro direções:

- Norte
- Sul
- Leste
- Oeste

Cada direção possui um semáforo. O sistema funciona em fases para impedir que vias conflitantes fiquem abertas ao mesmo tempo.

### Fase 1
- Norte e Sul ficam verdes
- Leste e Oeste ficam vermelhos

### Transição da Fase 1
- Norte e Sul passam para amarelo
- Leste e Oeste permanecem vermelhos

### Fase 2
- Norte e Sul ficam vermelhos
- Leste e Oeste ficam verdes

### Transição da Fase 2
- Norte e Sul permanecem vermelhos
- Leste e Oeste passam para amarelo

Esse ciclo se repete durante a simulação.

## 4. Lógica de concorrência utilizada

A lógica de concorrência foi baseada na ideia de que cada semáforo é representado por uma thread independente. Além disso, existe um controlador responsável por definir a fase ativa e a etapa atual da simulação.

As threads acessam um estado compartilhado que contém:

- fase atual do cruzamento
- etapa atual
- estado de cada semáforo
- contador de atualizações
- informação de encerramento da simulação

Como múltiplas threads acessam os mesmos dados, foi necessário utilizar sincronização para garantir exclusão mútua e consistência.

## 5. Sincronização e região crítica

A região crítica do sistema corresponde às partes do código em que as threads acessam ou modificam o estado compartilhado do cruzamento. Para proteger essa região, foram usados mecanismos de sincronização.

### Em C++
Foram utilizados:
- `std::mutex`
- `std::condition_variable`

### Em Rust
Foram utilizados:
- `Mutex`
- `Condvar`
- `Arc`

O mutex garante que apenas uma thread por vez acesse a região crítica, enquanto as variáveis de condição são usadas para coordenar o momento em que as threads devem atualizar seus estados.

## 6. Implementação em C++

A implementação em C++ utiliza threads para representar os semáforos e uma lógica de sincronização baseada em `mutex` e `condition_variable`.

Cada thread de semáforo:
- espera o sinal do controlador
- verifica a fase atual
- atualiza sua cor
- informa que terminou sua atualização

O controlador:
- define a nova fase e etapa
- notifica todas as threads
- espera todas concluírem
- imprime o estado atualizado do cruzamento

## 7. Implementação em Rust

A implementação em Rust segue a mesma lógica da versão em C++, mas utiliza as abstrações próprias da linguagem:

- `thread`
- `Arc`
- `Mutex`
- `Condvar`

O uso de Rust torna o compartilhamento de dados entre threads mais rigoroso e seguro, reduzindo a chance de erros de concorrência.

## 8. Condição de corrida

A condição de corrida ocorre quando duas ou mais threads acessam e modificam o mesmo dado ao mesmo tempo, sem sincronização adequada.

Neste problema, sem proteção correta, poderia acontecer de:
- Norte e Sul ficarem verdes
- ao mesmo tempo Leste e Oeste também ficarem verdes

Isso criaria um estado inconsistente no cruzamento. O uso de mutex e variáveis de condição evita esse problema.

## 9. Deadlock

Deadlock é uma situação em que duas ou mais threads ficam bloqueadas indefinidamente esperando recursos umas das outras.

Nesta solução, o risco de deadlock foi reduzido porque:
- existe um único mutex principal
- o fluxo de espera foi bem definido
- a comunicação entre controlador e threads segue uma ordem previsível

## 10. Desempenho

A sincronização garante segurança, mas também traz custo. Sempre que uma thread precisa esperar o mutex ou uma variável de condição, há sobrecarga de coordenação.

Portanto, a concorrência não significa necessariamente maior velocidade, mas sim uma forma adequada de modelar problemas em que múltiplas tarefas compartilham recursos.

## 11. Conclusão

O trabalho demonstrou a aplicação prática de programação concorrente com threads em um problema de controle de tráfego. A simulação foi implementada em C++ e Rust, atendendo ao enunciado proposto. O uso de mutex e variáveis de condição permitiu sincronização correta, evitando condição de corrida e tornando o sistema mais consistente.

Além disso, o projeto permitiu discutir desafios importantes da concorrência, como deadlock, desempenho e clareza na organização da lógica entre múltiplas threads.

## 12. Estrutura do projeto

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
