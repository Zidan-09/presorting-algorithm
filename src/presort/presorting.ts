import { test } from "../entities/test.ts";
import { sortAlgoritm, type SortTypes } from "../utils/sortTypes.ts";

function presortAlgoritm(array: number[], sortType: SortTypes): void {
  const last = array.length - 1;
  const mid = array.length >> 1;

  if (array[0] > array[last]) {
    [array[0], array[last]] = [array[last], array[0]];
    test.trocas++;
  }

  for (let i = 1; i < mid; i++) {
    test.comparacoes++;
    
    const j = last - i;

    if (array[i] > array[j]) {
      [array[i], array[j]] = [array[j], array[i]];
      test.trocas++;
    }

    if (array[i] < array[i - 1]) {
      test.comparacoes++;
      [array[i], array[i - 1]] = [array[i - 1], array[i]];
      test.trocas++;
    }
    
    if (array[j] > array[j + 1]) {
      test.comparacoes++;
      [array[j], array[j + 1]] = [array[j + 1], array[j]];
      test.trocas++;
    }
  }

  sortAlgoritm[sortType](array);
}

export { presortAlgoritm };