class Test {
  comparacoes: number;
  trocas: number;

  constructor() {
    this.comparacoes = 0;
    this.trocas = 0;
  }

  reset() {
    this.comparacoes = 0;
    this.trocas = 0;
  }
}

export const test = new Test();