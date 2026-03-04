export function generateRandomArray(size: number, min: number = 0, max: number = 1000): number[] {
  return Array.from({ length: size }, () =>
    Math.floor(Math.random() * (max - min + 1)) + min
  );
}