import { test } from "./utils.ts";

function insertionSort(array: number[]): void {
  for (let i = 1; i < array.length; i++) {

    const value = array[i];
    let j = i - 1;

    while (j >= 0) {

      test.comparacoes++;

      if (array[j] > value) {
        array[j + 1] = array[j];
        test.trocas++;
        j--;
      } else {
        break;
      }
    }

    array[j + 1] = value;
  }
}

function mergeSort(array: number[]) {
  const arrayLength = array.length;

  if (arrayLength < 2) return;

  const middleIndex = Math.floor(arrayLength / 2);
  
  const leftHalf = array.slice(0, middleIndex);
  const rightHalf = array.slice(middleIndex, arrayLength);

  mergeSort(leftHalf);
  mergeSort(rightHalf);

  merge(array, leftHalf, rightHalf);

  return array;
}

function merge(array: number[], left: number[], right: number[]) {
  const leftSize = left.length;
  const rightSize = right.length;

  let l = 0; let r = 0; let s = 0;

  while (l < leftSize && r < rightSize) {
    test.comparacoes++;

    if (left[l] <= right[r]) {
      array[s] = left[l];
      l++;

    } else {
      array[s] = right[r];
      r++;
    }

    test.trocas++;
    s++;
  }

  while (l < leftSize) {
    array[s] = left[l];
    l++;
    s++;
    test.trocas++;
  }

  while (r < rightSize) {
    array[s] = right[r];
    r++;
    s++;
    test.trocas++;
  }
}

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
      const temp = array[leftPointer];
      array[leftPointer] = array[rightPointer];
      array[rightPointer] = temp;
      test.trocas++;
    }
  }

  const temp = array[leftPointer];
  array[leftPointer] = array[highIdx];
  array[highIdx] = temp;
  test.trocas++;

  quickSort(array, lowIdx, leftPointer - 1);
  quickSort(array, leftPointer + 1, highIdx);
}

export { insertionSort, mergeSort, quickSort };