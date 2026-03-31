# Explicação Técnica: Busca em Redes Semânticas com Rust

Este projeto é um simulador educacional de **Inteligência Artificial Simbólica**. Ele demonstra como sistemas de IA baseados em conhecimento processam relações hierárquicas usando **Redes Semânticas**.

---

## 1. O que é uma Rede Semântica?
Uma rede semântica é um grafo (conjunto de nós e conexões) que representa conhecimento. No nosso código:
- **Nós**: Representam entidades (Vida, Ave, Mamífero, Canário, etc.).
- **Arestas (is-a)**: Definem "é um". Exemplo: *O Canário é um (is-a) Ave.*

## 2. Conceitos de IA Implementados

### A. Herança (Inheritance)
É a capacidade de um nó filho obter características dos seus ancestrais.
- **No Código**: O `Canário` não tem a propriedade `voa`. Quando buscamos, o algoritmo percebe que ele não tem e sobe para o pai (`Ave`). Lá ele encontra `voa: true`.
- **Analogia**: "Se meu pai é um pássaro e pássaros voam, eu provavelmente voo também."

### B. Sobrescrita (Overriding/Exceptions)
É quando uma regra geral é quebrada por um caso específico.
- **No Código**: O `Morcego` é um `Mamífero`. A regra geral de mamíferos é `voa: false`. Porém, o nó `Morcego` possui `voa: true` definido localmente. O algoritmo para a busca assim que encontra o valor local, ignorando o do pai.
- **Importância**: Isso permite que a IA trate exceções em regras gerais.

### C. Algoritmo de Busca (Pathfinding)
O coração da lógica está dentro da função `select_lesson`. O algoritmo funciona assim:
1. Inicia no **Nó Alvo** (ex: Morcego).
2. Verifica se a propriedade existe **neste nó**.
3. Se **NÃO** existir, move para o **Pai** e repete o passo 2.
4. Se **EXISTIR**, para a busca e retorna o valor (Sucesso).
5. Se chegar no **Nó Raiz** (Vida) e não encontrar nada, a propriedade é desconhecida.

---

## 3. Estrutura do Código Rust

### Os Dados (`struct Node`)
```rust
struct Node {
    pos: Vec2,                          // Posição visual na tela
    properties: Vec<(&'static str, bool)>, // Mapa de características (voa: S, pelos: S)
    parent: Option<String>,              // Link para o nó superior
}
```

### O Loop de Animação (`update`)
A cada frame, o Rust verifica um `timer`. Quando o timer atinge o `SEARCH_SPEED`, o índice do nó atual no caminho (`path`) avança. Isso cria o efeito visual de "pular" entre os nós enquanto a IA "pensa".

### A Visualização (`draw`)
Usamos a biblioteca **Macroquad** para desenhar formas simples:
- `draw_circle`: Para os nós.
- `draw_line`: Para as arestas de herança.
- `measure_text`: Essencial para garantir que as legendas não fiquem tortas ou sobrepostas.

---

## 4. Dicas para os Alunos
- **Tente Mudar os Valores**: No arquivo `main.rs`, mude o `voa: false` do Avestruz para `true` e veja a IA mudar o comportamento.
- **Novos Nós**: Adicione um nó "Pinguim" herdando de "Ave" com `voa: false`.
- **Busca Distante**: Observe como o "Rex" (Cão) precisa subir dois níveis até chegar em "Vida" para descobrir que ele respira. Isso mostra como o conhecimento pode ser encadeado.

---
*Este material foi criado para apoiar o ensino de Lógica e IA em Rust.*
