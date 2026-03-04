import { testBubble } from "./tests/bubble.ts";
import { testInsertion } from "./tests/insertion.ts";
import { testMerge } from "./tests/merge.ts";
import { testQuick } from "./tests/quick.ts";
import { testSelection } from "./tests/selection.ts";

function main() {
  console.log("Iniciando testes de algoritmos de ordenação + método de pré-ordenação...\n\n");

  testBubble(false);
  testSelection(false);
  testInsertion(false);
  testMerge(false);
  testQuick(false);
}

main();