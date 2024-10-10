class KeyState {
  constructor() {
    this.states = {
      'ArrowLeft': false,
      'ArrowRight': false,
      'ArrowUp': false,
      'ArrowDown': false,
      ' ': false,
    };

    document.addEventListener("keydown", (e) => {
      this.put(e.key, true);
    });

    document.addEventListener("keyup", (e) => {
      this.put(e.key, false);
    });
  }

  get(code) {
    return this.states[code];
  }

  put(code, state) {
    if (
      code === 'ArrowLeft'
      || code === 'ArrowRight'
      || code === 'ArrowUp'
      || code === 'ArrowDown'
      || code === ' '
    ) {
      this.states[code] = state;
    }
  }
}

