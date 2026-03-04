function swap(idx1: number, idx2: number, array: number[]) {
  const temp = array[idx1];
  array[idx1] = array[idx2];
  array[idx2] = temp;
  test.trocas++;
}

function generateRandomArray(size: number, min: number = 0, max: number = 1000): number[] {
  return Array.from({ length: size }, () =>
    Math.floor(Math.random() * (max - min + 1)) + min
  );
}

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

const test = new Test();


export { swap, generateRandomArray, test };