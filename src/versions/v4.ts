function presortAlgorithm(array: number[]): void {
  const last = array.length - 1;
  const mid = array.length >> 1;

  for (let i = 0; i < mid; i++) {
    const j = last - i;

    if (i + 1 < mid) {
      const ai = array[i];
      const ai1 = array[i + 1];
      if (ai > ai1) {
        array[i] = ai1;
        array[i + 1] = ai;
      }
    }

    if (j - 1 >= mid) {
      const aj = array[j];
      const aj1 = array[j - 1];
      if (aj < aj1) {
        array[j] = aj1;
        array[j - 1] = aj;
      }
    }

    const ai = array[i];
    const aj = array[j];
    if (ai > aj) {
      array[i] = aj;
      array[j] = ai;
    }
  }
}