export function generateRandomArray(size: number, min: number = 0, max: number = 1000): number[] {
  return Array.from({ length: size }, () =>
    Math.floor(Math.random() * (max - min + 1)) + min
  );
}

export function generateTestArray(size: number) {
  return {
    inverted: Array.from({ length: size }, (_, i) => size - i),
    zigzag: Array.from({ length: size }, (_, i) => i % 2 === 0 ? i : size - i),
    turtles: Array.from({ length: size }, (_, i) => i < size / 2 ? i + size : i % 10),
    duplicates: Array.from({ length: size }, () => Math.floor(Math.random() * 3)),
    almostSorted: (() => {
      const arr = Array.from({ length: size }, (_, i) => i);
      for (let k = 0; k < size / 100; k++) {
        const i = Math.floor(Math.random() * size);
        const j = Math.floor(Math.random() * size);
        [arr[i], arr[j]] = [arr[j], arr[i]];
      }
      return arr;
    })()
  };
}