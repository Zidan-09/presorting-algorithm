import { test } from "../entities/test.ts";

export function insertionSort(array: number[]): void {
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