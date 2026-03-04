import { bubbleSort } from "../sort/bubbleSort.ts";
import { insertionSort } from "../sort/insertionSort.ts";
import { mergeSort } from "../sort/mergeSort.ts";
import { quickSort } from "../sort/quickSort.ts";
import { selectionSort } from "../sort/selectionSort.ts";

type SortTypes = "BUBBLE" | "INSERTION" | "MERGE" | "QUICK" | "SELECTION";

const sortAlgoritm: Record<SortTypes, (array: number[]) => void> = {
  "BUBBLE": bubbleSort,
  "INSERTION": insertionSort,
  "MERGE": mergeSort,
  "QUICK": quickSort,
  "SELECTION": selectionSort
}
export { sortAlgoritm, type SortTypes }