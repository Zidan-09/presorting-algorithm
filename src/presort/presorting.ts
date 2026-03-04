import { swap } from "../utils/swap.ts";
import { test } from "../entities/test.ts";
import { sortAlgoritm, type SortTypes } from "../utils/sortTypes.ts";

function presortAlgoritm(array: number[], sortType: SortTypes): void {
  let leftPointer = 0;
  let rightPointer = array.length - 1;

  while (leftPointer < rightPointer) {

    test.comparacoes++;

    if (array[leftPointer] > array[rightPointer]) {
      swap(leftPointer, rightPointer, array);
    }

    leftPointer++;
    rightPointer--;
  }

  sortAlgoritm[sortType](array);
}

export { presortAlgoritm };