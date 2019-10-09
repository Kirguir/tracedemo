export class JasonError extends Error {
  constructor(message) {
    super(message);
    this.name = this.constructor.name;
  }
}

export class DetachedStateError extends JasonError {
  constructor(message) {
    super(message);
    this.name = this.constructor.name;
  }
}

export class MediaError extends JasonError {
  constructor(message) {
    super(message);
    this.name = this.constructor.name;
  }
}
