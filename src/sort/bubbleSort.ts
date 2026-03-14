import { test } from "../entities/test.js";

export function bubbleSort(array: number[]): void {
  let swapped: boolean = true;

  while (swapped) {
    swapped = false;

    for (let i = 0; i < array.length - 1; i++) {
      if (array[i + 1] < array[i]) {
        const temp = array[i];
        array[i] = array[i + 1];
        array[i + 1] = temp;

        swapped = true;
        test.trocas++;
      }

      test.comparacoes++;
    }
  }
}