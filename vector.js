class Vector {
  constructor(x, y) {
    this.x = x;
    this.y = y;
  }

  set(x, y) {
    this.x = x;
    this.y = y;
    return this;
  }

  add(v) {
    this.x += v.x;
    this.y += v.y;
    return this;
  }

  scale(a) {
    this.x *= a;
    this.y *= a;
    return this;
  }

  rotate(r) {
    [this.x, this.y] = [
      this.x * Math.cos(r) - this.y * Math.sin(r),
      this.x * Math.sin(r) + this.y * Math.cos(r),
    ];
    return this;
  }

  dist(v) {
    return Math.sqrt(Math.pow(this.x - v.x, 2) + Math.pow(this.y - v.y, 2));
  }

  windowX() {
    return this.x + 300;
  }
  windowY() {
    return -this.y + 300;
  }
}
