import { finalTest } from "./tests/finalTest.js";
import { testCustom } from "./tests/custom.js";
import type { SortTypes } from "./utils/sortTypes.js";

function main() {
  const method: SortTypes = "BUBBLE";

  finalTest(method);
  testCustom(method);
}

main();