export class Url {
	baseUrl: string;
	create: string;
	getAll: string;
	update: string;
	login: string;
	logout: string;

	constructor() {
		this.baseUrl = Url.getBaseUrl();
		this.create = `${this.baseUrl}api/v1/create`;
		this.getAll = `${this.baseUrl}api/v1/get`;
		this.update = `${this.baseUrl}api/v1/update`;
		this.login = `${this.baseUrl}api/v1/auth/login`;
		this.logout = `${this.baseUrl}api/v1/auth/logout`;
	}

	static getBaseUrl(): string {
		const { origin } = window.location;
		if (origin.includes("localhost:3000")) {
			return "http://localhost:8001/";
		}
		return `${origin}/`;
	}

	deleteUrl(id: number): string {
		return `${this.baseUrl}api/v1/delete/${id}`;
	}

	getByIdUrl(id: number): string {
		return `${this.baseUrl}api/v1/get/${id}`;
	}
}
