import { finalTest } from "./tests/finalTest.js";
import { ArrayTypes, SortTypes } from "./utils/sortTypes.js";

function main() {
  let method = process.argv[2];
  let arrayType = process.argv[3];
  let size = Number(process.argv[4]);

  if (!Object.values(SortTypes).includes(method as SortTypes)) {
    method = SortTypes.BUBBLE;
  }

  if (!Object.values(ArrayTypes).includes(arrayType as ArrayTypes)) {
    arrayType = ArrayTypes.RANDOM;
  }

  if (isNaN(size)) {
    size = 10000;
  }

  finalTest(method as SortTypes, arrayType as ArrayTypes, size);
}

main();