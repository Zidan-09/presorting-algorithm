import { finalTest } from "./tests/finalTest.ts";
import { testCustom } from "./tests/custom.ts";
import type { SortTypes } from "./utils/sortTypes.ts";

function main() {
  const method: SortTypes = "INSERTION";

  finalTest(method);
  testCustom(method);
}

main();