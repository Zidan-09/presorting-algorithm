import { test } from "../entities/test.js";

export function selectionSort(array: number[]): void {
  for (let i = 0; i < array.length; i++) {
    let minorNumberIdx = i;

    for (let j = i + 1; j < array.length; j++) {
      if (array[j] < array[minorNumberIdx]) {
        minorNumberIdx = j;
      }

      test.comparacoes++;
    }

    if (minorNumberIdx !== i) {
      [array[minorNumberIdx], array[i]] = [array[i], array[minorNumberIdx]];
      test.trocas++;
    }
  }
}