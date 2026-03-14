import { presortAlgoritm } from "../presort/presorting.ts";
import { generateRandomArray } from "../utils/generateArray.ts";
import { test } from "../entities/test.ts";
import { sortAlgoritm, type SortTypes } from "../utils/sortTypes.ts";

export function finalTest(method: SortTypes) {
  const sizes = [10000, 30000, 50000];

  const warmUpArray = generateRandomArray(1000, 1, 10000);
  sortAlgoritm[method]([...warmUpArray]);
  presortAlgoritm([...warmUpArray], method);
  test.reset();

  console.log(`--- Iniciando Testes para: ${method} ---\n`);

  sizes.forEach(size => {
    test.reset();
    const base = generateRandomArray(size, 1, 1000000);

    const test_1 = [...base];
    let start = performance.now();
    sortAlgoritm[method](test_1);
    let end = performance.now();

    console.log(`Sem pré-processamento com ${size} elementos: ${(end - start).toFixed(3)}ms | Comps: ${test.comparacoes} | Swaps: ${test.trocas}`);
    
    test.reset();

    const test_2 = [...base];
    start = performance.now();
    presortAlgoritm(test_2, method);
    end = performance.now();

    console.log(`Com pré-processamento com ${size} elementos: ${(end - start).toFixed(3)}ms | Comps: ${test.comparacoes} | Swaps: ${test.trocas}\n`);
    
    test.reset();
  });
}