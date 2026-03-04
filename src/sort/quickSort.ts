import { test } from "../entities/test.ts";
import { swap } from "../utils/swap.ts";

function quickSort(array: number[]): void;
function quickSort(array: number[], lowIdx: number, highIdx: number): void;

function quickSort(array: number[], lowIdx?: number, highIdx?: number): void {
  if (lowIdx === undefined || highIdx === undefined) {
    quickSort(array, 0, array.length - 1);
    return;
  }

  if (lowIdx >= highIdx) return;

  const pivotIdx = Math.floor(Math.random() * (highIdx - lowIdx + 1)) + lowIdx;
  const pivot = array[pivotIdx];

  const aux = array[highIdx];
  array[highIdx] = array[pivotIdx];
  array[pivotIdx] = aux;
  test.trocas++;

  let leftPointer = lowIdx;
  let rightPointer = highIdx;

  while (leftPointer < rightPointer) {
    while (leftPointer < rightPointer) {
      test.comparacoes++;
      if (array[leftPointer] <= pivot) {
        leftPointer++;
      } else {
        break;
      }
    }

    while (leftPointer < rightPointer) {
      test.comparacoes++;
      if (array[rightPointer] >= pivot) {
        rightPointer--;
      } else {
        break;
      }
    }

    if (leftPointer < rightPointer) {
      swap(leftPointer, rightPointer, array);
    }
  }

  swap(leftPointer, highIdx, array);

  quickSort(array, lowIdx, leftPointer - 1);
  quickSort(array, leftPointer + 1, highIdx);
}

export { quickSort }