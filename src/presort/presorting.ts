import { test } from "../entities/test.js";
import { sortAlgorithm, type SortTypes } from "../utils/sortTypes.js";

function presortAlgorithm(array: number[], sortType: SortTypes): void {
  const last = array.length - 1;
  const mid = array.length >> 1;

  for (let i = 0; i < mid; i++) {
    const j = last - i;

    test.comparacoes += 3;

    if (array[i] > array[j]) {
      const t = array[i];
      array[i] = array[j];
      array[j] = t;
      test.trocas++;
    }

    if (i + 1 < mid && array[i] > array[i + 1]) {
      const t = array[i];
      array[i] = array[i + 1];
      array[i + 1] = t;
      test.trocas++;
    }

    if (j - 1 > mid && array[j] < array[j - 1]) {
      const t = array[j];
      array[j] = array[j - 1];
      array[j - 1] = t;
      test.trocas++;
    }
  }

  sortAlgorithm[sortType](array);
}

export { presortAlgorithm };