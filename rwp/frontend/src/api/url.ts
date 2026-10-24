export class Url {
  baseUrl: string;
  create: string;
  getAll: string;
  update: string;

  constructor() {
    this.baseUrl = Url.getBaseUrl();
    this.create = `${this.baseUrl}api/v1/create`;
    this.getAll = `${this.baseUrl}api/v1/get/all`;
    this.update = `${this.baseUrl}api/v1/update`;
  }

  static getBaseUrl(): string {
    const { origin } = window.location;
    if (origin.includes("localhost:3000")) {
      return "http://0.0.0.0:8001/";
    }
    return `${origin}/`;
  }

  deleteUrl(name: string): string {
    return `${this.baseUrl}api/v1/delete/${name}`;
  }
}
