class KeyState {
  constructor(ws) {
    this.states = {
      'ArrowLeft': false,
      'ArrowRight': false,
      'ArrowUp': false,
      ' ': false,
    };
    this.ws = ws;

    document.addEventListener('keydown', (e) => {
      if (e.repeat) return;

      this.put(e.key, true);
      this.sendStates();
    });

    document.addEventListener('keyup', (e) => {
      if (e.repeat) return;

      this.put(e.key, false);
      this.sendStates();
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
      || code === ' '
    ) {
      this.states[code] = state;
    }
  }

  sendStates() {
    if (this.ws.readyState === 1) {
      // Sends a message if readyState is OPEN
      this.ws.send(JSON.stringify({
        type: 'keystate',
        data: this.states,
      }));
    }
  }
}

const url = 'ws://localhost:8080';

const canvas = document.querySelector('#screen');
const context = canvas.getContext('2d');

drawBackground();

try {
  const ws = new WebSocket(url);
  const keyState = new KeyState(ws);

  ws.onmessage = (event) => {
    // When a message comes
    const message = JSON.parse(event.data);

    if (message.type === 'objects') {
      drawBackground();

      const objects = message.data;

      objects.forEach((object, _) => {
        if (object.type === 'ship') {
          renderShip(object.data);
        } else if (object.type === 'bullet') {
          renderBullet(object.data);
        }
      });
    }
  };
} catch (e) {
  // Ignore
}

function drawBackground() {
  const before = context.fillStyle;
  context.fillStyle = '#000000';
  context.fillRect(0, 0, 600, 600);
  context.fillStyle = before;
}

function rotatePoint(point, rad) {
  return {
    x: point.x * Math.cos(rad) - point.y * Math.sin(rad),
    y: point.x * Math.sin(rad) + point.y * Math.cos(rad),
  };
}

function renderShip(ship) {
  const before = context.strokeStyle;
  context.strokeStyle = ship.color;

  const framePoints = [
    { x:  15.0, y:   0.0 },
    { x: -15.0, y:  10.0 },
    { x:  -5.0, y:   0.0 },
    { x: -15.0, y: -10.0 },
  ];
  const flarePoints = [
    { x: -15.0, y:  7.0 },
    { x: -22.0, y:  7.0 },
    { x: -15.0, y: -7.0 },
    { x: -22.0, y: -7.0 },
  ];

  if (!ship.isAlive) {
    // Draw explosion
    for (let i = 0; i < 8; i++) {
      const innerPoint = rotatePoint({ x: 0.0, y:  5.0 }, Math.PI * i / 4);
      const outerPoint = rotatePoint({ x: 0.0, y:  10.0 }, Math.PI * i / 4);

      context.beginPath();
      context.moveTo(ship.x + innerPoint.x, ship.y + innerPoint.y);
      context.lineTo(ship.x + outerPoint.x, ship.y + outerPoint.y);
      context.stroke();
    }

    return;
  }

  // Draw the frame
  context.beginPath();
  framePoints.forEach((point, i) => {
    const rotatedFramePoint = rotatePoint(point, ship.rad);

    if (i === 0) {
      context.moveTo(ship.x + rotatedFramePoint.x, ship.y + rotatedFramePoint.y);
    } else {
      context.lineTo(ship.x + rotatedFramePoint.x, ship.y + rotatedFramePoint.y);
    }
  });
  context.closePath();
  context.stroke();

  context.beginPath();
  context.arc(ship.x, ship.y, 5, 0, 2 * Math.PI);
  context.stroke();

  // Draw flares if the ship is accelerating
  if (ship.isAccelerating) {
    flarePoints.forEach((point, i) => {
      const rotatedFlarePoint = rotatePoint(point, ship.rad);

      if (i % 2 === 0) {
        context.beginPath();
        context.moveTo(ship.x + rotatedFlarePoint.x, ship.y + rotatedFlarePoint.y);
      } else {
        context.lineTo(ship.x + rotatedFlarePoint.x, ship.y + rotatedFlarePoint.y);
        context.stroke();
      }
    });
  }

  context.strokeStyle = before;
}

function renderBullet(bullet) {
  const before = context.fillStyle;
  context.fillStyle = bullet.color;

  context.beginPath();
  context.arc(bullet.x, bullet.y, 2.5, 0, 2 * Math.PI);
  context.fill();

  context.fillStyle = before;
}
