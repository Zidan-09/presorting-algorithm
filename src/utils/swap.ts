import { test } from "../entities/test.ts";

export function swap(idx1: number, idx2: number, array: number[]) {
  const temp = array[idx1];
  array[idx1] = array[idx2];
  array[idx2] = temp;
  test.trocas++;
}