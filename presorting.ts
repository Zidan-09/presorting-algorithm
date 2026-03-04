import { test, swap } from "./utils.ts";
import { insertionSort, mergeSort, quickSort } from "./sortingAlgoritms.ts";

function presortAlgoritm(array: number[]): void {
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

  insertionSort(array);
}

export { presortAlgoritm };