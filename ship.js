class Ship {
  constructor(id, posX, posY, rad, col) {
    this.id = id;
    this.pos = new Vector(posX, posY);
    this.rad = rad;
    this.speed = new Vector(0.0, 0.0);
    this.rotateSpeed = 0.07;
    this.accelFactor = 0.03;
    this.decelFactor = 0.005;
    this.isAccelerating = false;
    this.chargeTime = 500;
    this.lastFireTime = 0;
    this.isAlive = true;
    this.lastHitTime = 0;
    this.maxLives = 3;
    this.lives = 0;
    this.shipColor = col;
    this.isReady = false;
  }

  render(ctx) {
    ctx.strokeStyle = this.shipColor;
    const coord = new Vector(0.0, 0.0);

    const frame = [
      new Vector(15.0, 0.0),
      new Vector(-15.0, 10.0),
      new Vector(-5.0, 0.0),
      new Vector(-15.0, -10.0),
    ];
    const flare = [
      new Vector(-15.0, 7.0),
      new Vector(-22.0, 7.0),
      new Vector(-15.0, -7.0),
      new Vector(-22.0, -7.0),
    ];

    if (this.isAlive) {
      // Draw a frame
      ctx.beginPath();
      frame.forEach((point, i) => {
        coord.set(point.x, point.y).rotate(this.rad).add(this.pos);
        if (i === 0) {
          ctx.moveTo(coord.windowX(), coord.windowY());
        } else {
          ctx.lineTo(coord.windowX(), coord.windowY());
        }
      });
      ctx.closePath();
      ctx.stroke();
      
      ctx.beginPath();
      ctx.arc(this.pos.windowX(), this.pos.windowY(), 5, 0, 2 * Math.PI);
      ctx.stroke();

      // Draw flares
      if (this.isAccelerating) {
        flare.forEach((point, i) => {
          coord.set(point.x, point.y).rotate(this.rad).add(this.pos);
          if (i % 2 === 0) {
            ctx.beginPath();
            ctx.moveTo(coord.windowX(), coord.windowY());
          } else {
            ctx.lineTo(coord.windowX(), coord.windowY());
            ctx.stroke();
          }
        });
      }
    } else {
      // Draw explosion
      for (let i = 0; i < 8; i++) {
        ctx.beginPath();
        coord.set(0.0, 5.0).rotate(Math.PI / 4 * i).add(this.pos);
        ctx.moveTo(coord.windowX(), coord.windowY());
        coord.set(0.0, 10.0).rotate(Math.PI / 4 * i).add(this.pos);
        ctx.lineTo(coord.windowX(), coord.windowY());
        ctx.stroke();
      }
    }
  }

  entry() {
    this.lives = this.maxLives;
    this.isReady = false;
  }

  control(ks) {
    this.isAccelerating = ks.get('ArrowUp');
    if (this.isAlive) {
      const deltaSpeed = new Vector(0.0, 0.0);

      // Acceleration
      if (this.isAccelerating) {
        deltaSpeed.set(this.accelFactor, 0.0).rotate(this.rad);
        this.speed.add(deltaSpeed);
      }

      // Deceleration
      this.speed.scale(1 - this.decelFactor);

      // Rotate
      if (ks.get('ArrowLeft')) {
        this.rad += this.rotateSpeed;
      }
      if (ks.get('ArrowRight')) {
        this.rad -= this.rotateSpeed;
      }
    }

    // Move
    this.pos.add(this.speed);

    if (this.pos.x > 315) {
      this.pos.x -= 630;
    } else if (this.pos.x < -315) {
      this.pos.x += 630;
    }
    if (this.pos.y > 315) {
      this.pos.y -= 630;
    } else if (this.pos.y < -315) {
      this.pos.y += 630;
    }
  }

  move(x, y, rad, isAcc, isAlv, lives) {
    this.pos.set(x, y);
    this.rad = rad;
    this.isAccelerating = isAcc;
    this.isAlive = isAlv;
    this.lives = lives;
  }

  fire(ks) {
    const canFire = (this.isAlive && ks.get(' ') && Date.now() - this.lastFireTime > this.chargeTime);
    if (canFire) {
      this.lastFireTime = Date.now();
    }
    return canFire;
  }

  head() {
    const head = new Vector(15.0, 0.0);
    head.rotate(this.rad).add(this.pos);
    return head;
  }

  respawn(x, y, r) {
    if (!this.isAlive && Date.now() - this.lastHitTime > 1000 & this.lives > 0) {
      this.lives -= 1;

      this.pos.set(x, y);
      this.rad = r;
      this.speed.set(0.0, 0.0);
      this.isAlive = (this.lives > 0);
    }
  }

  data() {
    return JSON.stringify({
      type: "Ship",
      data: {
        id: this.id,
        pos: this.pos,
        rad: this.rad,
        isAccelerating: this.isAccelerating,
        isAlive: this.isAlive,
        lives: this.lives,
      },
    });
  }

  ready() {
    this.isReady = true;
  }
}
