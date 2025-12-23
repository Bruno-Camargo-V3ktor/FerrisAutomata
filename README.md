# Automata (Reconhecedor de Linguagens Regulares)

> Trabalho proposto pelo Prof. José Rui Sousa para a disciplina de Linguagens Formais e Autômatos.

Este software implementa um sistema de reconhecimento de linguagens do Tipo 3 (Regulares). O programa é capaz de ler arquivos de configuração de autômatos e validar se uma lista de palavras pertence à linguagem definida.

O sistema suporta:
* **AFD** (Autômato Finito Determinístico)
* **AFN** (Autômato Finito Não-Determinístico)
* **AFNe** (Autômato Finito Não-Determinístico com movimentos vazios/epsilon)

## 🚀 Diferenciais de Performance

Para garantir eficiência, o sistema utiliza **Multi-threading** no processamento de autômatos não-determinísticos (AFN) e com movimentos vazios (AFNe). Quando o autômato se ramifica em múltiplos caminhos possíveis, o programa processa essas variantes em paralelo, acelerando significativamente o tempo de reconhecimento das palavras.

## 📦 Como Usar

O programa funciona através da linha de comando. Você deve passar o caminho do arquivo contendo a tabela de transição e, em seguida, a lista de palavras que deseja testar.

### Sintaxe
```bash
./automato <arquivo_tabela.csv> <palavra1> <palavra2> ...

```

### Exemplo

```bash
./automato afn.csv abba aa bbb

```

**Saída:** O programa imprimirá no terminal quais palavras foram **aceitas** e quais foram **rejeitadas**.

---

## 📄 Guia: Como Escrever a Tabela de Transição

O arquivo de entrada deve ser um arquivo de texto simples (sugerimos a extensão `.csv` ou `.txt`), onde cada linha representa uma parte da definição do autômato. O separador utilizado é o **ponto e vírgula (;)**.

### Regras de Formatação

1. **Cabeçalho (Alfabeto):**
A primeira linha define os símbolos aceitos pelo autômato. A primeira coluna é reservada para os estados (use `-`), e as colunas seguintes são os símbolos de entrada.
* Exemplo: `-;a;b` (Autômato lê 'a' e 'b').


2. **Estados e Transições:**
As linhas seguintes descrevem as transições.
* **Formato:** `estado_atual;destino_se_ler_a;destino_se_ler_b;...`
* **Sem transição:** Use `-` se não houver caminho para aquele símbolo.
* **Múltiplos caminhos (AFN):** Se um estado puder ir para vários lugares com o mesmo símbolo, separe os destinos por vírgula (ex: `q0,q1`).


3. **Estado Inicial:**
O programa considera automaticamente o **primeiro estado listado** na tabela (logo abaixo do cabeçalho) como o estado inicial.
4. **Estados Finais (Aceitação):**
Para definir que um estado é final, adicione dois pontos (`:`) antes do nome dele na primeira coluna.
* Exemplo: `:qf` indica que `qf` é um estado de aceitação.



### Exemplos Práticos

Na pasta raiz do projeto, existem três arquivos de exemplo. Abaixo, a explicação de como cada um é estruturado:

#### 1. AFD (Determinístico)

Um caminho único para cada entrada.

```text
-;a;b
q0;q1;q2;
q1;qf;q2;
q2;q1;qf;
:qf;qf;qf;

```

#### 2. AFN (Não-Determinístico)

Note o uso da vírgula na segunda linha (`q0,q1`), indicando que ao ler 'a', o autômato pode permanecer em `q0` OU ir para `q1`.

```text
-;a;b
q0;q0,q1;q0
q1;-;q2
q2;q2,q3;q2
q3;-;q4
q4;q4,q5;q4
q5;-;q6
:q6;q6;q6

```

#### 3. AFNe (Com movimentos vazios)

A última coluna representa o movimento vazio (epsilon). Se o autômato pode transitar sem ler entrada, preencha esta coluna.

```text
-;a;b;c
q0;q1;q4;q3;-
q1;q1,q2;q1;q1;q0
q2;q2,q7;q2;q2;-
q3;q3;q3;q3,q6;q0
q4;q4;q4,q5;q4;q0
q5;q5;q5,q7;q5;-
q6;q6;q6;q6,q7;-
:q7;q7;q7;q7;-

```

---

⚠️ Limitações Conhecidas e Melhorias Futuras

    Loops Infinitos com Movimentos Vazios (ϵ-loops): Atualmente, o sistema não possui uma detecção automática de ciclos fechado
    compostos exclusivamente por transições vazias. Caso a tabela de transição contenha um ciclo onde um estado retorna a si mesmo sem
    consumir nenhuma entrada (ex: A -> B -> A apenas com movimentos vazios), o programa entrará em loop infinito. Isso fará com que a
    execução nunca termine, possivelmente resultando em um travamento (crash) por estouro de pilha (stack overflow) ou consumo excessivo de memória.

---