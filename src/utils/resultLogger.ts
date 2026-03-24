import fs from "fs";
import path from "path";

const FILE_PATH = path.resolve("results.csv");

function ensureFileExists() {
  if (!fs.existsSync(FILE_PATH)) {
    fs.writeFileSync(
      FILE_PATH,
      "algorithm,arrayType,size,withPresort,time,comparisons,swaps\n"
    );
  }
}

export function logResult(
  algorithm: string,
  arrayType: string,
  size: number,
  withPresort: boolean,
  time: string,
  comparisons: number,
  swaps: number
) {
  ensureFileExists();

  const line = `${algorithm},${arrayType},${size},${withPresort},${time},${comparisons},${swaps}\n`;

  fs.appendFileSync(FILE_PATH, line);
}