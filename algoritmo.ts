import { presortAlgoritm } from "./presorting.ts";
import { generateRandomArray, test } from "./utils.ts";
import { insertionSort, mergeSort, quickSort } from "./sortingAlgoritms.ts";

console.log("\n\n10.000 elementos\n\n");
for (let i = 0; i < 5; i++) {
  const base = generateRandomArray(10000, 1, 1000000);

  const test_1 = [...base];
  const test_2 = [...base];

  console.time("sem");
  insertionSort(test_1);
  console.timeEnd("sem");

  console.log("Comparações:", test.comparacoes);
  console.log("Trocas:", test.trocas, "\n\n");

  test.reset();

  console.time("com");
  presortAlgoritm(test_2);
  console.timeEnd("com");

  console.log("Comparações:", test.comparacoes);
  console.log("Trocas:", test.trocas, "\n\n");
}

test.reset();

console.log("\n\n30.000 elementos\n\n");
for (let i = 0; i < 5; i++) {
  const base = generateRandomArray(30000, 1, 1000000);

  const test_1 = [...base];
  const test_2 = [...base];

  console.time("sem");
  insertionSort(test_1);
  console.timeEnd("sem");

  console.log("Comparações:", test.comparacoes);
  console.log("Trocas:", test.trocas, "\n\n");

  test.reset();

  console.time("com");
  presortAlgoritm(test_2);
  console.timeEnd("com");

  console.log("Comparações:", test.comparacoes);
  console.log("Trocas:", test.trocas, "\n\n");
}

test.reset();

console.log("\n\n50.000 elementos\n\n");
for (let i = 0; i < 5; i++) {
  const base = generateRandomArray(50000, 1, 1000000);

  const test_1 = [...base];
  const test_2 = [...base];

  console.time("sem");
  insertionSort(test_1);
  console.timeEnd("sem");

  console.log("Comparações:", test.comparacoes);
  console.log("Trocas:", test.trocas, "\n\n");

  test.reset();

  console.time("com");
  presortAlgoritm(test_2);
  console.timeEnd("com");

  console.log("Comparações:", test.comparacoes);
  console.log("Trocas:", test.trocas, "\n\n");
}