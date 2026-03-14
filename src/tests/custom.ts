import { presortAlgoritm } from "../presort/presorting.js";
import { test } from "../entities/test.js";
import type { SortTypes } from "../utils/sortTypes.js";
import { sortAlgoritm } from "../utils/sortTypes.js";
import { generateTestArray } from "../utils/generateArray.js";

export function testCustom(sortType: SortTypes) {
  const sizes = [10000, 30000, 50000];

  let start: number;
  let end: number;

  const titles = Object.keys(generateTestArray(5));

  console.log(`--- Iniciando Testes com arrays específicos ---`);

  sizes.forEach(size => {
    test.reset();
    
    const arrays = Object.values(generateTestArray(size));

    for (let i = 0; i < arrays.length; i++) {
      const array = arrays[i];
      console.log(`\n${titles[i]}\n`);

      const test_1 = [...array];
      const test_2 = [...array];

      start = performance.now();
      sortAlgoritm[sortType](test_1);
      end = performance.now();

      console.log(`Sem pré-processamento com ${size} elementos: ${(end - start).toFixed(3)}ms | Comps: ${test.comparacoes} | Swaps: ${test.trocas}`);

      test.reset();

      start = performance.now();
      presortAlgoritm(test_2, sortType);
      end = performance.now();

      console.log(`Com pré-processamento com ${size} elementos: ${(end - start).toFixed(3)}ms | Comps: ${test.comparacoes} | Swaps: ${test.trocas}\n`);

      test.reset();
    }

    console.log("----------");
  });
}