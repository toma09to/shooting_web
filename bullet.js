class Bullet {
  constructor(id, pos, rad, col) {
    this.id = id;
    this.pos = new Vector(pos.x, pos.y);
    this.rad = rad;
    this.speed = 5.0;
    this.bulletColor = col;
  }

  move() {
    const deltaPos = new Vector(this.speed, 0.0).rotate(this.rad);
    this.pos.add(deltaPos);
  }

  render(ctx) {
    ctx.fillStyle = this.bulletColor;
    ctx.beginPath();
    ctx.arc(this.pos.windowX(), this.pos.windowY(), 2.5, 0, 2 * Math.PI);
    ctx.fill();
  }

  isAlive() {
    return (this.pos.x >= -360
         && this.pos.x <= 360
         && this.pos.y >= -360
         && this.pos.y <= 360);
  }

  isHit(ship) {
    return (this.pos.dist(ship.pos) < 12.0);
  }

  data() {
    return JSON.stringify({
      type: 'Bullet',
      data: {
        id: this.id,
        pos: this.pos,
      },
    });
  }
}
