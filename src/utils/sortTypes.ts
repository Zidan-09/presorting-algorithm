import { bubbleSort } from "../sort/bubbleSort.js";
import { insertionSort } from "../sort/insertionSort.js";
import { mergeSort } from "../sort/mergeSort.js";
import { quickSort } from "../sort/quickSort.js";
import { selectionSort } from "../sort/selectionSort.js";

type SortTypes = "BUBBLE" | "INSERTION" | "MERGE" | "QUICK" | "SELECTION";

const sortAlgoritm: Record<SortTypes, (array: number[]) => void> = {
  "BUBBLE": bubbleSort,
  "INSERTION": insertionSort,
  "MERGE": mergeSort,
  "QUICK": quickSort,
  "SELECTION": selectionSort
}
export { sortAlgoritm, type SortTypes }