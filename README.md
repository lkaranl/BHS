# BHS - Busca por Herança e Sobrescrita
**Um Simulador Educacional de Inteligência Artificial Simbólica**

---

## Resumo

O projeto **BHS (Busca por Herança e Sobrescrita)** é um ambiente de simulação e visualização desenvolvido na linguagem Rust. Seu objetivo central é fornecer uma ferramenta de apoio didático para o ensino de Inteligência Artificial Simbólica, com foco específico na representação de conhecimento através de **Redes Semânticas**. O simulador permite a observação em tempo real de algoritmos de busca processando inferências, lidando com herança de propriedades e resolvendo conflitos por meio de sobrescrita.

## Fundamentação Teórica

O sistema modela três conceitos fundamentais da representação do conhecimento:

1. **Redes Semânticas:** Estruturas de grafos direcionados onde os nós representam conceitos ou entidades do mundo real e as arestas representam relações semânticas (predominantemente a relação *is-a* ou "é-um").
2. **Herança (Inferência Hierárquica):** O mecanismo pelo qual um nó de nível inferior (específico) infere propriedades de seus nós ancestrais (genéricos). Isso simula o raciocínio dedutivo básico (ex: deduzir que um "Canário" voa porque é uma "Ave").
3. **Sobrescrita (Tratamento de Exceções):** A capacidade do sistema de interromper a busca hierárquica ao encontrar uma definição local que contradiz a regra geral. Este conceito é vital para modelar o raciocínio não-monotônico na IA (ex: definir que um "Morcego", embora seja um "Mamífero", possui a propriedade de voar).

## Pré-requisitos

Para compilar e executar o simulador, é necessário ter o ambiente de desenvolvimento da linguagem **Rust** configurado, o que inclui o compilador `rustc` e o gerenciador de pacotes `cargo`.

Além disso, como o projeto utiliza a biblioteca de renderização gráfica `macroquad`, dependências de sistema operacional para compilação gráfica podem ser necessárias (como ferramentas de build C++ no Windows/macOS, ou bibliotecas X11/Wayland e OpenGL no Linux).

## Instalação e Execução

Para configurar o projeto em sua máquina local para fins de estudo ou demonstração em sala de aula, siga os passos abaixo:

1. **Clone o repositório acadêmico:**
   ```bash
   git clone <URL_DO_REPOSITORIO>
   cd BHS
   ```

2. **Compilação e Execução:**
   Utilize o gerenciador de pacotes do Rust para compilar e iniciar o simulador simultaneamente:
   ```bash
   cargo run --release
   ```
   *Nota técnica: Recomenda-se o uso da flag `--release` para garantir a fluidez da renderização gráfica durante as simulações, embora a omissão desta flag seja útil durante o processo de depuração (`debug`).*

## Metodologia de Uso (Guia Didático)

O BHS foi projetado para ser um sistema de "código aberto e manipulável" durante as aulas. Professores e alunos são encorajados a interagir diretamente com o código-fonte (`src/main.rs`) para testar hipóteses:

* **Experimento de Modificação de Fatos:** Altere os valores booleanos das propriedades dos nós (ex: altere a propriedade de voo de uma ave não voadora) e observe como o algoritmo de busca altera seu caminho de inferência.
* **Experimento de Expansão de Ontologia:** Adicione novas entidades (ex: "Pinguim", "Ornitorrinco") definindo seus nós pai e propriedades específicas. Este exercício consolida o entendimento de taxonomia e ontologias em IA.
* **Análise de Complexidade de Busca:** Acompanhe visualmente o tempo (em etapas) que a IA leva para responder a perguntas dependendo da profundidade da árvore (ex: a diferença de passos para validar se um "Cão" respira versus se ele tem pelos).

## Tecnologias Empregadas

* **Linguagem:** Rust (foco em segurança de memória e performance previsível).
* **Renderização:** Macroquad (biblioteca focada em desenvolvimento ágil de aplicações gráficas bidimensionais).

## Referências Bibliográficas Sugeridas

Para aprofundamento nos temas demonstrados por este simulador, recomenda-se a seguinte leitura:
* RUSSELL, Stuart; NORVIG, Peter. *Inteligência Artificial: Uma Abordagem Moderna*. 3. ed. Elsevier, 2013. (Capítulo sobre Representação de Conhecimento e Raciocínio).