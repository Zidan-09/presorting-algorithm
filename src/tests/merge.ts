import { presortAlgoritm } from "../presort/presorting.ts";
import { generateRandomArray } from "../utils/generateArray.ts";
import { test } from "../entities/test.ts";
import { sortAlgoritm } from "../utils/sortTypes.ts";

export function testMerge(showArray: boolean) {
  test.reset();
  const method = "MERGE";

  console.log(`\n${method}\n`);
  console.log("\n\n10.000 elementos\n\n");
  for (let i = 0; i < 5; i++) {
    const base = generateRandomArray(10000, 1, 1000000);

    if (showArray) console.log("Antes:", base);

    const test_1 = [...base];
    const test_2 = [...base];

    console.time("sem");
    sortAlgoritm[method](test_1);
    console.timeEnd("sem");

    console.log("Comparações:", test.comparacoes);
    console.log("Trocas:", test.trocas, "\n\n");

    test.reset();

    console.time("com");
    presortAlgoritm(test_2, method);
    console.timeEnd("com");

    console.log("Comparações:", test.comparacoes);
    console.log("Trocas:", test.trocas, "\n\n");

    if (showArray) console.log("Depois", test_2, "\n");
  }

  test.reset();

  console.log("\n\n30.000 elementos\n\n");
  for (let i = 0; i < 5; i++) {
    const base = generateRandomArray(30000, 1, 1000000);

    if (showArray) console.log("Antes:", base);

    const test_1 = [...base];
    const test_2 = [...base];

    console.time("sem");
    sortAlgoritm[method](test_1);
    console.timeEnd("sem");

    console.log("Comparações:", test.comparacoes);
    console.log("Trocas:", test.trocas, "\n\n");

    test.reset();

    console.time("com");
    presortAlgoritm(test_2, method);
    console.timeEnd("com");

    console.log("Comparações:", test.comparacoes);
    console.log("Trocas:", test.trocas, "\n\n");

    if (showArray) console.log("Depois", test_2, "\n");
  }

  test.reset();

  console.log("\n\n50.000 elementos\n\n");
  for (let i = 0; i < 5; i++) {
    const base = generateRandomArray(50000, 1, 1000000);

    if (showArray) console.log("Antes:", base);

    const test_1 = [...base];
    const test_2 = [...base];

    console.time("sem");
    sortAlgoritm[method](test_1);
    console.timeEnd("sem");

    console.log("Comparações:", test.comparacoes);
    console.log("Trocas:", test.trocas, "\n\n");

    test.reset();

    console.time("com");
    presortAlgoritm(test_2, method);
    console.timeEnd("com");

    console.log("Comparações:", test.comparacoes);
    console.log("Trocas:", test.trocas, "\n\n");

    if (showArray) console.log("Depois", test_2, "\n");
  }
}