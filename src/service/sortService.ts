import { presortAlgorithm } from "../presort/presorting.js";
import { generateTestArray } from "../utils/generateArray.js";
import { test } from "../entities/test.js";
import { ArrayTypes, sortAlgorithm, type SortTypes } from "../utils/sortTypes.js";
import { logResult } from "../utils/resultLogger.js";

export function sortService(method: SortTypes, arrayType: ArrayTypes, size: number) {

  const warmUpArray = generateTestArray(1000, arrayType);
  sortAlgorithm[method]([...warmUpArray]);
  presortAlgorithm([...warmUpArray], method);

  console.log(`--- Iniciando Testes para: ${method} ---\n`);

  test.reset();
  const base = generateTestArray(size, arrayType);

  const test_1 = [...base];
  let start = performance.now();
  sortAlgorithm[method](test_1);
  let end = performance.now();

  console.log(`Sem pré-processamento com ${size} elementos: ${(end - start).toFixed(3)}ms | Comps: ${test.comparacoes} | Swaps: ${test.trocas}`);
  logResult(method, arrayType, size, false, `${(end - start).toFixed(3)}ms`, test.comparacoes, test.trocas);
  
  test.reset();

  const test_2 = [...base];
  start = performance.now();
  presortAlgorithm(test_2, method);
  end = performance.now();

  console.log(`Com pré-processamento com ${size} elementos: ${(end - start).toFixed(3)}ms | Comps: ${test.comparacoes} | Swaps: ${test.trocas}\n`);
  logResult(method, arrayType, size, true, `${(end - start).toFixed(3)}ms`, test.comparacoes, test.trocas);

  test.reset();
}