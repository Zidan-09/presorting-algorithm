function presortAlgoritm(array: number[]): void {
  const last = array.length - 1;
  const mid = array.length >> 1;

  for (let i = 0; i < mid; i++) {

    const j = last - i;

    if (array[i] > array[j]) {
      const t = array[i];
      array[i] = array[j];
      array[j] = t;
    }
  }
}