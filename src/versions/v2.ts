function presortAlgoritm(array: number[]): void {
  const last = array.length - 1;
  const mid = array.length >> 1;

  if (array[0] > array[last]) {
    const t = array[0];
    array[0] = array[last];
    array[last] = t;
  }

  for (let i = 1; i < mid; i++) {
    const j = last - i;

    if (array[i] > array[j]) {
      const t = array[i];
      array[i] = array[j];
      array[j] = t;
    }

    if (array[i] < array[i - 1]) {
      const t = array[i];
      array[i] = array[i - 1];
      array[i - 1] = t;
    }
    
    if (array[j] > array[j + 1]) {
      const t = array[j];
      array[j] = array[j + 1];
      array[j + 1] = t;
    }
  }
}