import { test } from "../entities/test.js";

export function mergeSort(array: number[]) {
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