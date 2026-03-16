import { bubbleSort } from "../sort/bubbleSort.js";
import { insertionSort } from "../sort/insertionSort.js";
import { mergeSort } from "../sort/mergeSort.js";
import { quickSort } from "../sort/quickSort.js";
import { selectionSort } from "../sort/selectionSort.js";

enum SortTypes {
  BUBBLE = "bubble",
  INSERTION = "insertion",
  MERGE = "merge",
  QUICK = "quick",
  SELECTION = "selection"
}

enum ArrayTypes {
  RANDOM = "random",
  INVERTED = "inverted",
  ZIGZAG = "zigzag",
  TURTLES = "turtles",
  DUPLICATES = "duplicates",
  ALMOSTSORTED = "almostsorted"
}

const sortAlgoritm: Record<SortTypes, (array: number[]) => void> = {
  "bubble": bubbleSort,
  "insertion": insertionSort,
  "merge": mergeSort,
  "quick": quickSort,
  "selection": selectionSort
}
export { sortAlgoritm, SortTypes, ArrayTypes }