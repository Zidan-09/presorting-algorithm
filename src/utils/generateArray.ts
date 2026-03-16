import { ArrayTypes } from "./sortTypes.js";

export function generateTestArray(size: number, arrayType: ArrayTypes): number[] {
  switch (arrayType) {
    case ArrayTypes.INVERTED:
      return Array.from({ length: size }, (_, i) => size - i);
    case ArrayTypes.ZIGZAG:
      return Array.from({ length: size }, (_, i) => i % 2 === 0 ? i : size - i);
    case ArrayTypes.TURTLES:
      return Array.from({ length: size }, (_, i) => i < size / 2 ? i + size : i % 10);
    case ArrayTypes.DUPLICATES:
      return Array.from({ length: size }, () => Math.floor(Math.random() * 3));
    case ArrayTypes.ALMOSTSORTED:
      return (() => {
      const arr = Array.from({ length: size }, (_, i) => i);
      for (let k = 0; k < size / 100; k++) {
        const i = Math.floor(Math.random() * size);
        const j = Math.floor(Math.random() * size);
        [arr[i], arr[j]] = [arr[j], arr[i]];
      }
      return arr;
    })();

    default:
      return Array.from({ length: size }, () =>
        Math.floor(Math.random() * (1000000 - 1 + 1)) + 1
      );
  }
}